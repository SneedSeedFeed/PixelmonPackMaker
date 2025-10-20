use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundRegistry(pub HashMap<String, SoundInfo>); // BTreeMap to maintain order from the source

impl SoundRegistry {
    pub fn register_mob_sound(&mut self, mob_name: &str, form_name: Option<&str>) -> String {
        let (k, file_name) = if let Some(form_name) = form_name {
            (
                format!("pixelmon.mob.{mob_name}.{form_name}"),
                format!("pixelmon:pixelmon/{mob_name}-{form_name}"),
            )
        } else {
            (
                format!("pixelmon.mob.{mob_name}"),
                format!("pixelmon:pixelmon/{mob_name}"),
            )
        };

        let v = SoundInfo {
            sounds: SoundList::Stream(vec![SoundListItem {
                name: file_name,
                stream: false,
            }]),
            subtitle: None, // None of my cries are subtitled
        };

        self.0.insert(k.clone(), v);
        format!("pixelmon:{k}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoundInfo {
    pub sounds: SoundList,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum SoundList {
    Raw(Vec<String>),
    Stream(Vec<SoundListItem>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoundListItem {
    pub name: String,
    pub stream: bool,
}
