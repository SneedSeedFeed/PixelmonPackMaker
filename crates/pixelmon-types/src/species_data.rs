use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpeciesData {
    pub name: String,
    pub dex: u32,
    pub default_forms: Vec<String>,
    pub forms: Vec<Form>,
    pub generation: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Form {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_group: Option<ExperienceGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moves: Option<Moves>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abilities: Option<Abilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement: Option<Movement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggression: Option<Aggression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battle_stats: Option<Stats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn: Option<Spawn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_genders: Option<Vec<Gender>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender_properties: Option<Vec<GenderProperties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egg_groups: Option<Vec<EggGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Types>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_evolutions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_base_form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mega_items: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub megas: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gigantamax: Option<Gigantamax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egg_cycles: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catch_rate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub male_percentage: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evolutions: Option<Vec<Evolution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_yields: Option<EvYields>,
    #[serde(rename = "growth_data", skip_serializing_if = "Option::is_none")]
    pub growth_data: Option<GrowthData>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExperienceGroup {
    Slow,
    MediumSlow,
    MediumFast,
    Fast,
    Erratic,
    Fluctuating,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Dimensions {
    pub height: Decimal,
    pub width: Decimal,
    pub length: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_height: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_height: Option<Decimal>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Moves {
    pub level_up_moves: Vec<LevelUpMove>,
    pub tutor_moves: Vec<String>,
    pub egg_moves: Vec<String>,
    pub tm_moves_9: Vec<String>,
    pub tm_moves_8: Vec<String>,
    pub tr_moves: Vec<String>,
    pub hm_moves: Vec<String>,
    pub transfer_moves: Vec<String>,
    pub tm_moves_7: Vec<String>,
    pub tm_moves_6: Vec<String>,
    pub tm_moves_5: Vec<String>,
    pub tm_moves_4: Vec<String>,
    pub tm_moves_3: Vec<String>,
    pub tm_moves_2: Vec<String>,
    pub tm_moves_1: Vec<String>,
    pub tm_moves: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LevelUpMove {
    pub level: u32,
    pub attacks: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Abilities {
    pub abilities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_abilities: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Movement {
    pub rideable: bool,
    pub can_fly: bool,
    pub can_surf: bool,
    pub can_ride_shoulder: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riding_offsets: Option<RidingOffsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_parameters: Option<FlyingParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounted_flying_parameters: Option<MountedFlyingParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swimming_parameters: Option<SwimmingParameters>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SwimmingParameters {
    pub depth_range_start: i32,
    pub depth_range_end: u32,
    pub swim_speed: u32,
    pub decay_rate: Decimal,
    pub refresh_rate: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chance_to_stop_on_block: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks_to_stop_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stop_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stop_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stop_cooldown_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stop_cooldown_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_rotate_while_stopped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_sink: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlyingParameters {
    pub fly_height_min: u32,
    pub fly_height_max: u32,
    pub fly_speed_modifier: Decimal,
    pub fly_refresh_rate_y: u32,
    #[serde(rename = "flyRefreshRateXZ")]
    pub fly_refresh_rate_xz: u32,
    pub fly_refresh_rate_speed: u32,
    pub flight_time_min: u32,
    pub flight_time_max: u32,
    pub flap_rate: u32,
    pub landing_materials: LandingMaterials,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MountedFlyingParameters {
    pub r#type: String,
    pub upper_angle_limit: u32,
    pub lower_angle_limit: i32,
    pub max_fly_speed: Decimal,
    pub deceleration_rate: Decimal,
    pub hover_deceleration_rate: Decimal,
    pub acceleration_rate: Decimal,
    pub strafe_acceleration_rate: Decimal,
    pub strafe_roll_conversion: u32,
    pub turn_rate: Decimal,
    pub pitch_rate: Decimal,
    pub stays_horizontal_flying: bool,
    pub gravity_drop_per_tick: Decimal,
    pub continuous_forward_motion: bool,
    pub continuous_forward_motion_ticks: u32,
    pub flying_stamina_charges: u32,
    pub hover_ticks: u32,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LandingMaterials {
    LeavesAndGrass,
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RidingOffsets {
    pub standing: RidingOffsetXYZ,
    pub moving: RidingOffsetXYZ,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RidingOffsetXYZ {
    x: Decimal,
    y: Decimal,
    z: Decimal,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Aggression {
    pub timid: u32,
    pub passive: i32,
    pub aggressive: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Stats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub special_attack: u32,
    pub special_defense: u32,
    pub speed: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvYields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_attack: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_defense: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GrowthData {
    pub mean: Decimal,
    pub standard_deviation: Decimal,
    pub min_render_scale: Decimal,
    pub max_render_scale: Decimal,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Spawn {
    pub base_exp: u32,
    pub base_friendship: u32,
    pub spawn_level: u32,
    pub spawn_level_range: i32,
    pub spawn_locations: Vec<SpawnLocation>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpawnLocation {
    Land,
    Air,
    AirPersistent,
    Underground,
    Water,
}

// Most controversial enum I will ever write
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    All,
    Male,
    Female,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GenderProperties {
    pub gender: Gender,
    pub palettes: Vec<Palette>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Palette {
    pub name: String, // Could make a special case for 'none' but ehhhh
    pub sprite: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sounds: Option<Vec<Sound>>,
    pub models: Vec<Model>,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<Particle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sound {
    pub sound_id: String,
    pub range: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub model_predicate: ModelPredicate,
    pub models: SeqOrMap,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)] // Fucking walking wake
pub enum SeqOrMap {
    Seq(Vec<ModelInternal>), // Most common so put this first
    Map(BTreeMap<String, Vec<ModelInternal>>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelPredicate {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_delay_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flying_or_swimming: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelInternal {
    pub texture: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_transparency: Option<Decimal>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_angle: Option<Vec<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_map: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_increment: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<ModelAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movement_threshold: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offsets: Option<Vec<Decimal>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelAttachment {
    pub attachment_point: String,
    pub model: ModelAttachmentInternal,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelAttachmentInternal {
    pub model: String,
    pub texture: String,
    pub offsets: Vec<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<Animation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_angle: Option<Vec<Decimal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Decimal>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Animation {
    pub r#type: String,
    pub animation: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Particle {
    pub probability: Decimal,
    pub options: ParticleOptions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParticleOptions {
    pub r#type: String,
    pub diameter: Decimal,
    pub lifetime: u32,
    pub tint: Rgba,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EggGroup {
    Monster,
    Grass,
    Dragon,
    WaterOne,
    Bug,
    Flying,
    Field,
    Fairy,
    Undiscovered,
    HumanLike,
    WaterThree,
    Mineral,
    Amorphous,
    Ditto,
    WaterTwo,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Types {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Gigantamax {
    pub can_have_factor: bool,
    pub can_gigantamax: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#move: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Evolution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<EvolutionCondition>>,
    pub evo_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moves: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<EvolutionItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anticonditions: Option<Vec<AntiCondition>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AntiCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<String>,
    pub evo_condition_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvolutionItem {
    #[serde(rename = "itemID")]
    pub item_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(
    tag = "evoConditionType",
    rename_all = "camelCase",
    deny_unknown_fields
)]
pub enum EvolutionCondition {
    Time {
        time: EvolutionTime,
    },
    #[serde(rename_all = "camelCase")]
    Party {
        #[serde(skip_serializing_if = "Option::is_none")]
        with_pokemon: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with_types: Option<Vec<Types>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with_forms: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with_palettes: Option<Vec<String>>,
    },
    Friendship {
        friendship: u32,
    },
    MoveUses {
        r#move: String,
        uses: u32,
    },
    HeldItem {
        item: EvolutionItem,
    },
    Biome {
        biomes: Vec<String>,
    },
    Critical {
        critical: u32,
    },
    #[serde(rename_all = "camelCase")]
    Move {
        r#move: Option<String>,
        attack_name: String,
    },
    Status {
        r#type: String,
    },
    #[serde(rename_all = "camelCase")]
    HasPalette {
        #[serde(skip_serializing_if = "Option::is_none")]
        possible_palettes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with_palettes: Option<Vec<String>>,
    },
    MoveType {
        r#type: Types,
    },
    #[serde(rename_all = "camelCase")]
    EvolutionRock {
        evolution_rock: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_range_squared: Option<u32>,
    },
    Chance {
        chance: Decimal,
    },
    StatRatio {
        stat1: StatEvo,
        stat2: StatEvo,
        ratio: Decimal,
    },
    Gender {
        genders: Vec<Gender>,
    },
    Recoil {
        recoil: String,
    },
    HealthAbsence {
        health: String,
    },
    Shiny {
        shiny: bool,
    },
    #[serde(rename_all = "camelCase")]
    HighAltitude {
        min_altitude: Decimal,
    },
    Weather {
        weather: String,
    },
    Nuggets {
        nuggets: u32,
    },
    Nature {
        natures: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    EvolutionScroll {
        evolution_scroll: EvolutionScroll,
        max_range_squared: u32,
    },
    #[serde(rename_all = "camelCase")]
    BlocksWalkedOutsideBall {
        blocks_to_walk: u32,
    },
    InsideBattle,
    GimmighoulCoins {
        amount: u32,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvolutionScroll {
    Darkness,
    Waters,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EvolutionTime {
    Day,
    Night,
    Dawn,
    Dusk,
    Afternoon,
    Morning,
    Midnight,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StatEvo {
    Attack,
    Defense,
}
