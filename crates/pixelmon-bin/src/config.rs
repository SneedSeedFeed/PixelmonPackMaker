use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    num::NonZero,
    path::PathBuf,
};

use serde::{
    Deserialize,
    de::{Unexpected, Visitor},
};
use serde_json::value::RawValue;

// Realistically I think a lot of these config options are worthless. Skip form names basically just exists for slowbro/king, treat as base is just there for ogerpon (but can be done with dumb_insert or treat_as_base_all)

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    /// Source .jar
    pub source: PathBuf,
    pub version_number: String,
    /// Number of threads to have work on the species data at a time
    pub num_threads: NonZero<usize>,
    /// JSON to place in the resource pack's pack.mcmeta
    pub resource_pack_mcmeta: Box<RawValue>,
    /// Text to place in the credits.txt
    pub credits: String,
    /// JSON to place in the data pack's pack.mcmeta
    pub data_pack_mcmeta: Box<RawValue>,
    /// Pokemon to "play dumb" for, grabbing the first sound file we can get, shoving it in the first palette and calling it a day.
    pub dumb_insert: HashSet<String>,
    /// Form names to skip completely for ALL pokemon
    pub skip_form_names_all: HashSet<String>,
    /// Form names to skip completely for specific pokemon (excluding those marked as a base form)
    pub skip_form_names: HashMap<String, ConfigForm>,
    /// Form names to treat as 'base' for all pokemon (i.e no form)
    pub treat_as_base_all: HashSet<String>,
    /// Form names to treat as 'base' for specific Pokemon
    pub treat_as_base: HashMap<String, String>,
    /// Files to deep copy at the last step, to overwrite files in pixelmon that don't meet the {pokemon}-{form} naming convention
    pub deep_copy: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigForm {
    All,
    Form(HashSet<String>),
    Except(HashSet<String>),
}

impl<'de> Deserialize<'de> for ConfigForm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = ConfigForm;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "either \"all\" or a list of form names")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut forms = HashSet::new();
                while let Some(form) = seq.next_element()? {
                    forms.insert(form);
                }
                Ok(ConfigForm::Form(forms))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == "all" {
                    Ok(ConfigForm::All)
                } else {
                    Err(serde::de::Error::invalid_value(Unexpected::Str(v), &"all"))
                }
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut except = None;
                while let Some(key) = map.next_key::<Cow<'_, str>>()? {
                    match key.as_ref() {
                        "except" => except = Some(map.next_value()?),
                        other => return Err(serde::de::Error::unknown_field(other, &["except"])),
                    }
                }

                let except = except.ok_or_else(|| serde::de::Error::missing_field("except"))?;
                Ok(ConfigForm::Except(except))
            }
        }

        deserializer.deserialize_any(Vis)
    }
}
