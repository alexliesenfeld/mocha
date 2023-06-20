#[test]
fn binary_body_test() {
    use httpmock::prelude::*;

    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path()

            .method(GET)
            .path("/translate")
            .query_param("word", "hello");
        then.status(200)
            .header("content-type", "text/html; charset=UTF-8")
            .body("Привет");
    });

    // Send an HTTP request to the mock server. This simulates your code.
    let response = isahc::get(server.url("/does-not-exist")).unwrap();

    // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
    hello_mock.assert();

    // Ensure the mock server did respond as specified.
    assert_eq!(response.status(), 200);
}
