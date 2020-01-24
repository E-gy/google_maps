use crate::{
    language::Language,
    time_zone::request::Request,
}; // use

impl Request {

    /// Adds the language parameter to the Time Zone API query.
    ///
    /// ## Arguments:
    ///
    /// * `language` ‧ The language that Google's response should be presented
    /// in.
    ///
    /// ## Example:
    ///
    /// * Set Google's response to the French language:
    /// ```
    /// .with_language(Language::French)
    /// ```

    pub fn with_language(&mut self, language: Language) -> &mut Request {
        // Set language in Request struct.
        self.language = Some(language);
        // Return modified Request struct to caller.
        self
    } // fn

} // impl