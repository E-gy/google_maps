use crate::Country;
use crate::places::place_autocomplete::request::Request;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the components parameter to the Place API _Place Autocomplete_
    /// query.
    ///
    /// ## Arguments:
    ///
    /// * `component` ‧ A grouping of places to which you would like to restrict
    /// your results. Currently, you can use components to filter by up to 5
    /// countries.
    ///
    /// * Multiple components may be stacked together.

    pub fn with_component(&'a mut self, component: Country) -> &'a mut Request {
        // Set components in Request struct.
        self.components.extend(vec![component]);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {

    /// Adds the components parameter to the Place API _Place Autocomplete_
    /// query.
    ///
    /// ## Arguments:
    ///
    /// * `components` ‧ A grouping of places to which you would like to restrict
    /// your results. Currently, you can use components to filter by up to 5
    /// countries.
    ///
    /// * Multiple components may be stacked together.

    pub fn with_components(&'a mut self, components: Vec<Country>) -> &'a mut Request {
        // Set components in Request struct.
        self.components.extend(components);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl