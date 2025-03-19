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

    forza_telemetry (id) {
        id -> Text,
        user_id -> Text,
        game_type -> ForzaType,
        date_time -> Timestamptz,
        is_race_on -> Bool,

        car_id -> Int4,
        car_class -> ForzaCarClass,
        car_performance_index -> Int4,
        drive_type -> ForzaDriveType,
        num_cylinders -> Int4,
        track_id -> Nullable<Int4>,

        engine_max_rpm -> Float,
        engine_idle_rpm -> Float,
        engine_current_rpm -> Float,

        acceleration_x -> Float,
        acceleration_y -> Float,
        acceleration_z -> Float,

        velocity_x -> Float,
        velocity_y -> Float,
        velocity_z -> Float,

        angular_velocity_x -> Float,
        angular_velocity_y -> Float,
        angular_velocity_z -> Float,

        yaw -> Float,
        pitch -> Float,
        roll -> Float,

        normalized_suspension_travel_front_left -> Float,
        normalized_suspension_travel_front_right -> Float,
        normalized_suspension_travel_rear_left -> Float,
        normalized_suspension_travel_rear_right -> Float,

        tire_slip_ratio_front_left -> Float,
        tire_slip_ratio_front_right -> Float,
        tire_slip_ratio_rear_left -> Float,
        tire_slip_ratio_rear_right -> Float,

        wheel_rotation_speed_front_left -> Float,
        wheel_rotation_speed_front_right -> Float,
        wheel_rotation_speed_rear_left -> Float,
        wheel_rotation_speed_rear_right -> Float,

        wheel_on_rumble_strip_front_left -> Bool,
        wheel_on_rumble_strip_front_right -> Bool,
        wheel_on_rumble_strip_rear_left -> Bool,
        wheel_on_rumble_strip_rear_right -> Bool,

        wheel_in_puddle_front_left -> Float,
        wheel_in_puddle_front_right -> Float,
        wheel_in_puddle_rear_left -> Float,
        wheel_in_puddle_rear_right -> Float,

        surface_rumble_front_left -> Float,
        surface_rumble_front_right -> Float,
        surface_rumble_rear_left -> Float,
        surface_rumble_rear_right -> Float,

        tire_slip_angle_front_left -> Float,
        tire_slip_angle_front_right -> Float,
        tire_slip_angle_rear_left -> Float,
        tire_slip_angle_rear_right -> Float,

        tire_combined_slip_front_left -> Float,
        tire_combined_slip_front_right -> Float,
        tire_combined_slip_rear_left -> Float,
        tire_combined_slip_rear_right -> Float,

        suspension_travel_meters_front_left -> Float,
        suspension_travel_meters_front_right -> Float,
        suspension_travel_meters_rear_left -> Float,
        suspension_travel_meters_rear_right -> Float,

        position_x -> Nullable<Float>,
        position_y -> Nullable<Float>,
        position_z -> Nullable<Float>,

        speed -> Nullable<Float>,
        power -> Nullable<Float>,
        torque -> Nullable<Float>,

        tire_temp_front_left -> Nullable<Float>,
        tire_temp_front_right -> Nullable<Float>,
        tire_temp_rear_left -> Nullable<Float>,
        tire_temp_rear_right -> Nullable<Float>,

        boost -> Nullable<Float>,
        fuel -> Nullable<Float>,
        distance_traveled -> Nullable<Float>,

        best_lap -> Nullable<Float>,
        last_lap -> Nullable<Float>,
        current_lap -> Nullable<Float>,
        current_race_time -> Nullable<Float>,
        lap_number -> Nullable<Int4>,
        position -> Nullable<Int4>,

        acceleration -> Nullable<Int4>,
        brake -> Nullable<Int4>,
        clutch -> Nullable<Int4>,
        handbrake -> Nullable<Int4>,
        gear -> Nullable<Int4>,
        steer -> Nullable<Int4>,

        normalized_driving_lane -> Nullable<Int4>,
        normalized_ai_brake_difference -> Nullable<Int4>,
        tire_wear_front_left -> Nullable<Float>,
        tire_wear_front_right -> Nullable<Float>,
        tire_wear_rear_left -> Nullable<Float>,
        tire_wear_rear_right -> Nullable<Float>,
    }
}

diesel::joinable!(forza_telemetry -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(users, forza_telemetry);
