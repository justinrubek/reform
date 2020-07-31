table! {
    entries (id) {
        id -> Int4,
        schema_id -> Int4,
        data -> Jsonb,
    }
}

table! {
    forms (id) {
        id -> Int4,
        name -> Text,
        fields -> Jsonb,
        mappings -> Jsonb,
    }
}

table! {
    schemas (id) {
        id -> Int4,
        data -> Jsonb,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        hash -> Text,
    }
}

joinable!(entries -> schemas (schema_id));

allow_tables_to_appear_in_same_query!(
    entries,
    forms,
    schemas,
    users,
);
