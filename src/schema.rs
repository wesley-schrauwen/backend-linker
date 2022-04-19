table! {
    links (id) {
        id -> Uuid,
        name -> Text,
        target_url -> Text,
        shortened_url -> Text,
        created_at -> Timestamptz,
        deleted_at -> Nullable<Timestamp>,
        version -> Int4,
    }
}
