# Defining Responses

httpmock allows you to define custom responses when creating mocks, giving you the flexibility to create tailored and
dynamic responses based on specific request criteria. Let's see how you can define custom responses in mock creation:

```rust
use httpmock::prelude::*;
use isahc::get;

#[test]
fn my_custom_response_definition_test() {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/data")
            .header("Content-Type", "application/json");
        then.status(200)
            .header("Content-Type", "text/plain")
            .body("Custom Response");
    });

    // Act: Send the HTTP request
    let response = get(server.url("/api/data")).unwrap();

    // Assert
    mock.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(response.headers()["Content-Type"], "text/plain");
    assert_eq!(response.text().unwrap(), "Custom Response");
}
```

In this example, we define a test case that utilizes custom response definition. Within the mock closure, we use the when object to set specific criteria for the request, such as the HTTP method, path, and headers. If the incoming request matches the defined criteria, the mock will respond with a custom response. In this case, the response has a status code of 200, a "Content-Type" header of "text/plain," and a body containing the text "Custom Response".

By defining custom responses, you can create unique and tailored mock behavior based on specific request attributes. This allows you to simulate different scenarios and test your application's handling of various responses. Custom response definition enhances your testing capabilities, enabling you to thoroughly validate your HTTP-dependent applications.

With httpmock's support for custom response definition, you have the flexibility to create mocks that closely resemble the behavior of real APIs, empowering you to conduct comprehensive testing and ensure the reliability of your applications.