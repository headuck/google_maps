use crate::directions::{error::Error, request::Request};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

impl<'a> Request<'a> {

    /// Builds the query string for the Google Maps Directions API based on the
    /// input provided by the client.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn build(&'a mut self) -> Result<&'a mut Request, Error> {

        // Ensure request has been validated before building the query string:
        if !self.validated {
            return Err(Error::RequestNotValidated);
        }

        // Builds the "required parameters" portion of the query string:
        let mut query = format!(
            "key={}&origin={}&destination={}",
            self.client_settings.key,
            String::from(&self.origin), // URL-encoding performed by From trait
            String::from(&self.destination), // URL-encoding performed by From trait
        ); // format!

        // Builds the "optional parameters" portion of the query string:

        // Alternatives key/value pair:
        if let Some(alternatives) = &self.alternatives {
            query.push_str("&alternatives=");
            query.push_str(&alternatives.to_string())
        } // if

        // Arrival time key/value pair:
        if let Some(arrival_time) = &self.arrival_time {
            query.push_str(&format!("&arrival_time={}", arrival_time.timestamp()));
        } // if

        // Avoid key/value pair:
        if let Some(restrictions) = &self.restrictions {
            query.push_str("&avoid=");
            query.push_str(
                &*utf8_percent_encode(
                    &String::from(
                        restrictions
                            .iter()
                            .map(|avoid| String::from(avoid) + "|")
                            .collect::<String>()
                            .trim_end_matches('|'),
                    ),
                    NON_ALPHANUMERIC,
                )
                .to_string(),
            )
        } // if

        // Departure time key/value pair:
        if let Some(departure_time) = &self.departure_time {
            query.push_str("&departure_time=");
            query.push_str(&String::from(departure_time))
        } // if

        // Language key/value pair:
        if let Some(language) = &self.language {
            query.push_str("&language=");
            query.push_str(&String::from(language))
        } // if

        // Travel mode key/value pair:
        if let Some(travel_mode) = &self.travel_mode {
            query.push_str("&mode=");
            query.push_str(&String::from(travel_mode).to_lowercase())
        } // if

        // Region key/value pair:
        if let Some(region) = &self.region {
            query.push_str("&region=");
            query.push_str(&String::from(region))
        } // if

        // Traffic model key/value pair:
        if let Some(traffic_model) = &self.traffic_model {
            query.push_str("&traffic_model=");
            query.push_str(&String::from(traffic_model))
        } // if

        // Transit mode key/value pair:
        if let Some(transit_modes) = &self.transit_modes {
            query.push_str("&transit_mode=");
            query.push_str(
                &*utf8_percent_encode(
                    &String::from(
                        transit_modes
                            .iter()
                            .map(|mode| String::from(mode) + "|")
                            .collect::<String>()
                            .trim_end_matches('|'),
                    ),
                    NON_ALPHANUMERIC,
                )
                .to_string(),
            )
        } // if

        // Transit route preference key/value pair:
        if let Some(transit_route_preference) = &self.transit_route_preference {
            query.push_str("&transit_routing_preference=");
            query.push_str(&String::from(transit_route_preference))
        } // if

        // Unit system key/value pair:
        if let Some(unit_system) = &self.unit_system {
            query.push_str("&units=");
            query.push_str(&String::from(unit_system))
        } // if

        // Waypoints key/value pair:
        if let Some(waypoints) = &self.waypoints {
            query.push_str("&waypoints=");
            if self.waypoint_optimization {
                query.push_str("optimize:true|");
            } // if
            query.push_str(
                &*utf8_percent_encode(
                    &String::from(
                        waypoints
                            .iter()
                            .map(|waypoint| String::from(waypoint) + "|")
                            .collect::<String>()
                            .trim_end_matches('|'),
                    ),
                    NON_ALPHANUMERIC,
                )
                .to_string(),
            )
        } // if

        // Set query string in Request struct.
        self.query = Some(query);

        // Return modified Request struct to caller.
        Ok(self)
    } // fn

} // impl