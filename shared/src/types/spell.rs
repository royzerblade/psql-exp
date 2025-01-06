use serde_derive::{Deserialize, Serialize};
use structmap::ToMap;
use structmap_derive::ToMap;

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize, ToMap)]
pub struct QuerySpell {
    pub id: i32,
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

impl Default for QuerySpell {
    fn default() -> QuerySpell {
        QuerySpell {
            id: -1,
            description: "".to_string(),
            saving_throw_type: "".to_string(),
            damage_type: "".to_string(),
            damage: "".to_string(),
            shape: "".to_string(),
            area_affected: "".to_string(),
            range: "".to_string(),
            duration: "".to_string(),
            is_concentration: false,
        }
    }
}
