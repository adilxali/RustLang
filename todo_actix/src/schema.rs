// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        task -> Varchar,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
