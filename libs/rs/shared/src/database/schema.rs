diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use crate::database::models::forza::export::{ForzaType, ForzaCarClass, ForzaDriveType};

    forza_telemetry (user_id, game_type, date_time) {
        id -> Text,
        user_id -> Text,
        game_type -> ForzaType,
        date_time -> Timestamptz,
        is_race_on -> Bool,

        drive_type -> ForzaDriveType,
        car_class -> ForzaCarClass,
        car_id -> Int4,
        car_performance_index -> Int4,
        num_cylinders -> Int4,

        engine_max_rpm -> Float,
        engine_idle_rpm -> Float,
        engine_current_rpm -> Float,

        track_id -> Nullable<Int4>,

        speed -> Nullable<Float>,
        power -> Nullable<Float>,
        torque -> Nullable<Float>,
        boost -> Nullable<Float>,
        fuel -> Nullable<Float>,
        distance_traveled -> Nullable<Float>,

        best_lap -> Nullable<Float>,
        last_lap -> Nullable<Float>,
        current_lap -> Nullable<Float>,
        current_race_time -> Nullable<Float>,
        lap_number -> Nullable<Int4>,
        race_position -> Nullable<Int4>,

        acceleration -> Nullable<Int4>,
        brake -> Nullable<Int4>,
        clutch -> Nullable<Int4>,
        handbrake -> Nullable<Int4>,
        gear -> Nullable<Int4>,
        steer -> Nullable<Int4>,

        normalized_driving_lane -> Nullable<Int4>,
        normalized_ai_brake_difference -> Nullable<Int4>,

        accelerations -> Jsonb,
        velocities -> Jsonb,
        angular_velocities ->Jsonb,
        orientations -> Jsonb,
        normalized_suspension_travels -> Jsonb,
        tire_slips_ratios -> Jsonb,
        wheel_rotation_speeds -> Jsonb,
        wheel_on_rumble_strips -> Jsonb,
        wheel_in_puddles -> Jsonb,
        surface_rumbles -> Jsonb,
        tire_slip_angles -> Jsonb,
        tire_combined_slips -> Jsonb,
        suspension_travel_meters -> Jsonb,
        positions -> Nullable<Jsonb>,
        tire_temperatures -> Nullable<Jsonb>,
        tire_wears -> Nullable<Jsonb>,
    }
}

diesel::joinable!(forza_telemetry -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(users, forza_telemetry);

diesel::table! {
  use diesel::sql_types::*;
  use crate::database::models::f1::export::F1Type;
  use crate::packets::f1::headers::export::F1PacketId;
  use crate::packets::f1::event::export::EventCode;
  use crate::packets::f1::enums::export::MfdPanelIndex;

  f1_telemetry (session_id, user_id, game_type, date_time) {
    session_id -> Int8,
    user_id -> Text,
    game_type -> F1Type,
    date_time -> Timestamptz,
    packet_type -> F1PacketId,
    session_time -> Float,
    player_car_index -> Int4,
    secondary_player_car_index -> Int4,
    num_active_cars -> Int4,
    metadata -> Jsonb,
    time_trial_pb_car_index -> Nullable<Int4>,
    time_trial_rival_car_index -> Nullable<Int4>,
    event_code -> Nullable<EventCode>,
    event -> Nullable<Jsonb>,
    motions -> Nullable<Array<Jsonb>>,
    motion_extended -> Nullable<Jsonb>,
    session -> Nullable<Jsonb>,
    laps -> Nullable<Array<Jsonb>>,
    participants -> Nullable<Array<Jsonb>>,
    next_front_wing_value -> Nullable<Float>,
    car_setups -> Nullable<Array<Jsonb>>,
    mfd_panel_index -> Nullable<MfdPanelIndex>,
    mfd_panel_index_secondary_player -> Nullable<MfdPanelIndex>,
    suggested_gear -> Nullable<SmallInt>,
    car_telemetry -> Nullable<Array<Jsonb>>,
    car_status -> Nullable<Array<Jsonb>>,
    final_classification -> Nullable<Array<Jsonb>>,
    lobby -> Nullable<Array<Jsonb>>,
    car_damage -> Nullable<Array<Jsonb>>,
    session_history -> Nullable<Jsonb>,
    tyre_sets -> Nullable<Jsonb>,
    time_trial -> Nullable<Jsonb>,
  }
}

diesel::joinable!(f1_telemetry -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(users, f1_telemetry);
