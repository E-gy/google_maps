use crate::directions::request::Request;

impl Request {

    /// Specify whether service may provide more than one route alternative in
    /// the response.
    ///
    /// ## Arguments:
    ///
    /// * `alternatives` ‧ Whether more than one route should be in Google's
    /// response.
    ///
    /// ## Description
    ///
    /// If set to `true`, specifies that the Directions service may provide more
    /// than one route alternative in the response. Note that providing route
    /// alternatives may increase the response time from the server. This is
    /// only available for requests without intermediate waypoints.
    ///
    /// ## Example:
    ///
    /// * Allow more than one route in the response:
    /// ```
    /// .with_alternatives(true)
    /// ```

    pub fn with_alternatives(&mut self, alternatives: bool) -> &mut Request {
        self.alternatives = Some(alternatives);
        self
    } // fn

} // impl