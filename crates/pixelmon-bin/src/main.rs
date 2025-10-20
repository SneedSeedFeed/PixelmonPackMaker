use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    ops::Deref,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

pub mod expixel;
pub mod resource;
use anyhow::{Context, anyhow};
use clap::Parser;
use itertools::Itertools;
use pixelmon_types::{
    sound_registry::SoundRegistry,
    species_data::{Sound, SpeciesData},
};
use zip::{ZipWriter, write::SimpleFileOptions};

use crate::{expixel::get_sound_expixel, resource::get_sound_resource};

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

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

fn main() {
    let args = Args::parse();
    let zip_file = std::fs::File::open(args.input).unwrap();
    let mut zip_reader = zip::ZipArchive::new(zip_file).unwrap();

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
        ));
    }

    let mut buf = String::new();
    zip_reader
        .by_name("assets/pixelmon/sounds.json")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let sound_registry = Mutex::new(serde_json::from_str::<SoundRegistry>(&buf).unwrap());

    let resource_pack_zip = Mutex::new(ZipWriter::new(
        std::fs::File::create("output_resource_pack.zip").unwrap(),
    ));

    resource_pack_zip
        .lock()
        .unwrap()
        .add_directory_from_path(
            "assets/pixelmon/sounds/pixelmon",
            SimpleFileOptions::default(),
        )
        .unwrap();

    const NUM_THREADS: usize = 16;
    let len = species_data.len();

    std::thread::scope(|s| {
        for chunk in species_data
            .iter_mut()
            .chunks(len / NUM_THREADS)
            .into_iter()
        {
            let chunk = chunk.collect::<Vec<_>>();
            s.spawn(|| {
                for (_, species) in chunk {
                    process_species(species, &sound_registry, &resource_pack_zip).unwrap();
                }
            });
        }
    });

    let sound_json = serde_json::to_string_pretty(sound_registry.lock().unwrap().deref()).unwrap();

    {
        let mut lock = resource_pack_zip.lock().unwrap();
        lock.start_file("assets/pixelmon/sounds.json", SimpleFileOptions::default())
            .unwrap();
        lock.write_all(sound_json.as_bytes()).unwrap();

        lock.start_file_from_path("pack.mcmeta", SimpleFileOptions::default())
            .unwrap();

        lock.write_all(
            br#"{
  "pack": {
    "pack_format": 34,
    "description": "Shitty ahh pixelmon sound replacer"
  }
}"#,
        )
        .unwrap();

        lock.start_file_from_path("Credits.txt", SimpleFileOptions::default())
            .unwrap();
        lock.write_all(b"Game sound effects/cries made/compiled by RegularPerson\nSome sounds made/compiled by Mysticus, Random Talking Bush and MeruZena all @ https://sounds.spriters-resource.com/")
            .unwrap()
    }

    resource_pack_zip.into_inner().unwrap().finish().unwrap();

    let data_pack = std::fs::File::create("output_data_pack.zip").unwrap();
    let mut data_pack = ZipWriter::new(data_pack);

    data_pack
        .start_file("pack.mcmeta", SimpleFileOptions::default())
        .unwrap();
    data_pack
        .write_all(
            br#"{
  "pack": {
    "pack_format": 48,
    "description": "Shitty ahh pixelmon sound replacer datapack, updates species.json to have the correct sound mappings"
  }
}"#,
        )
        .unwrap();

    for (path, species) in species_data {
        data_pack
            .start_file(&path, SimpleFileOptions::default())
            .unwrap();
        data_pack
            .write_all(&serde_json::to_vec(&species).unwrap())
            .unwrap();
    }

    data_pack.finish().unwrap();
}

// Separating this away from the big ass species data helps the IDE. Those serde macros are brutal

// Either due to not being relevant or lacking data, skip these form names
static SKIP_FORM_NAMES: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from_iter(["gmax", "alolan", "gmaxrs", "gmaxss"]));

// These are the 'base' form thus are form_name = None
static TREAT_AS_NO_FORM_NAME: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from_iter(["teal", "base"]));

// For the same reasons, don't do anything 'per form' just throw any new sfxs on the first form in their vec and call it a day
static JUST_USE_BASE: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from_iter([
        "aegislash",
        "arceus",
        "basculin",
        "bidoof",
        "burmy",
        "castform",
        "cherrim",
        "cramorant",
        "darmanitan",
        "decidueye",
        "deerling",
        "deoxys",
        "dialga",
        "dragonite",
        "dubwool",
        "dudunsparce",
        "eiscue",
        "electrode",
        "enamorus",
        "eternatus",
        "furfrou",
        "gastrodon",
        "genesect",
        "giratina",
        "goodra",
        "greninja",
        "keldeo",
        "landorus",
        "lunala",
        "lunatone",
        "magearna",
        "mareep",
        "marshadow",
        "meloetta",
        "mimikyu",
        "minior",
        "morpeko",
        "ogerpon",
        "palkia",
        "pichu",
        "pikachu",
        "poltchageist",
        "polteageist",
        "ponyta",
        "rapidash",
        "rotom",
        "samurott",
        "sawsbuck",
        "shellos",
        "silvally",
        "sinistcha",
        "sinistea",
        "sneasel",
        "solgaleo",
        "squawkabilly",
        "stunfisk",
        "tauros",
        "terapagos",
        "thundurus",
        "tornadus",
        "typhlosion",
        "unown",
        "voltorb",
        "wooloo",
        "wooper",
        "wormadam",
        "xerneas",
        "yamask",
        "zoroark",
        "zorua",
    ])
});

fn process_species(
    species: &mut SpeciesData,
    sound_registry: &Mutex<SoundRegistry>,
    resource_zip: &Mutex<zip::ZipWriter<File>>,
) -> anyhow::Result<()> {
    let name = species.name.to_lowercase();

    if JUST_USE_BASE.contains(name.as_str()) {
        let sound_file = match get_sound_expixel(&name, None)? {
            Some(f) => f,
            None => match get_sound_resource(&name, None)? {
                Some(f) => f,
                None => return Err(anyhow!("Failed to get sound file for {}", &name)),
            },
        };

        let form = species.forms.first_mut().context("no forms")?;

        let first_palette = form
            .gender_properties
            .as_mut()
            .and_then(|f| f.first_mut())
            .and_then(|props| props.palettes.first_mut())
            .context(format!("No gender properties for {name}-{}", form.name))?;

        let sound_data = std::fs::read(&sound_file)?;
        let sound_id = sound_registry
            .lock()
            .unwrap()
            .register_mob_sound(&name, None);

        first_palette.sounds.replace(vec![Sound {
            sound_id,
            range: 14,
        }]);

        let dest_path = format!("assets/pixelmon/sounds/pixelmon/{}.ogg", name);

        {
            let mut lock = resource_zip.lock().unwrap();
            lock.start_file_from_path(&dest_path, SimpleFileOptions::default())?;
            lock.write_all(&sound_data).unwrap();
        }
    } else {
        for form in species.forms.iter_mut() {
            if SKIP_FORM_NAMES.contains(form.name.as_str()) {
                continue;
            }

            let form_name =
                Some(form.name.as_str()).filter(|name| !TREAT_AS_NO_FORM_NAME.contains(name));

            let sound_file = match get_sound_expixel(&name, form_name)? {
                Some(f) => f,
                None => match get_sound_resource(&name, form_name)? {
                    Some(f) => f,
                    None => {
                        return Err(anyhow!(
                            "Failed to get sound file for {} {form_name:?}",
                            &name
                        ));
                    }
                },
            };

            let first_palette = form
                .gender_properties
                .as_mut()
                .and_then(|f| f.first_mut())
                .and_then(|props| props.palettes.first_mut())
                .context(format!("No gender properties for {name}-{}", form.name))?;

            let sound_data = std::fs::read(&sound_file)?;
            let sound_id = sound_registry
                .lock()
                .unwrap()
                .register_mob_sound(&name, form_name);

            first_palette.sounds.replace(vec![Sound {
                sound_id,
                range: 14,
            }]);

            let dest_path = form_name
                .map(|form_name| {
                    format!("assets/pixelmon/sounds/pixelmon/{}-{form_name}.ogg", name)
                })
                .unwrap_or_else(|| format!("assets/pixelmon/sounds/pixelmon/{}.ogg", name));

            {
                let mut lock = resource_zip.lock().unwrap();
                lock.start_file_from_path(&dest_path, SimpleFileOptions::default())?;
                lock.write_all(&sound_data).unwrap();
            }
        }
    }

    Ok(())
}
