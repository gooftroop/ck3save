use super::MetadataOwned;
use crate::flavor::reencode_float;
use jomini::common::Date as Ck3Date;
use jomini::JominiDeserialize;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, JominiDeserialize)]
pub struct Gamestate {
    pub meta_data: MetadataOwned,
    pub living: HashMap<u64, LivingCharacter>,
}

#[derive(Debug, JominiDeserialize)]
pub struct LivingCharacter {
    pub alive_data: Option<AliveData>,
    pub birth: Option<f32>,
    pub dna: Option<String>,
    pub dynasty_house: Option<f32>,
    pub faith: Option<f32>,
    pub family_data: Option<FamilyData>,
    pub first_name: Option<String>,
    pub immortal: Option<f32>,
    pub landed_data: Option<LandedData>,
    pub nickname: Option<String>,
    pub playable_data: Option<PlayableData>,
    pub portrait_override: Option<PortraitOverride>,
    pub skill: Option<f32>,
    pub traits: Option<f64>,
    pub variables: Option<Variables>,
    pub weight: Option<Weight>,
}

#[derive(Debug, JominiDeserialize)]
pub struct AliveData {
    pub activity: Option<f32>,
    pub fertility: Option<f32>,
    pub focus: Option<Focus>,
    pub health: Option<f32>,
    #[jomini(default, deserialize_with = "deserialize_eu4_float")]
    pub gold: Option<f64>,
    pub heir: Option<Vec<f32>>,
    pub kills: Option<Vec<f32>>,
    pub income: Option<f32>,
    pub inventory: Option<Inventory>,
    pub languages: Option<Vec<String>>,
    pub lifestyle_xp: Option<LifestyleXp>,
    pub location: Option<f32>,
    #[jomini(alias = "modifier", duplicated)]
    pub modifier: Vec<Modifier>,
    pub owned_activities: Option<Vec<f32>>,
    pub piety: Option<Piety>,
    pub prestige: Option<Prestige>,
    pub schemes: Option<Vec<f32>>,
    pub stories: Option<Vec<f32>>,
    pub wars: Option<Vec<f32>>,
}

#[derive(Debug, JominiDeserialize)]
pub struct FamilyData {
    pub child: Option<Vec<f64>>,
    #[jomini(alias = "concubine", duplicated)]
    pub concubine: Vec<f64>,
    #[jomini(alias = "former_spouses", duplicated)]
    pub former_spouse: Vec<f64>,
    pub primary_spouse: Option<f64>,
    #[jomini(alias = "spouse", duplicated)]
    pub spouse: Vec<f64>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Focus {
    #[jomini(alias = "type")]
    pub _type: Option<String>,
    pub date: Option<Ck3Date>,
    pub changes: Option<f32>,
    pub progress: Option<f32>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Inventory {
    pub owner: Option<f64>,
    pub equipped: Option<Equipped>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Equipped {
    pub wall_big_1: Option<f64>,
    pub wall_big_2: Option<f64>,
    pub wall_big_3: Option<f64>,
    pub pedestal_1: Option<f64>,
}

#[derive(Debug, JominiDeserialize)]
pub struct LandedData {
    pub domain: Option<Vec<f64>>,
    pub vassal_contracts: Option<Vec<f64>>,
    pub unit: Option<Vec<f64>>,
    pub war: Option<Vec<f64>>,
    pub last_war_finish_date: Option<Ck3Date>,
    pub became_ruler_date: Option<Ck3Date>,
    pub laws: Option<Vec<String>>,
    pub current_strength: Option<f64>,
    pub strength: Option<f64>,
    pub strength_without_hires: Option<f64>,
    pub levy: Option<f64>,
    pub balance: Option<f64>,
    pub succession: Option<Vec<f64>>,
    pub domain_limit: Option<f32>,
    pub vassal_limit: Option<f32>,
    pub vassals_towards_limit: Option<f32>,
    pub government: Option<String>,
    pub realm_capital: Option<f64>,
    pub council: Option<Vec<f64>>,
    pub diplo_centers: Option<Vec<f64>>,
    pub decision_cooldowns: Option<DecisionCooldowns>,
    pub interaction_cooldowns: Option<InteractionCooldowns>,
    pub royal_court: Option<RoyalCourt>,
    pub court_positions: Option<Vec<f64>>,
}

#[derive(Debug, JominiDeserialize)]
pub struct DecisionCooldowns {
    pub commission_artifact_decision: Option<Ck3Date>,
    pub invite_knights_decision: Option<Ck3Date>,
    pub invite_claimants_decision: Option<Ck3Date>,
    pub diligent_development_focus_decision: Option<Ck3Date>,
    pub meditate_in_seclusion_decision: Option<Ck3Date>,
    pub host_feast_decision: Option<Ck3Date>,
    pub start_hunt_decision: Option<Ck3Date>,
    pub hold_court_decision: Option<Ck3Date>,
    pub ao_adoption_decision: Option<Ck3Date>,
    pub dag_cdo_buy_slave_decision: Option<Ck3Date>,
    pub dag_cdo_host_orgy_decision: Option<Ck3Date>,
    pub dag_cdo_visit_royal_brothel_decision: Option<Ck3Date>,
    pub ie_host_orgy_decision: Option<Ck3Date>,
    pub qs_invite_holy_man: Option<Ck3Date>,
    pub qs_invite_debutante: Option<Ck3Date>,
    pub qs_invite_noble: Option<Ck3Date>,
    pub qs_invite_poet: Option<Ck3Date>,
    pub invite_novice: Option<Ck3Date>,
    pub adroit_visit_church_brothel_decision: Option<Ck3Date>,
}

#[derive(Debug, JominiDeserialize)]
pub struct InteractionCooldowns {
    pub carn_sex_interaction: Option<Ck3Date>,
    pub carnal_court_sex_interaction: Option<Ck3Date>,
    pub start_rapta_maritus: Option<Ck3Date>,
    pub regula_covert_conversion_interaction: Option<Ck3Date>,
    pub adroit_visit_priestess_interaction: Option<Ck3Date>,
}

#[derive(Debug, JominiDeserialize)]
pub struct RoyalCourt {
    pub court_grandeur: Option<CourtGrandeur>,
    pub court_amenitie: Option<CourtAmenities>,
    pub court_type: Option<CourtType>,
    pub language: Option<String>,
    pub language_adoption_date: Option<Ck3Date>,
}

#[derive(Debug, JominiDeserialize)]
pub struct CourtGrandeur {
    pub base: Option<f32>,
    pub current: Option<f32>,
    pub expected: Option<f32>,
}

#[derive(Debug, JominiDeserialize)]
pub struct CourtAmenities {
    pub court_fashion: Option<String>,
    pub court_food_qualit: Option<String>,
    pub court_lodging_standards: Option<String>,
    pub court_servants: Option<String>,
    pub cooldown: Option<CourtAmenitiesCooldown>,
}

#[derive(Debug, JominiDeserialize)]
pub struct CourtAmenitiesCooldown {
    pub court_fashion: Option<Ck3Date>,
    pub court_food_quality: Option<Ck3Date>,
    pub court_lodging_standards: Option<Ck3Date>,
    pub court_servants: Option<Ck3Date>,
}

#[derive(Debug, JominiDeserialize)]
pub struct CourtType {
    pub court_type: Option<String>,
    pub last_court_type_switch_date: Option<Ck3Date>,
}

#[derive(Debug, JominiDeserialize)]
pub struct LifestyleXp {
    pub diplomacy_lifestyle: Option<f64>,
    pub martial_lifestyle: Option<f64>,
    pub stewardship_lifestyle: Option<f64>,
    pub intrigue_lifestyle: Option<f64>,
    pub learning_lifestyle: Option<f64>,
    pub arcane_lifestyle: Option<f64>,
    pub pure_divine_lifestyle: Option<f64>,
    pub fallen_divine_lifestyle: Option<f64>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Modifier {
    pub modifier: Option<String>,
    pub expiration_date: Option<Ck3Date>,
}

#[derive(Debug, JominiDeserialize)]
pub struct PlayableData {
    pub knights: Option<Vec<f64>>,
    pub was_player: Option<bool>,
    pub last_court_event_added_date: Option<Ck3Date>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Piety {
    pub currency: Option<f64>,
    pub accumulated: Option<f64>,
}

#[derive(Debug, JominiDeserialize)]
pub struct PortraitOverride {
    pub portrait_modifier_overrides: Option<PortraitModifierOverrides>,
}

#[derive(Debug, JominiDeserialize)]
pub struct PortraitModifierOverrides {
    pub custom_beards: Option<String>,
    pub custom_hair: Option<String>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Prestige {
    pub currentcy: Option<f64>,
    pub accumulated: Option<f64>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Variables {
    pub data: Option<Vec<VariableData>>,
    pub list: Option<Vec<VariablesListData>>,
}

#[derive(Debug, JominiDeserialize)]
pub struct VariableData {
    pub flag: Option<String>,
    pub tick: Option<f32>,
    pub data: Option<VariableDataMap>,
}

#[derive(Debug, JominiDeserialize)]
pub struct VariableDataMap {
    #[jomini(alias = "type")]
    pub _type: Option<String>,
    pub identity: Option<f64>,
}

#[derive(Debug, JominiDeserialize)]
pub struct VariablesListData {
    pub name: Option<String>,
    #[jomini(alias = "item", duplicated)]
    pub item: Vec<VariableListItem>,
}

#[derive(Debug, JominiDeserialize)]
pub struct VariableListItem {
    #[jomini(alias = "type")]
    pub _type: Option<String>,
    pub flag: Option<String>,
    pub identity: Option<f64>,
}

#[derive(Debug, JominiDeserialize)]
pub struct Weight {
    pub current: Option<f32>,
    pub target: Option<f32>,
}

pub(crate) fn deserialize_eu4_float<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let val: Option<f64> = Option::deserialize(deserializer)?;
    val.map(reencode_float).map(Ok).transpose()
}
