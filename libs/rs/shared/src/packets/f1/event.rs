use std::str::FromStr;

use binrw::BinRead;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{
    packets::f1::enums::{ButtonStatus, MAX_NUM_CARS},
    utils::{u8_to_bool, u8_to_usize},
};

use super::enums::{InfringementType, PenaltyType, SafetyCarEventType, SafetyCarType};

#[non_exhaustive]
#[derive(
    BinRead,
    Display,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Copy,
    Clone,
    Debug,
    Hash,
    Serialize,
    Deserialize,
    EnumString,
)]
#[br(little, repr(u8))]
pub enum EventCode {
    /// Session started.
    #[strum(serialize = "SSTA")]
    SSTA,
    /// Session ended.
    #[strum(serialize = "SEND")]
    SEND,
    /// Fastest lap.
    #[strum(serialize = "FTLP")]
    FTLP,
    /// Retirement.
    #[strum(serialize = "RTMT")]
    RTMT,
    /// DRS enabled.
    #[strum(serialize = "DRSE")]
    DRSE,
    /// DRS disabled.
    #[strum(serialize = "DRSD")]
    DRSD,
    /// Teammate in pits.
    #[strum(serialize = "TMPT")]
    TMPT,
    /// Chequered flag.
    #[strum(serialize = "CHQF")]
    CHQF,
    /// Race winner.
    #[strum(serialize = "RCWN")]
    RCWN,
    /// Penalty.
    #[strum(serialize = "PENA")]
    PENA,
    /// Speed trap.
    #[strum(serialize = "SPTP")]
    SPTP,
    /// Start lights.
    #[strum(serialize = "STLG")]
    STLG,
    /// Lights out.
    #[strum(serialize = "LGOT")]
    LGOT,
    /// Drive through served.
    #[strum(serialize = "DTSV")]
    DTSV,
    /// Stop-go served.
    #[strum(serialize = "SGSV")]
    SGSV,
    /// Flashback.
    #[strum(serialize = "FLBK")]
    FLBK,
    /// Buttons.
    #[strum(serialize = "BUTN")]
    BUTN,
    /// Red flag.
    #[strum(serialize = "RDFL")]
    RDFL,
    /// Overtake.
    #[strum(serialize = "OVTK")]
    OVTK,
    /// Safety car.
    #[strum(serialize = "SCAR")]
    SCAR,
    /// Collision.
    #[strum(serialize = "COLL")]
    COLL,
}

impl EventCode {
    pub fn from_bytes(bytes: [u8; 4]) -> Result<Self, String> {
        let code = String::from_utf8(bytes.to_vec()).unwrap();
        match EventCode::from_str(&code) {
            Ok(event_code) => Ok(event_code),
            Err(_) => Err(format!("Invalid event code: {}", code)),
        }
    }
}

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(_packet_format: u16))]
pub enum EventDetails {
    /// Sent when the session starts.
    #[br(magic = b"SSTA")]
    SessionStarted,
    /// Sent when the session ends.
    #[br(magic = b"SEND")]
    SessionEnded,
    /// Sent when a driver achieves the fastest lap.
    #[br(magic = b"FTLP")]
    FastestLap {
        /// Index of the car that's achieved the fastest lap.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Fastest lap event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
        /// Lap time in seconds.
        lap_time: f32,
    },
    /// Sent when a driver retires.
    #[br(magic = b"RTMT")]
    Retirement {
        /// Index of the retiring car.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Retirement event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
    },
    /// Sent when race control enable DRS.
    #[br(magic = b"DRSE")]
    DrsEnabled,
    /// Sent when race control disable DRS.
    #[br(magic = b"DRSD")]
    DrsDisabled,
    /// Sent when your teammate enters the pit lane.
    #[br(magic = b"TMPT")]
    TeamMateInPits {
        /// Index of teammate's car.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Teammate in pits event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
    },
    /// Sent when the chequered flag has been waved.
    #[br(magic = b"CHQF")]
    ChequeredFlag,
    /// Sent when the race winner has been announced.
    #[br(magic = b"RCWN")]
    RaceWinner {
        /// Index of race winner's car.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Race winner event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
    },
    /// Sent when a penalty has been issued.
    #[br(magic = b"PENA")]
    Penalty {
        /// Penalty type.
        penalty_type: PenaltyType,
        /// Infringement type.
        infringement_type: InfringementType,
        /// Index of the car the penalty is applied to.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Penalty event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
        /// Index of the other car involved.
        /// Set to 255 if only one driver is involved.
        #[br(map(u8_to_usize))]
        other_vehicle_index: usize,
        /// Time gained/spent doing the action in seconds.
        time: u8,
        /// Number of the lap the infringement occurred on.
        lap_num: u8,
        /// Number of places gained by this infringement.
        places_gained: u8,
    },
    /// Sent when a speed trap is triggered.
    #[br(magic = b"SPTP")]
    SpeedTrap {
        /// Index of the car that's triggered the speed trap.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Speed trap event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
        /// Top speed achieved in kilometres per hour.
        speed: f32,
        /// Whether the driver is overall fastest in the session.
        #[br(try_map(u8_to_bool))]
        is_overall_fastest_in_session: bool,
        /// Whether this speed is personal fastest in the session.
        #[br(try_map(u8_to_bool))]
        is_driver_fastest_in_session: bool,
        /// Index of the vehicle that's the fastest in the session.
        #[br(
            map(u8_to_usize),
            assert(
                fastest_vehicle_index < MAX_NUM_CARS,
                "Speed trap event has an invalid fastest vehicle index: {}",
                fastest_vehicle_index
            )
        )]
        fastest_vehicle_index: usize,
        /// Fastest speed in the session in kilometres per hour.
        fastest_speed_in_session: f32,
    },
    /// Sent when a start light is lit.
    #[br(magic = b"STLG")]
    StartLights {
        /// Number of lights showing.
        num_lights: u8,
    },
    /// "It's lights out, and away we go!"
    #[br(magic = b"LGOT")]
    LightsOut,
    /// Sent when a driver has served a drive-through penalty.
    #[br(magic = b"DTSV")]
    DriveThroughServed {
        /// Index of the vehicle serving the penalty.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Drive-through served event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
    },
    /// Sent when a driver has served a stop-go penalty.
    #[br(magic = b"SGSV")]
    StopGoServed {
        /// Index of the vehicle serving the penalty.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Stop-go served event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
    },
    /// Sent when a flashback is activated.
    #[br(magic = b"FLBK")]
    Flashback {
        /// Frame identifier that's been flashed back to.
        frame_identifier: u32,
        /// Session time that's been flashed back to.
        flashback_session_time: f32,
    },
    /// Sent when the button status has changed.
    #[br(magic = b"BUTN")]
    Buttons {
        /// Bitmap specifying which buttons are currently pressed.
        #[br(map(ButtonStatus::from_bits_retain))]
        button_status: ButtonStatus,
    },
    /// Sent when the red flag is shown.
    /// Available from the 2023 format onwards.
    #[br(magic = b"RDFL")]
    RedFlag,
    /// Sent when a car has overtaken another.
    /// Available from the 2023 format onwards.
    #[br(magic = b"OVTK")]
    Overtake {
        /// Index of the overtaking vehicle.
        #[br(
            map(u8_to_usize),
            assert(
                overtaking_vehicle_index < MAX_NUM_CARS,
                "Overtake event has an invalid overtaking vehicle index: {}",
                overtaking_vehicle_index
            )
        )]
        overtaking_vehicle_index: usize,
        /// Index of the overtaken vehicle.
        #[br(
            map(u8_to_usize),
            assert(
                overtaken_vehicle_index < MAX_NUM_CARS,
                "Collision event has an invalid overtaken vehicle index: {}",
                overtaken_vehicle_index
            )
        )]
        overtaken_vehicle_index: usize,
    },
    /// Sent when safety car gets deployed.
    /// Available from the 2024 format onwards.
    #[br(magic = b"SCAR")]
    SafetyCar {
        /// Type of the safety car that's been deployed.
        safety_car_type: SafetyCarType,
        /// New safety car deployment status.
        event_type: SafetyCarEventType,
    },
    /// Sent when two vehicles collide.
    /// Available from the 2024 format onwards.
    #[br(magic = b"COLL")]
    Collision {
        /// Index of the first vehicle involved in the collision.
        #[br(
            map(u8_to_usize),
            assert(
                vehicle_index < MAX_NUM_CARS,
                "Collision event has an invalid vehicle index: {}",
                vehicle_index
            )
        )]
        vehicle_index: usize,
        /// Index of the second vehicle involved in the collision.
        #[br(
            map(u8_to_usize),
            assert(
                other_vehicle_index < MAX_NUM_CARS,
                "Collision event has an invalid other vehicle index: {}",
                other_vehicle_index
            )
        )]
        other_vehicle_index: usize,
    },
}
