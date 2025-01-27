use crate::{
    client_settings::ClientSettings,
    geocoding::reverse::ReverseRequest,
    latlng::LatLng
}; // use crate

impl<'a> ReverseRequest<'a> {

    /// Initializes the builder pattern for a Geolocation API query with the
    /// required, non-optional parameters.
    ///
    /// # Arguments:
    ///
    /// * `key` - Your application's Google Cloud API key.
    ///
    /// * `latlng` - The latitude and longitude values specifying the location
    /// for which you wish to obtain the closest, human-readable address.

    pub fn new(
        client_settings: &'a ClientSettings,
        latlng: LatLng
    ) -> ReverseRequest<'a> {
        // Instantiate struct and return it to caller:
        ReverseRequest {
            // Required parameters:
            client_settings,
            latlng,
            // Optional parameters:
            language: None,
            location_types: None,
            result_types: None,
            // Internal use only:
            query: None,
        } // struct
    } // fn

} // impl