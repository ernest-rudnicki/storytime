// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 24]
        name -> Varchar,
        email -> Text,
    }
}
