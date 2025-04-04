use binrw::BinRead;
use serde::{Deserialize, Serialize};

#[cfg(feature = "db")]
use derive_jsonb::AsJsonb;

use crate::utils::u8_to_bool;

use super::enums::{RevLights, Surface};

// f32 range ends were offset by a tenth to account for values ever so slightly
// greater or lower than the respective extreme.
// There's probably a better way of going about handling this,
// but this will do for now.

#[non_exhaustive]
#[derive(BinRead, PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
#[br(little, import(_packet_format: u16))]
#[cfg_attr(feature = "db", derive(AsJsonb))]
pub struct CarTelemetryData {
    /// Speed of the car in kilometres per hour.
    pub speed: u16,
    /// Amount of throttle applied. Value in range `(0.0..=1.0)`.
    #[br(
        assert(
            (-0.1..=1.1).contains(&throttle),
            "Car telemetry entry has an invalid throttle value: {}",
            throttle
        ),
    )]
    pub throttle: f32,
    /// Steering lock. Value in range `(-1.0..=1.0)`.
    #[br(
        assert(
            (-1.1..=1.1).contains(&steer),
            "Car telemetry entry has an invalid steering lock value: {}",
            steer
        ),
    )]
    pub steer: f32,
    /// Amount of brake applied. Value in range `(0.0..=1.0)`.
    #[br(
        assert(
            (-0.1..=1.1).contains(&brake),
            "Car telemetry entry has an invalid brake value: {}",
            brake
        ),
    )]
    pub brake: f32,
    /// Amount of clutch applied (percentage).
    #[br(
        assert(
            clutch <= 100,
            "Car telemetry entry has an invalid clutch value: {}",
            clutch
        ),
    )]
    pub clutch: u8,
    /// Selected gear. Neutral = 0, reverse = -1.
    #[br(
        assert(
            (-1..=8).contains(&gear),
            "Car telemetry entry has an invalid gear value: {}",
            gear
        ),
    )]
    pub gear: i8,
    /// Engine RPM.
    pub engine_rpm: u16,
    /// Whether DRS is enabled.
    #[br(try_map(u8_to_bool))]
    pub drs_enabled: bool,
    /// Rev lights indicator (percentage).
    #[br(
        assert(
            rev_lights_percent <= 100,
            "Car telemetry entry has an invalid rev lights percentage: {}",
            rev_lights_percent
        ),
    )]
    pub rev_lights_percent: u8,
    /// Bitmap of active rev lights.
    #[br(map(RevLights::from_bits_retain))]
    pub rev_lights_bit_value: RevLights,
    /// Brakes' temperature values in degrees Celsius.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub brakes_temperature: [u16; 4],
    /// Tyres' surface temperature values in degrees Celsius.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub tyres_surface_temperature: [u8; 4],
    /// Tyres' inner temperature values in degrees Celsius.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub tyres_inner_temperature: [u8; 4],
    /// Engine's temperature in degrees Celsius
    pub engine_temperature: u16,
    /// Tyre pressure values in PSI.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub tyres_pressure: [f32; 4],
    /// Driving surface of each tyre.
    /// See [`wheel_index`](mod@crate::packets::f1::constants::wheel_index)
    /// for wheel order.
    pub surface_type: [Surface; 4],
}
