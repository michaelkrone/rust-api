table! {
    temperatures (id) {
        id -> Int8,
        value -> Float8,
        unit -> Bpchar,
        node -> Int4,
        // ts -> Nullable<Timestamptz>,
    }
}
