use crate::types::shapes::Shapes;
use crate::types::stats::Stats;

#[derive(Clone, Eq, PartialEq)]
pub struct Spell {
    pub description: String,
    pub saving_throw_type: Stats,
    pub damage_type: String,
    pub damage: String,
    pub shape: Shapes,
    pub area_affected: String,
    pub range: String,
    pub duration: String,
    pub is_concentration: bool,
}
