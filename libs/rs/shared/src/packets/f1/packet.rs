use super::{
    car_damage::CarDamageData,
    car_setups::CarSetupData,
    car_status::CarStatusData,
    car_telemetry::CarTelemetryData,
    enums::{
        BrakingAssist, CarDamage, CarDamageRate, Collisions, CornerCuttingStringency,
        DynamicRacingLine, DynamicRacingLineType, FlashbackLimit, ForecastAccuracy,
        FormationLapExperience, Formula, GameMode, GearboxAssist, LowFuelMode, MfdPanelIndex,
        PitStopExperience, RaceStarts, RecoveryMode, RedFlagIntensity, RuleSet,
        SafetyCarExperience, SafetyCarIntensity, SafetyCarStatus, SessionLength, SpeedUnit,
        SurfaceSimType, TemperatureUnit, TrackId, TyreTemperature, Weather, MAX_NUM_CARS,
    },
    event::{EventCode, EventDetails},
    final_classification::FinalClassificationData,
    headers::{F1PacketHeader, F1PacketId},
    laps::LapData,
    lobby::LobbyInfoData,
    motion::CarMotionData,
    participants::ParticipantsData,
    session::{
        check_num_forecast_samples, get_forecast_samples_padding, MarshalZone,
        WeatherForecastSample, MARSHAL_ZONE_RAW_SIZE, MAX_AI_DIFFICULTY, MAX_NUM_MARSHAL_ZONES,
        MAX_NUM_SESSIONS,
    },
    session_history::{
        get_lap_history_raw_size, LapHistoryData, TyreStintHistoryData, MAX_NUM_LAPS,
        MAX_NUM_TYRE_STINTS,
    },
    time_trial::TimeTrialDataSet,
    tyre_sets::{TyreSetData, NUM_TYRE_SETS},
};
use crate::utils::{u8_to_bool, u8_to_usize};
use binrw::BinRead;
#[cfg(feature = "db")]
use derive_jsonb::AsJsonb;
use serde::{Deserialize, Serialize};

/// Structured representation of raw F1 game packet data that's
/// returned as a successful result of the [`parse`] function.
///
/// Each [`Option`] field acts as a slot for a payload of a packet of a certain type.
/// Only one of these fields can be [`Some`] for a given `F1Packet` instance.
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little)]
pub struct F1Packet {
    /// Universal packet header.
    pub header: F1PacketHeader,
    /// Physics data for all cars in the ongoing session.
    #[br(if(header.packet_id == F1PacketId::Motion), args(header.packet_format))]
    pub motion: Option<F1PacketMotion>,
    /// Data about the ongoing session.
    #[br(if(header.packet_id == F1PacketId::Session), args(header.packet_format))]
    pub session: Option<F1PacketSession>,
    /// Lap data for all cars on track.
    #[br(if(header.packet_id == F1PacketId::Laps), args(header.packet_format))]
    pub laps: Option<F1PacketLaps>,
    /// Details of events that happen during the course of the ongoing session.
    #[br(if(header.packet_id == F1PacketId::Event), args(header.packet_format))]
    pub event: Option<F1PacketEvent>,
    /// List of participants in the session.
    #[br(if(header.packet_id == F1PacketId::Participants), args(header.packet_format))]
    pub participants: Option<F1PacketParticipants>,
    /// Setup data for all cars in the ongoing session.
    #[br(if(header.packet_id == F1PacketId::CarSetups), args(header.packet_format))]
    pub car_setups: Option<F1PacketCarSetups>,
    /// Telemetry data for all cars in the ongoing session.
    #[br(if(header.packet_id == F1PacketId::CarTelemetry), args(header.packet_format))]
    pub car_telemetry: Option<F1PacketCarTelemetry>,
    /// Status data for all cars in the ongoing session.
    #[br(if(header.packet_id == F1PacketId::CarStatus), args(header.packet_format))]
    pub car_status: Option<F1PacketCarStatus>,
    /// Final classification confirmation at the end of the session.
    #[br(
        if(header.packet_id == F1PacketId::FinalClassification),
        args(header.packet_format)
    )]
    pub final_classification: Option<F1PacketFinalClassification>,
    /// Details of players in a multiplayer lobby.
    #[br(if(header.packet_id == F1PacketId::LobbyInfo), args(header.packet_format))]
    pub lobby: Option<F1PacketLobby>,
    /// Car damage parameters for all cars in the ongoing session.
    #[br(if(header.packet_id == F1PacketId::CarDamage), args(header.packet_format))]
    pub car_damage: Option<F1PacketCarDamage>,
    /// Session history data for a specific car.
    #[br(if(header.packet_id == F1PacketId::SessionHistory), args(header.packet_format))]
    pub session_history: Option<F1PacketSessionHistory>,
    /// Details of tyre sets assigned to a vehicle during the session.
    /// Available from the 2023 format onwards.
    #[br(if(header.packet_id == F1PacketId::TyreSets), args(header.packet_format))]
    pub tyre_sets: Option<F1PacketTyreSets>,
    /// Extended player car only motion data.
    /// Available from the 2023 format onwards.
    #[br(if(header.packet_id == F1PacketId::MotionEx), args(header.packet_format))]
    pub motion_ex: Option<F1PacketMotionEx>,
    /// Extra information that's only relevant to the time trial game mode.
    /// Available from the 2024 format onwards.
    #[br(if(header.packet_id == F1PacketId::TimeTrial), args(header.packet_format))]
    pub time_trial: Option<F1PacketTimeTrial>,
}

/// The session packet includes details about the current session in progress.
///
/// ```
/// Frequency: 2 per second
/// Size: 753 bytes
/// ```
#[allow(clippy::struct_excessive_bools)]
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketSession {
    /// Current weather.
    pub weather: Weather,
    /// Track temperature in degrees Celsius.
    pub track_temperature: i8,
    /// Air temperature in degrees Celsius.
    pub air_temperature: i8,
    /// Total number of laps in this session.
    pub total_laps: u8,
    /// Track's length in metres.
    pub track_length: u16,
    /// Session's type.
    /// See [`session_type`](mod@crate::packets::f1::enums::SessionType)
    /// for possible values.
    pub session_type: u8,
    /// Unique identifier of the track.
    pub track_id: TrackId,
    /// Formula of cars being raced.
    pub formula: Formula,
    /// Time left in the session in seconds.
    pub session_time_left: u16,
    /// Session's duration in seconds.
    pub session_duration: u16,
    /// Pit lane's speed limit in kilometres per hour.
    pub pit_speed_limit: u8,
    /// Whether the game is paused.
    #[br(try_map(u8_to_bool))]
    pub game_paused: bool,
    /// Whether the player is spectating.
    #[br(try_map(u8_to_bool))]
    pub is_spectating: bool,
    /// Index of the car being spectated.
    #[br(map(u8_to_usize))]
    pub spectator_car_index: usize,
    /// Whether SLI Pro support is active.
    #[br(try_map(u8_to_bool))]
    pub sli_pro_native_support: bool,
    /// Number of marshal zones to follow.
    #[br(
        map(u8_to_usize),
        assert(
            num_marshal_zones <= MAX_NUM_MARSHAL_ZONES,
            "Session packet has an invalid number of marshal zones: {}",
            num_marshal_zones
        )
    )]
    pub num_marshal_zones: usize,
    /// List of marshal zones.
    /// Should have a size equal to
    /// [`num_marshal_zones`](field@crate::packets::F1PacketSession::num_marshal_zones).
    #[br(
        count(num_marshal_zones),
        args{ inner: (packet_format,) },
        pad_after(
            (MAX_NUM_MARSHAL_ZONES - num_marshal_zones) * MARSHAL_ZONE_RAW_SIZE
        )
    )]
    pub marshal_zones: Vec<MarshalZone>,
    /// Safety car deployment status.
    pub safety_car_status: SafetyCarStatus,
    /// Whether this game is online.
    #[br(try_map(u8_to_bool))]
    pub network_game: bool,
    /// Number of weather samples to follow.
    #[br(
        map(u8_to_usize),
        assert(
            check_num_forecast_samples(packet_format, num_weather_forecast_samples),
            "Session packet has an invalid number of weather forecast samples: {}",
            num_weather_forecast_samples
        )
    )]
    pub num_weather_forecast_samples: usize,
    /// List of up to weather forecast samples.
    /// Should have a size equal to
    /// [`num_weather_forecast_samples`](field@crate::packets::F1PacketSession::num_weather_forecast_samples).
    #[br(
        count(num_weather_forecast_samples),
        args{ inner: (packet_format,) },
        pad_after(
            get_forecast_samples_padding(packet_format, num_weather_forecast_samples)
        )
    )]
    pub weather_forecast_samples: Vec<WeatherForecastSample>,
    /// Weather forecast accuracy.
    pub forecast_accuracy: ForecastAccuracy,
    /// AI difficulty rating in range `(0..=110)`.
    #[br(
        assert(
            ai_difficulty <= MAX_AI_DIFFICULTY,
            "Session packet has an invalid AI difficulty value: {}",
            ai_difficulty
        )
    )]
    pub ai_difficulty: u8,
    /// Identifier for season - persists across saves.
    pub season_link_identifier: u32,
    /// Identifier for weekend - persists across saves.
    pub weekend_link_identifier: u32,
    /// Identifier for session - persists across saves.
    pub session_link_identifier: u32,
    /// Ideal lap for the player to pit on for current strategy.
    pub pit_stop_window_ideal_lap: u8,
    /// The latest lap for the player to pit on for current strategy.
    pub pit_stop_window_latest_lap: u8,
    /// Predicted position for the player to rejoin at.
    pub pit_stop_rejoin_position: u8,
    /// Whether the steering assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub steering_assist: bool,
    /// Type of braking assist enabled.
    pub braking_assist: BrakingAssist,
    /// Type of gearbox assist enabled.
    pub gearbox_assist: GearboxAssist,
    /// Whether the pit assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub pit_assist: bool,
    /// Whether the pit release assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub pit_release_assist: bool,
    /// Whether the ERS assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub ers_assist: bool,
    /// Whether the DRS assist is enabled.
    #[br(try_map(u8_to_bool))]
    pub drs_assist: bool,
    /// Type of the dynamic racing line assist.
    pub dynamic_racing_line: DynamicRacingLine,
    /// Type of the dynamic racing line (2D/3D).
    pub dynamic_racing_line_type: DynamicRacingLineType,
    /// Game mode's identifier.
    pub game_mode: GameMode,
    /// Rule set's identifier.
    pub rule_set: RuleSet,
    /// Local time of day as minutes since midnight.
    pub time_of_day: u32,
    /// Session's length.
    pub session_length: SessionLength,
    /// Speed unit used by player 1.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub speed_unit_lead_player: Option<SpeedUnit>,
    /// Temperature unit used by player 1.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub temperature_unit_lead_player: Option<TemperatureUnit>,
    /// Speed unit used by player 2.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub speed_unit_secondary_player: Option<SpeedUnit>,
    /// Temperature unit used by player 2.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub temperature_unit_secondary_player: Option<TemperatureUnit>,
    /// Number of full safety cars called during the session.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub num_safety_car_periods: u8,
    /// Number of virtual safety cars called during the session.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub num_virtual_safety_car_periods: u8,
    /// Number of red flags called during the session.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub num_red_flag_periods: u8,
    /// Whether equal car performance is enabled.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub equal_car_performance: bool,
    /// Recovery mode assist.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub recovery_mode: Option<RecoveryMode>,
    /// Flashback limit type.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub flashback_limit: Option<FlashbackLimit>,
    /// Surface simulation type.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub surface_sim_type: Option<SurfaceSimType>,
    /// Low fuel driving difficulty.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub low_fuel_mode: Option<LowFuelMode>,
    /// Race starts assist.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub race_starts: Option<RaceStarts>,
    /// Tyre temperature simulation type.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub tyre_temperature: Option<TyreTemperature>,
    /// Whether the pit lane tyre simulation
    /// (cold tyres and low grip right after a stop) is enabled.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub pit_lane_tyre_sim: bool,
    /// Car damage simulation type.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub car_damage: Option<CarDamage>,
    /// Car damage rate.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub car_damage_rate: Option<CarDamageRate>,
    /// Collision simulation type.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub collisions: Option<Collisions>,
    /// Whether collisions are disabled only for lap 1.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub collisions_off_for_first_lap_only: bool,
    /// Whether unsafe pit release is disabled in a multiplayer game.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub mp_unsafe_pit_release_disabled: bool,
    /// Whether collisions get disabled for griefing in a multiplayer game.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub mp_collisions_off_for_griefing: bool,
    /// Corner cutting stringency.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub corner_cutting_stringency: Option<CornerCuttingStringency>,
    /// Whether parc fermé rules are enabled.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub parc_ferme_rules: bool,
    /// Pit stop experience.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub pit_stop_experience: Option<PitStopExperience>,
    /// Safety car intensity.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub safety_car_intensity: Option<SafetyCarIntensity>,
    /// Safety car experience.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub safety_car_experience: Option<SafetyCarExperience>,
    /// Whether formation lap is enabled.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub formation_lap: bool,
    /// Formation lap experience.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub formation_lap_experience: Option<FormationLapExperience>,
    /// Red flag intensity.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub red_flag_intensity: Option<RedFlagIntensity>,
    /// Whether this single player game affects the license level.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub affects_license_level_solo: bool,
    /// Whether this multiplayer game affects the license level.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024), try_map(u8_to_bool))]
    pub affects_license_level_mp: bool,
    /// Number of sessions in the ongoing race weekend.
    #[br(
        map(u8_to_usize),
        if(packet_format >= 2024),
        assert(
            num_sessions_in_weekend <= MAX_NUM_SESSIONS,
            "Session packet has an invalid number of sessions in a weekend: {}",
            num_sessions_in_weekend
        )
    )]
    pub num_sessions_in_weekend: usize,
    /// List of sessions that shows this weekend's structure.
    /// Should have a size equal to
    /// [`num_sessions_in_weekend`](field@crate::packets::F1PacketSession::num_sessions_in_weekend).
    /// See [`session_type`](mod@crate::constants::session_type)
    /// for possible values.
    /// Available from the 2024 format onwards.
    #[br(
        if(packet_format >= 2024),
        count(num_sessions_in_weekend),
        pad_after(MAX_NUM_SESSIONS - num_sessions_in_weekend)
    )]
    pub weekend_structure: Vec<u8>,
    /// Distance (in metres) around the track where sector 2 starts.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub sector2_lap_distance_start: f32,
    /// Distance (in metres) around the track where sector 3 starts.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub sector3_lap_distance_start: f32,
}

/// Data about all the lap times of cars in the session.
///
/// ```
/// Frequency: In game settings
/// Size: 1285 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketLaps {
    /// Lap data for all cars on track. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub data: Vec<LapData>,
    /// Index of personal best car in time trial mode (255 if invalid).
    #[br(map(u8_to_usize))]
    pub time_trial_pb_car_index: usize,
    /// Index of rival's car in time trial mode (255 if invalid).
    #[br(map(u8_to_usize))]
    pub time_trial_rival_car_index: usize,
}

/// Various notable events that happen during a session.
///
/// ```
/// Frequency: When event occurs
/// Size: 45 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(packet_format: u16))]
pub struct F1PacketEvent {
    /// 4-letter event code.
    #[br(
        try_map(|bytes: [u8; 4]| EventCode::from_bytes(bytes)),
        restore_position
    )]
    pub code: EventCode,
    /// Extra data for this event.
    #[br(args(packet_format))]
    pub details: EventDetails,
}

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
/// The motion packet gives physics data for all the cars being driven.
///
/// N.B. For the normalised vectors below, to convert to float values divide by 32767.0f –
/// 16-bit signed values are used to pack the data and on the assumption that direction values
/// are always between -1.0f and 1.0f.
///
/// ```
/// Frequency: In game settings
/// Size: 1349 bytes
/// ```
pub struct F1PacketMotion {
    /// Motion data for all cars on track. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub data: Vec<CarMotionData>,
    /// Extra player-car-only motion data.
    /// Available only in the 2022 format.
    #[br(if(packet_format == 2022))]
    pub motion_ex: Option<F1PacketMotionEx>,
}

/// Extended motion data for player's car. Available as a:
/// - part of [`F1PacketMotion`](mod@crate::packets::f1::packet::F1PacketMotion) in the 2022 format
/// - standalone packet from the 2023 format onwards
///
/// ```
/// Frequency: In game settings
/// Size: 237 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketMotionEx {
    /// Positions of suspension for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub suspension_position: [f32; 4],
    /// Velocity values of suspension for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub suspension_velocity: [f32; 4],
    /// Acceleration values of suspension for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub suspension_acceleration: [f32; 4],
    /// Speed of each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub wheel_speed: [f32; 4],
    /// Slip ratio for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub wheel_slip_ratio: [f32; 4],
    /// Slip angles for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub wheel_slip_angle: [f32; 4],
    /// Lateral forces for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub wheel_lat_force: [f32; 4],
    /// Longitudinal forces for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub wheel_long_force: [f32; 4],
    /// Height of centre of gravity above ground.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub height_of_cog_above_ground: f32,
    /// X velocity in local space.
    pub local_velocity_x: f32,
    /// Y velocity in local space.
    pub local_velocity_y: f32,
    /// Z velocity in local space.
    pub local_velocity_z: f32,
    /// Angular velocity X component.
    pub angular_velocity_x: f32,
    /// Angular velocity Y component.
    pub angular_velocity_y: f32,
    /// Angular velocity Z component.
    pub angular_velocity_z: f32,
    /// Angular acceleration X component.
    pub angular_acceleration_x: f32,
    /// Angular acceleration Y component.
    pub angular_acceleration_y: f32,
    /// Angular acceleration Z component.
    pub angular_acceleration_z: f32,
    /// Current front wheels angle in radians.
    pub front_wheels_angle: f32,
    /// Vertical forces for each wheel.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    /// Available from the 2023 format onwards.
    #[br(if(packet_format >= 2023))]
    pub wheel_vert_force: [f32; 4],
    /// Front plank edge height above road surface.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub front_aero_height: f32,
    /// Rear plank edge height above road surface.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub rear_aero_height: f32,
    /// Roll angle of the front suspension.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub front_roll_angle: f32,
    /// Roll angle of the rear suspension.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub rear_roll_angle: f32,
    /// Yaw angle of the chassis relative to the direction of motion - radians.
    /// Available from the 2024 format onwards.
    #[br(if(packet_format >= 2024))]
    pub chassis_yaw: f32,
}

/// Data of participants in the session, mostly relevant for multiplayer.
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(
    little,
    import(packet_format: u16),
    assert(
        num_active_cars <= MAX_NUM_CARS,
        "Participants packet has an invalid number of cars: {}",
        num_active_cars
    )
)]
/// This is a list of participants in the race. If the vehicle is controlled by AI, then the name will be the driver name. If this is a multiplayer game, the names will be the Steam Id on PC, or the LAN name if appropriate.
///
/// N.B. on Xbox One, the names will always be the driver name, on PS4 the name will be the LAN name if
/// playing a LAN game, otherwise it will be the driver name.
///
/// ```
/// Frequency: Every 5 seconds
/// Size: 1350 bytes
/// ```
pub struct F1PacketParticipants {
    /// Number of active cars in the session.
    #[br(map(u8_to_usize))]
    pub num_active_cars: usize,
    /// Data for all participants.
    /// Should have a size equal to
    /// [`num_active_cars`](field@crate::packets::f1::packet::F1PacketParticipants::num_active_cars).
    #[br(count(num_active_cars), args{ inner: (packet_format,) })]
    pub data: Vec<ParticipantsData>,
}

/// Car setups for all cars in the race.
/// In multiplayer games, other player cars will appear as blank.
/// You will only be able to see your car setup and AI cars.
///
/// ```
/// Frequency: 2 per second
/// Size: 1133 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketCarSetups {
    /// Setup data for all cars on track. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub data: Vec<CarSetupData>,
    /// Value of front wing after next pit stop - player only.
    /// Available from the 2024 format onwards
    #[br(if(packet_format >= 2024))]
    pub next_front_wing_value: f32,
}

/// Telemetry (such as speed, DRS, throttle application, etc.)
/// for all cars in the race.
///
/// ```
/// Frequency: In game settings
/// Size: 1352 bytes
/// ```
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketCarTelemetry {
    /// Telemetry data for all cars on track. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub data: Vec<CarTelemetryData>,
    /// Index of currently open MFD panel for player 1.
    pub mfd_panel_index: MfdPanelIndex,
    /// Index of currently open MFD panel for player 2.
    pub mfd_panel_index_secondary_player: MfdPanelIndex,
    /// Suggested gear (0 if no gear suggested).
    #[br(
        assert(
            (-1..=8).contains(&suggested_gear),
            "Car telemetry entry has an invalid suggested gear value: {}",
            suggested_gear
        )
    )]
    pub suggested_gear: i8,
}

/// Car status data for each car in the race.
///
/// ```
/// Frequency: In game settings
/// Size: 1239 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketCarStatus {
    /// Car status data for all cars. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub data: Vec<CarStatusData>,
}

/// Final classification confirmation at the end of a race.
///
/// ```
/// Frequency: When race is finished
/// Size: 1020 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketFinalClassification {
    /// Number of cars in the final classification.
    #[br(
        map(u8_to_usize),
        assert(
            num_cars <= MAX_NUM_CARS,
            "Final classification packet has an invalid number of cars: {}",
            num_cars
        )
    )]
    pub num_cars: usize,
    /// Final classification data for all cars.
    /// Should have a size equal to
    /// [`num_cars`](field@crate::packets::f1::packet::F1PacketFinalClassification::num_cars).
    #[br(count(num_cars), args{ inner: (packet_format,) })]
    pub data: Vec<FinalClassificationData>,
}

/// Packet detailing all the players that are currently in a multiplayer lobby.
///
/// ```
/// Frequency: 2 every seconds when in a multiplayer lobby
/// Size: 1306 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketLobby {
    /// Number of players in the lobby.
    #[br(
        map(u8_to_usize),
        assert(
            num_players <= MAX_NUM_CARS,
            "Lobby packet has an invalid number of players: {}",
            num_players
        )
    )]
    pub num_players: usize,
    /// Lobby info data for all players.
    /// Should have a size equal to
    /// [`num_players`](field@crate::packets::f1::packet::F1PacketLobby::num_players).
    #[br(count(num_players), args{ inner: (packet_format,) })]
    pub data: Vec<LobbyInfoData>,
}

/// Car damage parameters for all cars in the session.
///
/// ```
/// Frequency: 10 per second
/// Size: 953 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(packet_format: u16))]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct F1PacketCarDamage {
    /// Car damage data. Should have a size of 22.
    #[br(count(MAX_NUM_CARS), args{ inner: (packet_format,) })]
    pub data: Vec<CarDamageData>,
}

/// Packet detailing lap and tyre data history for a given driver in the session.
///
/// ```
/// Frequency: 20 per second but cycles for each car
/// Size: 1460 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketSessionHistory {
    /// Index of the car this packet refers to.
    #[br(
        map(u8_to_usize),
        assert(
            vehicle_index < MAX_NUM_CARS,
            "Session history packet has an invalid vehicle index: {}",
            vehicle_index
        )
    )]
    pub vehicle_index: usize,
    /// Number of laps in the data (including the current one).
    #[br(
        map(u8_to_usize),
        assert(
            num_laps <= MAX_NUM_LAPS,
            "Session history packet has an invalid number of laps: {}",
            num_laps
        ),
    )]
    pub num_laps: usize,
    /// Number of tyre stints in the data (including the current one).
    #[br(
        map(u8_to_usize),
        assert(
            num_tyre_stints <= MAX_NUM_TYRE_STINTS,
            "Session history packet has an invalid number of tyre stints: {}",
            num_tyre_stints
        )
    )]
    pub num_tyre_stints: usize,
    /// Number of the lap the best lap time was achieved on.
    #[br(map(u8_to_usize))]
    pub best_lap_time_lap_num: usize,
    /// Number of the lap the best sector 1 time was achieved on.
    #[br(map(u8_to_usize))]
    pub best_sector1_lap_num: usize,
    /// Number of the lap the best sector 2 time was achieved on.
    #[br(map(u8_to_usize))]
    pub best_sector2_lap_num: usize,
    /// Number of the lap the best sector 3 time was achieved on.
    #[br(map(u8_to_usize))]
    pub best_sector3_lap_num: usize,
    /// Lap history. Should have a size equal to
    /// [`num_laps`](field@crate::packets::f1::packet::F1PacketSessionHistory::num_laps).
    #[br(
        count(num_laps),
        args{ inner: (packet_format,) },
        pad_after((MAX_NUM_LAPS - num_laps) * get_lap_history_raw_size(packet_format))
    )]
    pub lap_history_data: Vec<LapHistoryData>,
    /// Tyre stint history.
    /// Should have a size equal to
    /// [`num_tyre_stints`](field@crate::packets::f1::packet::F1PacketSessionHistory::num_tyre_stints).
    #[br(count(num_tyre_stints), args{ inner: (packet_format,) })]
    pub tyre_stint_history_data: Vec<TyreStintHistoryData>,
}

/// In-depth details about tyre sets assigned to a vehicle during the session.
/// Available from the 2023 format onwards.
///
/// ```
/// Frequency: 20 per second but cycles for each car
/// Size: 231 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(packet_format: u16))]
pub struct F1PacketTyreSets {
    /// Index of the car this packet relates to.
    #[br(map(u8_to_usize))]
    pub vehicle_index: usize,
    /// 13 dry + 7 wet tyre sets.
    #[br(count(NUM_TYRE_SETS), args{ inner: (packet_format,) })]
    pub data: Vec<TyreSetData>,
    /// Index of fitted tyre set.
    #[br(
        map(u8_to_usize),
        assert(
            fitted_index < NUM_TYRE_SETS,
            "Tyre sets packet has an invalid fitted set index: {}",
            fitted_index
        )
    )]
    pub fitted_index: usize,
}

/// Extra information that's only relevant to time trial game mode.
/// Available from the 2024 format onwards.
///
/// ```
/// Frequency: 1 per second
/// Size: 101 bytes
/// ```
#[non_exhaustive]
#[derive(BinRead, Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "db", derive(AsJsonb))]
#[br(little, import(_packet_format: u16))]
pub struct F1PacketTimeTrial {
    /// Data set of player's best run this session.
    pub player_session_best_data_set: TimeTrialDataSet,
    /// Data set of player's personal best run.
    pub personal_best_data_set: TimeTrialDataSet,
    /// Data set of rival's best run.
    pub rival_data_set: TimeTrialDataSet,
}
