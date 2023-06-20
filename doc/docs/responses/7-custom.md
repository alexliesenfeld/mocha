---
sidebar_position: 9
---

# Custom Matchers

With `httpmock`, you have the flexibility to define your own custom request matchers using closures. 
These matchers are functions that return `true` if a request matches the desired criteria or constraints you specify 
and `false` otherwise. `httpmock` seamlessly integrates these custom matchers into its request matching process, 
allowing you to create more specific and tailored request expectations. You can specify a custom matcher using the
`Mock::matches` method.

```rust
use httpmock::prelude::*;
use isahc::get;

#[test]
fn custom_request_matcher_test() {
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

Please note that custom request matchers  **cannot be used
in standalone mode.**