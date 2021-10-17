table! {
    users (id) {
        id -> Uuid,
        profile_picture -> Varchar,
        username -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        role -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
