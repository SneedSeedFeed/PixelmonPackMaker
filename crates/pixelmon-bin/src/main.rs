use std::{
    collections::HashSet,
    io::{Read, Write},
    ops::Deref,
    path::PathBuf,
    sync::Mutex,
};

pub mod expixel;
pub mod resource;
pub mod resource_pack_writer;
use anyhow::{Context, anyhow};
use clap::Parser;
use itertools::Itertools;
use pixelmon_types::{
    sound_registry::SoundRegistry,
    species_data::{Form, Sound, SpeciesData},
};

use serde::Serialize;
use zip::{ZipWriter, write::SimpleFileOptions};

use crate::{
    config::Config, expixel::get_sound_expixel, resource::get_sound_resource,
    resource_pack_writer::ResourcePackWriter,
};

#[derive(Parser)]
struct Args {
    config: PathBuf,
}

pub mod config;

fn is_pixelmon_filepath(s: &str) -> bool {
    let mut splits = s.split('/').rev();
    let Some(json_question_mark) = splits.next() else {
        return false;
    };
    let Some(is_species_question_mark) = splits.next() else {
        return false;
    };

    is_species_question_mark == "species"
        && json_question_mark.ends_with(".json")
        && !json_question_mark.contains("000_missingno")
}

fn is_pixelmon_sound_file(s: &str) -> bool {
    s.starts_with("assets/pixelmon/sounds/pixelmon/") && s.ends_with(".ogg")
}

#[derive(Debug, Serialize)]
struct PackCreationReport {
    changed_species_files: Vec<String>,
    added_sound_files: Vec<String>,
    replaced_sound_files: Vec<String>,
    unchanged_sound_files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let config: Config =
        serde_json::from_reader(&mut std::fs::File::open(args.config).unwrap()).unwrap();

    let zip_file = std::fs::File::open(&config.source).unwrap();
    let mut zip_reader = zip::ZipArchive::new(zip_file).unwrap();

    let existing_sound_files = zip_reader
        .file_names()
        .filter(|s| is_pixelmon_sound_file(s))
        .map(PathBuf::from)
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<HashSet<_>>();

    let files = zip_reader
        .file_names()
        .filter(|s| is_pixelmon_filepath(s))
        .map(String::from)
        .collect::<Vec<_>>();

    let mut species_data = Vec::new();
    for file_name in files {
        let mut buf = String::new();
        zip_reader
            .by_name(&file_name)
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        species_data.push((
            file_name,
            serde_json::from_str::<SpeciesData>(&buf).unwrap(),
            false,
        ));
    }

    let mut buf = String::new();
    zip_reader
        .by_name("assets/pixelmon/sounds.json")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let sound_registry = Mutex::new(SoundRegistry::default()); // Supposedly don't have to replace the whole sounds.json?

    let resource_pack_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!(
            "pixelmon_cry_replacer_resource_pack_{}.zip",
            config.version_number
        ))
        .unwrap();
    let mut resource_pack_zip = ZipWriter::new(resource_pack_file);

    resource_pack_zip
        .add_directory_from_path(
            "assets/pixelmon/sounds/pixelmon",
            SimpleFileOptions::default(),
        )
        .unwrap();

    let len = species_data.len();

    let resource_pack_zip = Mutex::new(ResourcePackWriter::new(resource_pack_zip));

    std::thread::scope(|s| {
        for chunk in species_data
            .iter_mut()
            .chunks(len / config.num_threads)
            .into_iter()
        {
            let chunk = chunk.collect::<Vec<_>>();
            s.spawn(|| {
                for (_, species, did_mutate) in chunk {
                    process_species(
                        species,
                        &sound_registry,
                        &resource_pack_zip,
                        did_mutate,
                        &config,
                    )
                    .unwrap();
                }
            });
        }
    });

    let sound_json = serde_json::to_string_pretty(sound_registry.lock().unwrap().deref()).unwrap();

    let (mut resource_pack_zip, mut added_sounds) =
        resource_pack_zip.into_inner().unwrap().into_inner();

    resource_pack_zip
        .start_file("assets/pixelmon/sounds.json", SimpleFileOptions::default())
        .unwrap();
    resource_pack_zip.write_all(sound_json.as_bytes()).unwrap();

    resource_pack_zip
        .start_file_from_path("pack.mcmeta", SimpleFileOptions::default())
        .unwrap();

    resource_pack_zip
        .write_all(config.resource_pack_mcmeta.get().as_bytes())
        .unwrap();

    resource_pack_zip
        .start_file_from_path("Credits.txt", SimpleFileOptions::default())
        .unwrap();
    resource_pack_zip
        .write_all(config.credits.as_bytes())
        .unwrap();

    for (src, dest) in config.deep_copy {
        if !existing_sound_files.contains(&format!("{dest}.ogg")) {
            print!("{dest} is not a sound file from the source, please review config");
            std::process::exit(1)
        }
        resource_pack_zip
            .deep_copy_file_from_path(
                format!("assets/pixelmon/sounds/pixelmon/{src}.ogg"),
                format!("assets/pixelmon/sounds/pixelmon/{dest}.ogg"),
            )
            .unwrap();
        added_sounds.insert(format!("{dest}.ogg"));
    }

    resource_pack_zip.finish().unwrap();

    let data_pack = std::fs::File::create(format!(
        "pixelmon_cry_replacer_data_pack_{}.zip",
        config.version_number
    ))
    .unwrap();
    let mut data_pack = ZipWriter::new(data_pack);

    data_pack
        .start_file("pack.mcmeta", SimpleFileOptions::default())
        .unwrap();
    data_pack
        .write_all(config.data_pack_mcmeta.get().as_bytes())
        .unwrap();

    data_pack
        .start_file_from_path("Credits.txt", SimpleFileOptions::default())
        .unwrap();
    data_pack.write_all(config.credits.as_bytes()).unwrap();

    let mut changed_species_files = Vec::<String>::new();
    for (path, species, did_mutate) in species_data {
        if did_mutate {
            let (idx, _) = path
                .char_indices()
                .rev()
                .find(|(_, char)| *char == '/')
                .unwrap();

            let file_name = String::from(&path[idx + 1..path.len()]);
            changed_species_files.push(file_name);

            data_pack
                .start_file(&path, SimpleFileOptions::default())
                .unwrap();
            data_pack
                .write_all(&serde_json::to_vec_pretty(&species).unwrap())
                .unwrap();
        }
    }

    data_pack.finish().unwrap();

    let mut added_sound_files = Vec::new();
    let mut replaced_sound_files = Vec::new();
    let mut unchanged_sound_files = Vec::new();

    for added_sound in &added_sounds {
        if existing_sound_files.contains(added_sound) {
            replaced_sound_files.push(added_sound.clone())
        } else {
            added_sound_files.push(added_sound.clone())
        }
    }

    for existing_sound in existing_sound_files {
        if !added_sounds.contains(&existing_sound) {
            unchanged_sound_files.push(existing_sound)
        }
    }

    changed_species_files.sort();
    added_sound_files.sort();
    replaced_sound_files.sort();
    unchanged_sound_files.sort();
    let pack_report = PackCreationReport {
        changed_species_files,
        added_sound_files,
        replaced_sound_files,
        unchanged_sound_files,
    };

    let mut report_file =
        std::fs::File::create(format!("pack_report_{}.json", config.version_number)).unwrap();

    serde_json::to_writer_pretty(&mut report_file, &pack_report).unwrap();
}

fn process_species(
    species: &mut SpeciesData,
    sound_registry: &Mutex<SoundRegistry>,
    resource_zip: &Mutex<ResourcePackWriter>,
    did_mutate: &mut bool,
    config: &Config,
) -> anyhow::Result<()> {
    let pokemon_name = species.name.to_lowercase();

    if config.dumb_insert.contains(&pokemon_name) {
        process_form_dumb(
            species,
            &pokemon_name,
            sound_registry,
            resource_zip,
            did_mutate,
        )
    } else {
        let to_skip = config.skip_form_names.get(&pokemon_name);
        for form in species.forms.iter_mut() {
            if config.skip_form_names_all.contains(&form.name) {
                continue;
            }

            if let Some(to_skip) = to_skip {
                match to_skip {
                    config::ConfigForm::All => continue,
                    config::ConfigForm::Form(to_skip) => {
                        if to_skip.contains(&form.name) {
                            continue;
                        }
                    }
                    config::ConfigForm::Except(dont_skip) => {
                        if !dont_skip.contains(&form.name) {
                            continue;
                        }
                    }
                }
            }

            let form_name = Some(form.name.as_str())
                .filter(|&form_name| !config.treat_as_base_all.contains(form_name))
                .filter(|&form_name| {
                    config
                        .treat_as_base
                        .get(&pokemon_name)
                        .is_none_or(|base_form| base_form != form_name)
                })
                .map(String::from); // form_name could be moved back into process_form to save this alloc 

            process_form(
                form,
                &pokemon_name,
                form_name.as_deref(),
                sound_registry,
                resource_zip,
                did_mutate,
            )?;
        }
        Ok(())
    }
}

fn process_form(
    form: &mut Form,
    pokemon_name: &str,
    form_name: Option<&str>,
    sound_registry: &Mutex<SoundRegistry>,
    resource_zip: &Mutex<ResourcePackWriter>,
    did_mutate: &mut bool,
) -> anyhow::Result<()> {
    let sound_file = match get_sound_expixel(pokemon_name, form_name)? {
        Some(f) => f,
        None => match get_sound_resource(pokemon_name, form_name)? {
            Some(f) => f,
            None => {
                return Err(anyhow!(
                    "Failed to get sound file for {} {form_name:?}",
                    &pokemon_name
                ));
            }
        },
    };

    let first_palette = form
        .gender_properties
        .as_mut()
        .and_then(|f| f.first_mut())
        .and_then(|props| props.palettes.first_mut())
        .context(format!(
            "No gender properties for {pokemon_name}-{}",
            form.name
        ))?;

    let sound_data = std::fs::read(&sound_file)?;
    let sound_id = sound_registry
        .lock()
        .unwrap()
        .register_mob_sound(pokemon_name, form_name);

    let removed_sounds = first_palette.sounds.replace(vec![Sound {
        sound_id: sound_id.clone(),
        range: 14,
    }]);

    if removed_sounds
        .map(|a| {
            !a.eq(&vec![Sound {
                sound_id: sound_id.clone(),
                range: 14,
            }])
        })
        .unwrap_or_default()
    {
        *did_mutate = true
    }

    let dest_path = form_name
        .map(|form_name| {
            format!(
                "assets/pixelmon/sounds/pixelmon/{}-{form_name}.ogg",
                pokemon_name
            )
        })
        .unwrap_or_else(|| format!("assets/pixelmon/sounds/pixelmon/{}.ogg", pokemon_name));

    {
        let mut lock = resource_zip.lock().unwrap();
        lock.write_sound_file(pokemon_name, form_name, &sound_data, dest_path)
            .unwrap();
    }
    Ok(())
}

fn process_form_dumb(
    species: &mut SpeciesData,
    pokemon_name: &str,
    sound_registry: &Mutex<SoundRegistry>,
    resource_zip: &Mutex<ResourcePackWriter>,
    did_mutate: &mut bool,
) -> anyhow::Result<()> {
    let sound_file = match get_sound_expixel(pokemon_name, None)? {
        Some(f) => f,
        None => match get_sound_resource(pokemon_name, None)? {
            Some(f) => f,
            None => return Err(anyhow!("Failed to get sound file for {}", pokemon_name)),
        },
    };

    let form = species.forms.first_mut().context("no forms")?;

    let first_palette = form
        .gender_properties
        .as_mut()
        .and_then(|f| f.first_mut())
        .and_then(|props| props.palettes.first_mut())
        .context(format!(
            "No gender properties for {pokemon_name}-{}",
            form.name
        ))?;

    let sound_data = std::fs::read(&sound_file)?;
    let sound_id = sound_registry
        .lock()
        .unwrap()
        .register_mob_sound(pokemon_name, None);

    let removed_sounds = first_palette.sounds.replace(vec![Sound {
        sound_id: sound_id.clone(),
        range: 14,
    }]);

    if removed_sounds
        .map(|a| {
            !a.eq(&vec![Sound {
                sound_id: sound_id.clone(),
                range: 14,
            }])
        })
        .unwrap_or_default()
    {
        *did_mutate = true
    }

    let dest_path = format!("assets/pixelmon/sounds/pixelmon/{}.ogg", pokemon_name);

    {
        let mut lock = resource_zip.lock().unwrap();
        lock.write_sound_file(pokemon_name, None, &sound_data, dest_path)?;
    };

    Ok(())
}
