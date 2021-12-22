table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        reward -> Numeric,
        current_state -> Int4,
        compliance_reward -> Int4,
        infection -> Int4,
        next_event_time -> Int4,
    }
}

table! {
    levels (id) {
        id -> Int4,
        money -> Int4,
        score -> Int4,
        sections -> Nullable<Varchar>,
        compliance -> Int4,
        map_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    section_stats (id) {
        id -> Int4,
        simulation_fields -> Numeric,
        no_alive -> Int4,
        no_dead -> Int4,
        no_recovering -> Int4,
        healthcare_level -> Text,
        travel_restrictions -> Int4,
        event_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(section_stats -> events (event_id));

allow_tables_to_appear_in_same_query!(
    events,
    levels,
    section_stats,
);
