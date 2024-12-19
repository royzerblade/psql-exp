use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct QuerySpell {
    pub description: String,
    pub saving_throw_type: String,
    pub damage_type: String,
    pub damage: String,
    pub shape: String,
    pub area_affected: String,
    pub range: String,
    pub duration: String,
    pub is_concentration: bool,
}
