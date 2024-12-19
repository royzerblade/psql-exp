#[derive(Clone, Eq, PartialEq)]
pub struct Spell {
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
