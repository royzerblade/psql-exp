// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "shapes_enum"))]
    pub struct ShapesEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "stats_enum"))]
    pub struct StatsEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatsEnum;
    use super::sql_types::ShapesEnum;

    spells (id) {
        id -> Int4,
        description -> Text,
        saving_throw_type -> StatsEnum,
        damage_type -> Text,
        damage -> Text,
        shape -> ShapesEnum,
        area_affected -> Text,
        range -> Text,
        duration -> Text,
        is_concentration -> Bool,
    }
}
