table! {
    geschenke (id) {
        id -> Int4,
        short_description -> Nullable<Text>,
        description -> Nullable<Text>,
        creator -> Nullable<Int4>,
        receiver -> Int4,
        gifter -> Nullable<Int4>,
        obtained_date -> Nullable<Date>,
        gifted_date -> Nullable<Date>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    geschenke,
    users,
);
