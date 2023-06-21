---
sidebar_position: 1
---

# Getting Started
httpmock is a Rust HTTP mocking library that allows you to simulate responses from HTTP servers, making it ideal for testing and integration purposes. This section will guide you through the process of getting started with httpmock and demonstrate its key features.

## Installation
To use httpmock in your Rust project, you need to add it as a dependency in your Cargo.toml file:

```toml
[dev-dependencies]
httpmock = "0.6"
```

## Example Usage
Let's walk through a simple example to demonstrate how to use httpmock to mock HTTP services.

```rust
use httpmock::prelude::*;

// Start a lightweight mock server.
let server = MockServer::start();

// Create a mock on the server.
let hello_mock = server.mock(|when, then| {
    when.method(GET)
        .path("/translate")
        .query_param("word", "hello");
    then.status(200)
        .header("content-type", "text/html; charset=UTF-8")
        .body("Привет");
});

// Send an HTTP request to the mock server. This simulates your code.
let response = isahc::get(server.url("/translate?word=hello")).unwrap();

// Ensure the specified mock was called exactly one time 
// or fail with a detailed error description.
hello_mock.assert();

// Ensure the mock server did respond as specified.
assert_eq!(response.status(), 200);
```

In this example, we create a MockServer that acts as our mock HTTP server. We then define a mock using the mock method, specifying the expected request and the desired response. Finally, we make a request to the mock server and verify the response and the expected request using assertions and the assert method.

This is just a basic example to demonstrate the usage of httpmock. You can configure more complex request and response scenarios, use custom matchers, and even run a standalone mock server for end-to-end testing.