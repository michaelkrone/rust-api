table! {
    measurements (id) {
        id -> Int8,
        val -> Float8,
        typ -> Varchar,
        node -> Int4,
        ts -> Timestamp,
    }
}

table! {
    nodes (id) {
        id -> Int4,
        mac -> Varchar,
        ip -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
        status -> Nullable<Int4>,
        ts -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(measurements, nodes,);
