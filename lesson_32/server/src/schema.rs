// @generated automatically by Diesel CLI.

diesel::table! {
    device (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        state -> Jsonb,
        room_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    house (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    room (id) {
        id -> Int4,
        name -> Varchar,
        house_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(device -> room (room_id));
diesel::joinable!(room -> house (house_id));

diesel::allow_tables_to_appear_in_same_query!(device, house, room,);
