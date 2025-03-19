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
