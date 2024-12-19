use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::spells)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
