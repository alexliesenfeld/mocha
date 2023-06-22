# Matching Requests

Request matchers in `httpmock` provide a powerful mechanism to define specific criteria for matching incoming HTTP requests. By using request matchers, you can precisely control how your mock server responds to different requests based on various attributes such as HTTP method, path, headers, query parameters, and request body.

To apply request matchers, you typically use them in conjunction with the `when` variable when defining a mock.
The when variable is of type `When` and provides builder-like semantics to construct conditions that incoming requests
should meet to trigger a specific response. For example:
```rust
server.mock(|when, then| {
    when.method(GET).path("/users");
    then.status(200).body("[{\"name\": \"John\"}]");
});
```

In the above example, the mock is defined to match `GET` requests with the path `/users` and respond with a `200`
status code and a JSON array of user objects.

Request matchers provide a flexible and expressive way to define the behavior of your mock server based on specific request attributes. By utilizing these matchers effectively, you can create accurate and targeted responses, ensuring thorough testing and simulation of various request scenarios in your applications.

# Built-in Matchers

`httpmock` comes equipped with builder-like `When` structure with methods that simplify the process of defining precise criteria for matching incoming requests. Let's explore some built-in matchers:

- **Method Matcher**: The method matcher allows you to specify the HTTP method (GET, POST, PUT, DELETE, etc.) that a request should have in order to be considered a match.
- **Path Matcher**: The path matcher enables you to define the URL path that a request should match. It supports exact path matching as well as pattern-based matching. Some example path matchers are `path`, `path_contains`, and `path_matches`.
- **Header Matcher**: The header matcher allows you to verify specific headers in the incoming requests. You can check for the presence of a header, validate its exact value, or even match it against a regular expression. Some example header matchers are `header`, `header_exists`, `cookie`, and `cookie_exists`.
- **Query Parameter Matcher**: The query parameter matcher enables you to validate query parameters present in the request URL. You can ensure the presence of specific parameters, validate their values, or match them against regular expressions. With this matcher, you can accurately simulate different query parameter combinations and test the behavior of your application accordingly. Some example query parameter matchers are `query_param`, and `query_param_exists`.
- **Body Matcher**: The body matcher allows you to validate the content of the request body. You can check for an exact match, a partial match, or use regular expressions to match against the body content. This matcher enables you to simulate responses based on different request body payloads, facilitating comprehensive testing of your application's handling of request data. Some request body matchers are `body`, `body_contains`, `body_matches`, `json_body`, and more.

These built-in matchers form the backbone of 'httpmock''s request matching capabilities, providing you with a versatile toolkit to create precise and targeted mocks for your testing needs. By leveraging these matchers effectively, you can ensure accurate simulations of various request scenarios and thoroughly validate the behavior of your HTTP-dependent applications.


# Custom Matchers
`httpmock` provides the flexibility to define custom request matchers, allowing you to create specialized matching 
criteria tailored to your specific testing requirements. Custom matchers enable you to extend the capabilities of
`httpmock` beyond the built-in matchers, providing even finer control over request matching. Let's see how you can 
define and use a custom request matcher:

```rust
use httpmock::prelude::*;
use isahc::get;

#[test]
fn my_custom_request_matcher_test() {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.matches(|req| req.path.to_lowercase().ends_with("test"));
        then.status(200);
    });

    // Act: Send the HTTP request
    let response = get(server.url("/thisIsMyTest")).unwrap();

    // Assert
    mock.assert();
    assert_eq!(response.status(), 200);
}
```

In this example, we define a test case that utilizes a custom request matcher. Within the mock closure, we use 
the matches method to define a custom matching criterion. In this case, the matcher checks if the request path ends 
with the string "test" (case-insensitive). If the request matches the defined criterion, a response with a status 
code of `200` is returned.

By implementing custom matchers, you can precisely define the conditions under which a mock should be triggered, 
enabling more complex and specialized testing scenarios. Custom matchers provide flexibility and control, allowing 
you to create mocks that closely mimic real-world request matching behavior.

With `httpmock`'s support for custom request matchers, you can extend the capabilities of the library and tailor the 
request matching to suit your specific testing needs, ensuring accurate and thorough testing of your 
HTTP-dependent applications.