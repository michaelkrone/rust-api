table! {
    applications (id) {
        id -> Int4,
        name -> Varchar,
        notes -> Nullable<Varchar>,
        typ -> Varchar,
        ts -> Timestamp,
    }
}

table! {
    locations (id) {
        id -> Int4,
        name -> Varchar,
        notes -> Nullable<Varchar>,
        lat -> Nullable<Float8>,
        long -> Nullable<Float8>,
    }
}

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
        nid -> Uuid,
        mac -> Varchar,
        name -> Varchar,
        notes -> Nullable<Varchar>,
        locations_id -> Nullable<Int4>,
        applications_ids -> Nullable<Array<Int4>>,
        ts -> Timestamp,
    }
}

table! {
    nodes_status (id) {
        id -> Int4,
        nodes_id -> Int4,
        nid -> Uuid,
        ip -> Nullable<Varchar>,
        status -> Nullable<Int4>,
        ts -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    applications,
    locations,
    measurements,
    nodes,
    nodes_status,
);
