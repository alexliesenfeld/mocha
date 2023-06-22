# Server and Mock Lifecycle

`httpmock` utilizes a pool of mock servers in the background to optimize resource utilization (threads, ports, etc.).
`MockServer::start` automatically handles the pooling logic and cleanup of the pooled server instance prior to its usage.
When the corresponding mock server variable goes out of scope, the mock server is returned to the pool of available 
servers again.

When you create mocks, they will remain active until the associated mock server is recycled for use in 
another test function. As a result, the mocks you define continue to exist even after your test function completes. 
These mocks are only removed from the mock server instance when another test function is assigned the same instance.

This code is expected to run without any errors:
```rust
use httpmock::prelude::*;
use isahc::get as http_get;

#[test]
fn scope_test() {
    let url: String;

    // Create a mock server that will get out of scope after mock definition.
    // This will return the MockServer to the pool again.
    {
        let server = MockServer::start();
        let _ = server.mock(|when, then| {
            when.path("/test");
            then.status(200);
        });

        url = server.url("/test");
    }


    // Act
    let response = http_get(url).unwrap();

    // Assert
    assert_eq!(response.status(), 200);
}
```
