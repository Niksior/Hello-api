table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
        players -> Int2,
        icon -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}
