// @generated automatically by Diesel CLI.

diesel::table! {
    spells (id) {
        id -> Int4,
        description -> Text,
        saving_throw_type -> Text,
        damage_type -> Text,
        damage -> Text,
        shape -> Text,
        area_affected -> Text,
        range -> Text,
        duration -> Text,
        is_concentration -> Bool,
    }
}
