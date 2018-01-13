table! {
    geschenke (id) {
        id -> Int4,
        short_description -> Varchar,
        description -> Nullable<Text>,
        creator -> Nullable<Int4>,
        receiver -> Int4,
        gifter -> Nullable<Int4>,
        obtained_date -> Nullable<Timestamp>,
        gifted_date -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Nullable<Varchar>,
        salt -> Nullable<Varchar>,
        autologin -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    geschenke,
    users,
);
