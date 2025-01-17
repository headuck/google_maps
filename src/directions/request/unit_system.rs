//! Contains the `UnitSystem` enum and its associated traits. It is used specify
//! whether imperial or metric units are used in Directions responses.

use crate::directions::error::Error;
use serde::{Deserialize, Serialize};

/// Specifies the [unit
/// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
/// to use when displaying results.
///
/// Directions results contain `text` within `distance` fields that may be
/// displayed to the user to indicate the distance of a particular "step" of the
/// route. By default, this text uses the unit system of the origin's country or
/// region.
///
/// For example, a route from "Chicago, IL" to "Toronto, ONT" will display
/// results in miles, while the reverse route will display results in
/// kilometers. You may override this unit system by setting one explicitly
/// within the request's `units` parameter, passing one of the following values:
///
/// **Note**: this unit system setting only affects the `text` displayed within
/// `distance` fields. The `distance` fields also contain `values` which are
/// always expressed in meters.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum UnitSystem {
    /// Specifies that distances in the response should be expressed in imperial
    /// units, miles and feet.
    #[serde(alias = "imperial")]
    Imperial,

    /// Specifies that distances in the response should be expressed in metric
    /// units, using kilometres and metres.
    #[serde(alias = "metric")]
    Metric,
} // enum

impl std::convert::From<&UnitSystem> for String {
    /// Converts a `UnitSystem` enum to a `String` that contains a [unit
    /// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
    /// code.
    fn from(units: &UnitSystem) -> String {
        match units {
            UnitSystem::Imperial => String::from("imperial"),
            UnitSystem::Metric => String::from("metric"),
        } // match
    } // fn
} // impl

impl std::convert::TryFrom<&str> for UnitSystem {
    // Error definitions are contained in the
    // `google_maps\src\directions\error.rs` module.
    type Error = crate::directions::error::Error;
    /// Gets a `UnitSystem` enum from a `String` that contains a valid [unit
    /// system](https://developers.google.com/maps/documentation/directions/intro#UnitSystems)
    /// code.
    fn try_from(units: &str) -> Result<UnitSystem, Error> {
        match units {
            "imperial" => Ok(UnitSystem::Imperial),
            "metric" => Ok(UnitSystem::Metric),
            _ => Err(Error::InvalidUnitSystemCode(units.to_string())),
        } // match
    } // fn
} // impl

impl std::default::Default for UnitSystem {
    /// Returns a reasonable default variant for the `UnitSystem` enum type.
    fn default() -> Self {
        UnitSystem::Metric
    } // fn
} // impl

impl std::fmt::Display for UnitSystem {
    /// Formats a `UnitSystem` enum into a string that is presentable to the
    /// end user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnitSystem::Imperial => write!(f, "Imperial"),
            UnitSystem::Metric => write!(f, "Metric"),
        } // match
    } // fn
} // impl