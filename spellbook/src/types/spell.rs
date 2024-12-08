use shapes::Shapes;
use stats::Stats;

mod shapes;
mod stats;

pub struct Spell {
    description: String,
    savingThrowType: Stats,
    damageType: String,
    damage: String,
    shape: Shapes,
    areaAffected: String,
    range: String,
    duration: String,
    isConcentration: bool,
}
