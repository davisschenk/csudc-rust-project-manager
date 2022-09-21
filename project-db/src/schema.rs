table! {
    aircrafts (id) {
        id -> Integer,
        name -> Text,
        registration -> Nullable<Text>,
    }
}

table! {
    datasets (id) {
        id -> Integer,
        flight_number -> Integer,
        flight_id -> Integer,
        start -> Timestamp,
        end -> Timestamp,
    }
}

table! {
    flights (id) {
        id -> Integer,
        name -> Text,
        datasets -> Integer,
        location_id -> Integer,
        aircraft_id -> Integer,
        pilot_id -> Integer,
        start -> Timestamp,
        end -> Timestamp,
        speed_ms -> Integer,
        height_m -> Integer,
    }
}

table! {
    locations (id) {
        id -> Integer,
        name -> Text,
        kml -> Text,
        project_id -> Integer,
    }
}

table! {
    pilots (id) {
        id -> Integer,
        name -> Text,
        registration -> Nullable<Text>,
    }
}

table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        directory -> Text,
        start -> Date,
        end -> Nullable<Date>,
    }
}

allow_tables_to_appear_in_same_query!(
    aircrafts,
    datasets,
    flights,
    locations,
    pilots,
    projects,
);
