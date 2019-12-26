table! {
    current_members_view (id) {
        id -> Uuid,
        real_name -> Varchar,
        playa_name -> Varchar,
        location -> Nullable<Varchar>,
        expected_arrival -> Timestamp,
        expected_departure -> Timestamp,
    }
}
