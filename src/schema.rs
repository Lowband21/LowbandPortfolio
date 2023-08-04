// @generated automatically by Diesel CLI.

diesel::table! {
    bio (id) {
        id -> Int4,
        #[max_length = 1000]
        #[sql_name = "bio"]
        bio_content -> Varchar,
    }
}

diesel::table! {
    project (id) {
        id -> Int4,
        #[max_length = 80]
        name -> Varchar,
        #[max_length = 240]
        description -> Varchar,
    }
}

diesel::table! {
    skill (id) {
        id -> Int4,
        #[max_length = 80]
        name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bio,
    project,
    skill,
);
