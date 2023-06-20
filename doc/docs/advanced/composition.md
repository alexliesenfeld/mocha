    /// A semantic convenience for applying multiple operations
    /// on a given `When` instance via an discrete encapsulating
    /// function.
    ///
    /// ## Example:
    ///
    /// ```
    /// # use httpmock::When;
    /// // Assuming an encapsulating function like:
    ///
    /// fn is_authorized_json_post_request(when: When) -> When {
    ///     when.method(httpmock::Method::POST)
    ///         .header("authorization", "SOME API KEY")
    ///         .header("content-type", "application/json")
    /// }
    ///
    /// // It can be applied without breaking the usual `When`
    /// // semantic style. Meaning instead of:
    /// #
    /// # fn counter_example(when: When) -> When {
    /// is_authorized_json_post_request(when.json_body_partial(r#"{"key": "value"}"#))
    /// # }
    ///
    /// // the `and` method can be used to preserve the
    /// // legibility of the method chain:
    /// # fn semantic_example(when: When) -> When {
    ///  when.query_param("some-param", "some-value")
    ///      .and(is_authorized_json_post_request)
    ///      .json_body_partial(r#"{"key": "value"}"#)
    /// # }
    ///
    /// // is still intuitively legible as "when some query
    /// // parameter equals "some-value", the request is an
    /// // authorized POST request, and the request body
    /// // is the literal JSON object `{"key": "value"}`.
    ///