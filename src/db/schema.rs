table! {
    email_invitations (id) {
        id -> Uuid,
        email -> Varchar,
        invited_by -> Nullable<Uuid>,
        token -> Varchar,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        last_sent_at -> Nullable<Timestamptz>,
    }
}

table! {
    emergency_contacts (id) {
        id -> Uuid,
        member_id -> Uuid,
        name -> Varchar,
        phone_number -> Nullable<Varchar>,
        first_contact -> Bool,
    }
}

table! {
    members (id) {
        id -> Uuid,
        real_name -> Varchar,
        playa_name -> Nullable<Varchar>,
        email -> Varchar,
        phone -> Varchar,
        location -> Nullable<Varchar>,
        years_burned -> Int4,
        known_allergies -> Nullable<Array<Text>>,
        known_medications -> Nullable<Array<Text>>,
        dietary_restrictions -> Nullable<Array<Text>>,
        invited_by -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    signups (id) {
        id -> Uuid,
        member_id -> Uuid,
        year_id -> Uuid,
        attendance_probability -> Int4,
        expected_arrival -> Timestamp,
        expected_departure -> Timestamp,
        sleeping_arrangement -> Varchar,
        willing_to_early_arrival -> Nullable<Bool>,
        willing_to_post_burn -> Nullable<Bool>,
        read_essential_mindshark -> Bool,
        read_project_descriptions -> Bool,
        will_pay_dues -> Bool,
        will_perform_duties -> Bool,
        will_tear_down -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    ticket_information (id) {
        id -> Uuid,
        member_id -> Uuid,
        year_id -> Uuid,
        ticket_source_id -> Nullable<Uuid>,
        ticket_status -> Varchar,
        extra_tickets -> Int4,
        vehicle_pass -> Bool,
        extra_vehicle_passes -> Int4,
        pool_participant -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    url_invitations (id) {
        id -> Uuid,
        invited_by -> Nullable<Uuid>,
        token -> Varchar,
        remaining_uses -> Int4,
        expires_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        last_sent_at -> Nullable<Timestamptz>,
    }
}

table! {
    year_configuration (id) {
        id -> Uuid,
        year -> Int4,
        maximum_attendees -> Int4,
        dues_amount -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(email_invitations -> members (invited_by));
joinable!(emergency_contacts -> members (member_id));
joinable!(signups -> members (member_id));
joinable!(signups -> year_configuration (year_id));
joinable!(ticket_information -> members (member_id));
joinable!(ticket_information -> year_configuration (year_id));
joinable!(url_invitations -> members (invited_by));

allow_tables_to_appear_in_same_query!(
    email_invitations,
    emergency_contacts,
    members,
    signups,
    ticket_information,
    url_invitations,
    year_configuration,
);
