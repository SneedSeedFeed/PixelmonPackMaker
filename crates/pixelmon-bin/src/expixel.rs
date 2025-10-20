use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

#[allow(clippy::type_complexity)]
const EXPIXEL_MANUAL_MATCHES: &[((&str, Option<&str>), Option<&str>)] = &[
    (("hooh", None), Some("ho-oh")),                 // hyphen
    (("persian", None), Some("persain")),            // spelling
    (("milotic", None), Some("milotic1")),           // erroneous number
    (("whismur", None), Some("whismur1")),           // erroneous number
    (("mimejr", None), Some("mime_jr")),             // underscore issue
    (("brutebonnet", None), Some("brute_bonnet")),   // underscore issue
    (("hakamo-o", None), Some("hakamoo")), // has a duplicate but the misspelled one actually sounds better lol
    (("porygon-z", None), Some("porygonz")), // missing hyphen
    (("irontreads", None), Some("ironthreads")), // spelling
    (("cryogonal", None), Some("cryoganal")), // spelling
    (("indeedee", Some("male")), Some("indeedeem")), // Form formatting
    (("urshifu", Some("singlestrike")), None), // Has wrong file type
    (("hoopa", Some("confined")), Some("hoopa")), // Base is confined
    (("shaymin", Some("land")), Some("shaymin")), // Base is land
    (("zygarde", Some("complete")), Some("zygarde-100")),
    (("zygarde", Some("fifty_percent")), Some("zygarde-50")),
    (("zygarde", Some("ten_percent")), Some("zygarde-10")),
    (("calyrex", Some("icerider")), Some("calyrex-ice_rider")),
    (
        ("calyrex", Some("shadowrider")),
        Some("calyrex-shadow_rider"),
    ),
];

#[allow(clippy::type_complexity)]
static MANUAL_MATCH: LazyLock<HashMap<(&'static str, Option<&'static str>), Option<&'static str>>> =
    LazyLock::new(|| HashMap::from_iter(EXPIXEL_MANUAL_MATCHES.iter().copied()));

pub fn get_sound_expixel(pokemon: &str, form: Option<&str>) -> anyhow::Result<Option<PathBuf>> {
    // Just in case base or teal form get manually passed in
    let form = form.filter(|&some| !(some == "base" || some == "teal"));

    if let Some(file_name) = MANUAL_MATCH.get(&(pokemon, form)) {
        return Ok(file_name.map(|name| format!("expixel-sounds/{name}.ogg").into()));
    }

    let form = form.map(|v| {
        if v == "galarian" {
            "galar"
        } else if v == "hisuian" {
            "hisui"
        } else {
            v
        }
    });

    let pattern = if let Some(form) = &form {
        format!("expixel-sounds/{pokemon}-{form}*.ogg") // Sometimes a pokemon is hisui(an) or whatever so we want some fuzzy matching. Inelegant but fuck you, me.
    } else {
        format!("expixel-sounds/{pokemon}.ogg")
    };

    // Todo, just read the dir once lol
    let file = glob::glob(&pattern)?.next();

    match file {
        Some(Ok(o)) => Ok(Some(o)),
        Some(Err(e)) => Err(e.into()),
        None => Ok(None),
    }
}
