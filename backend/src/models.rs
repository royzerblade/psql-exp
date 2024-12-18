use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::spell)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QuerySpell(shared::types::spell::Spell);
