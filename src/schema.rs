// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 24]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 24]
        password -> Varchar,
        created_at -> Timestamp,
    }
}
