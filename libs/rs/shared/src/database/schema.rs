diesel::table! {
    use diesel::sql_types::*;
    use crate::database::models::forza::export::{ForzaType, ForzaCarClass, ForzaDriveType};

    forza_data (id) {
        id -> Text,
        game_type -> ForzaType,
        date_time -> Timestamptz,
        is_race_on -> Bool,

        car_id -> Int4,
        car_class -> ForzaCarClass,
        car_performance_index -> Int4,
        drive_type -> ForzaDriveType,
        num_cylinders -> Int4,
        track_id -> Nullable<Int4>,

        engine_max_rpm -> Numeric,
        engine_idle_rpm -> Numeric,
        engine_current_rpm -> Numeric,

        acceleration_x -> Numeric,
        acceleration_y -> Numeric,
        acceleration_z -> Numeric,

        velocity_x -> Numeric,
        velocity_y -> Numeric,
        velocity_z -> Numeric,

        angular_velocity_x -> Numeric,
        angular_velocity_y -> Numeric,
        angular_velocity_z -> Numeric,

        yaw -> Numeric,
        pitch -> Numeric,
        roll -> Numeric,

        normalized_suspension_travel_front_left -> Numeric,
        normalized_suspension_travel_front_right -> Numeric,
        normalized_suspension_travel_rear_left -> Numeric,
        normalized_suspension_travel_rear_right -> Numeric,

        tire_slip_ratio_front_left -> Numeric,
        tire_slip_ratio_front_right -> Numeric,
        tire_slip_ratio_rear_left -> Numeric,
        tire_slip_ratio_rear_right -> Numeric,

        wheel_rotation_speed_front_left -> Numeric,
        wheel_rotation_speed_front_right -> Numeric,
        wheel_rotation_speed_rear_left -> Numeric,
        wheel_rotation_speed_rear_right -> Numeric,

        wheel_on_rumble_strip_front_left -> Bool,
        wheel_on_rumble_strip_front_right -> Bool,
        wheel_on_rumble_strip_rear_left -> Bool,
        wheel_on_rumble_strip_rear_right -> Bool,

        wheel_in_puddle_front_left -> Numeric,
        wheel_in_puddle_front_right -> Numeric,
        wheel_in_puddle_rear_left -> Numeric,
        wheel_in_puddle_rear_right -> Numeric,

        surface_rumble_front_left -> Numeric,
        surface_rumble_front_right -> Numeric,
        surface_rumble_rear_left -> Numeric,
        surface_rumble_rear_right -> Numeric,

        tire_slip_angle_front_left -> Numeric,
        tire_slip_angle_front_right -> Numeric,
        tire_slip_angle_rear_left -> Numeric,
        tire_slip_angle_rear_right -> Numeric,

        tire_combined_slip_front_left -> Numeric,
        tire_combined_slip_front_right -> Numeric,
        tire_combined_slip_rear_left -> Numeric,
        tire_combined_slip_rear_right -> Numeric,

        suspension_travel_meters_front_left -> Numeric,
        suspension_travel_meters_front_right -> Numeric,
        suspension_travel_meters_rear_left -> Numeric,
        suspension_travel_meters_rear_right -> Numeric,

        position_x -> Nullable<Numeric>,
        position_y -> Nullable<Numeric>,
        position_z -> Nullable<Numeric>,

        speed -> Nullable<Numeric>,
        power -> Nullable<Numeric>,
        torque -> Nullable<Numeric>,

        tire_temp_front_left -> Nullable<Numeric>,
        tire_temp_front_right -> Nullable<Numeric>,
        tire_temp_rear_left -> Nullable<Numeric>,
        tire_temp_rear_right -> Nullable<Numeric>,

        boost -> Nullable<Numeric>,
        fuel -> Nullable<Numeric>,
        distance_traveled -> Nullable<Numeric>,

        best_lap -> Nullable<Numeric>,
        last_lap -> Nullable<Numeric>,
        current_lap -> Nullable<Numeric>,
        current_race_time -> Nullable<Numeric>,
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
        tire_wear_front_left -> Nullable<Numeric>,
        tire_wear_front_right -> Nullable<Numeric>,
        tire_wear_rear_left -> Nullable<Numeric>,
        tire_wear_rear_right -> Nullable<Numeric>,
    }
}
