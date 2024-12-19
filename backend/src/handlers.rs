use crate::connect::establish_connection;
use crate::models::QuerySpell;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SpellId {
    entry_id: i32,
}

pub async fn read_spells() -> Result<HttpResponse> {
    use crate::schema::spells::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();

    let results = spells
        .select(QuerySpell::as_select())
        .load(connection)
        .expect("Error loading posts");

    Ok(HttpResponse::Ok().json(results))
}

pub async fn delete_spell(spell_id: web::Path<SpellId>) -> Result<HttpResponse> {
    use crate::schema::spells::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();

    diesel::delete(spells.filter(id.eq(spell_id.entry_id)))
        .execute(connection)
        .expect(&format!("Unable to find spell {:?}", spell_id.entry_id));

    Ok(HttpResponse::Ok().json("Deleted successfully"))
}

pub async fn create_spell(spell: web::Json<QuerySpell>) -> Result<HttpResponse> {
    use crate::schema::spells::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();

    diesel::insert_into(spells)
        .values(&spell.into_inner())
        .execute(connection)
        .expect("Error inserting new spell");

    Ok(HttpResponse::Ok().json("data inserted into the database"))
}

pub async fn update_spell(
    spell_id: web::Path<SpellId>,
    spell_update: web::Json<QuerySpell>,
) -> Result<HttpResponse> {
    use crate::schema::spells::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();

    let updated_spell = diesel::update(spells.filter(id.eq(spell_id.entry_id)))
        .set(&spell_update.into_inner())
        .execute(connection)
        .expect("Failed to update spell");
    Ok(HttpResponse::Ok().json(updated_spell))
}
