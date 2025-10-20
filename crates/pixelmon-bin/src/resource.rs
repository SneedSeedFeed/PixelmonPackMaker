use std::{
    borrow::Cow,
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::LazyLock,
};
use strsim::generic_jaro_winkler;

use anyhow::{Context, anyhow};

#[allow(clippy::type_complexity)]
const RESOURCE_WAV_MANUAL_MATCHES: &[((&str, Option<&str>), Option<&str>)] = &[
    (("pikachu", None), Some("025 - Pikachu (01)")),
    (
        ("ursalunabloodmoon", None),
        Some("901B - Ursaluna (Bloodmoon)"),
    ),
    // Koraidon/Miraidon - only keep the main forms
    (("koraidon", None), Some("1007AB - Koraidon (Apex Build)")),
    (
        ("miraidon", None),
        Some("1008UM - Miraidon (Ultimate Mode)"),
    ),
    (
        ("zacian", None),
        Some("888H - Zacian (Hero of Many Battles)"),
    ), // Base form
    (
        ("zacian", Some("crowned")),
        Some("888C - Zacian (Crowned Sword)"),
    ),
    (
        ("zamazenta", None),
        Some("889H - Zamazenta (Hero of Many Battles)"),
    ), // Base form
    (
        ("zamazenta", Some("crowned")),
        Some("889C - Zamazenta (Crowned Shield)"),
    ),
    // Missing, need to pull from pokemon go or home or something
    (("meltan", None), None),
    (("melmetal", None), None),
    (("floette", Some("az")), Some("670E - Floette (Eternal)")),
    (
        ("urshifu", Some("rapidstrike")),
        Some("892RS - Urshifu (Rapid Strike)"),
    ),
    (
        ("urshifu", Some("singlestrike")),
        Some("892SS - Urshifu (Single Strike)"),
    ),
];

#[allow(clippy::type_complexity)]
static WAV_MANUAL_MATCH: LazyLock<
    HashMap<(&'static str, Option<&'static str>), Option<&'static str>>,
> = LazyLock::new(|| HashMap::from_iter(RESOURCE_WAV_MANUAL_MATCHES.iter().copied()));

// this shit could probably use some more optimisations in future if it becomes my main source of sounds
pub fn get_sound_resource(pokemon: &str, form: Option<&str>) -> anyhow::Result<Option<PathBuf>> {
    // Just in case base or teal form get manually passed in
    let form = form.filter(|&some| !(some == "base" || some == "teal"));

    let expected_file_name = form
        .map(|form| format!("{pokemon}-{form}"))
        .unwrap_or_else(|| pokemon.to_string());

    let cached_ogg: PathBuf = format!("resource-sounds-converted/{expected_file_name}.ogg").into();

    if cached_ogg.is_file() {
        return Ok(Some(cached_ogg));
    }

    let target = match WAV_MANUAL_MATCH.get(&(pokemon, form)) {
        Some(m) => match m.map(PathBuf::from) {
            Some(m) => {
                let m: PathBuf = format!("resource-sounds/{}.wav", m.to_string_lossy()).into();
                if !m.is_file() {
                    return Err(anyhow!("File {} from manual match not found", m.display()));
                }
                m
            }
            None => return Ok(None),
        },
        None => {
            let mut file_names = Vec::new();

            for file in PathBuf::from("resource-sounds").read_dir()? {
                let file = file?;

                let path = file.path();

                if path.extension().context("no extension")? != "wav" {
                    continue;
                }

                let Some(name_from) = format_resource_name(&path)? else {
                    continue;
                };

                // I can't satisfy the &IntoIter requirement bullshit without collectiong to a vec
                let similarity = generic_jaro_winkler(
                    &name_from.chars().collect::<Vec<_>>(),
                    &expected_file_name.chars().collect::<Vec<_>>(),
                );

                file_names.push((similarity, path, name_from));
            }

            file_names.sort_by(|(score_a, _, _), (score_b, _, _)| score_b.total_cmp(score_a));

            let (top_score, path, _) = file_names.first().cloned().unwrap();

            if top_score < 0.8 {
                return Ok(None);
            } else {
                path
            }
        }
    };

    let converted_path = convert(&target, pokemon, form)?;

    Ok(Some(converted_path))
}

fn convert(wav_path: &Path, pokemon: &str, form: Option<&str>) -> anyhow::Result<PathBuf> {
    let ogg_path: PathBuf = form
        .map(|form| format!("resource-sounds-converted/{pokemon}-{form}.ogg"))
        .unwrap_or_else(|| format!("resource-sounds-converted/{pokemon}.ogg"))
        .into();

    // belt and braces
    if ogg_path.is_file() {
        return Ok(ogg_path);
    }

    let output = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(wav_path)
        .arg(&ogg_path)
        .output()?;

    if !output.status.success() {
        Err(anyhow!("Error when converting {}", wav_path.display()))
    } else {
        Ok(ogg_path)
    }
}

// do what we can to get the .wav file name in-line with what we want so we can
fn format_resource_name(path: &Path) -> anyhow::Result<Option<String>> {
    let file_stem = path
        .file_stem()
        .context("No filename")
        .map(OsStr::to_string_lossy)
        .map(Cow::into_owned)?;

    let len = file_stem.len();
    let first_hyphen = file_stem.find('-').context("Malformed name")?;

    let file_stem = file_stem[first_hyphen + 1..len].to_lowercase();

    let Some(first_bracket) = file_stem.find('(') else {
        return Ok(Some(file_stem.trim().replace(' ', "")));
    };

    let (name_part, form_part) = file_stem.split_at(first_bracket - 1);

    let form_part = form_part
        .trim()
        .trim_matches([')', '('])
        .replace("form", "") // helps cover tatsugiri and palafin
        .trim()
        .replace(' ', "");
    let name_part = name_part.trim().replace(' ', "");

    if form_part.chars().all(char::is_numeric) {
        return Ok(None); // Just for skipping Pikachu (_) lol
    }

    Ok(Some(format!("{name_part}-{form_part}")))
}
