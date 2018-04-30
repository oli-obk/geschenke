table! {
    presents (id) {
        id -> Int4,
        short_description -> Varchar,
        description -> Nullable<Text>,
        creator -> Nullable<Int4>,
        recipient -> Int4,
        gifter -> Nullable<Int4>,
        reserved_date -> Nullable<Timestamp>,
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

table! {
    friends (id) {
        id -> Int4,
        friend -> Int4,
    }
}

allow_tables_to_appear_in_same_query!{
    presents,
    users,
    friends,
}
