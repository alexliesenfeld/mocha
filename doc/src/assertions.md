# Using Assertions
`httpmock` provides detailed error messages to help users understand why their expected requests did not match the 
actual requests received by the mock server. These error messages offer valuable insights for debugging and resolving 
issues in your tests. Let's take a closer look at the structure and content of the error messages.

Let's look at the code example from the introduction, but this time let's modify it to demonstrate a scenario where 
our HTTP client sends a request that doesn't match the specified mock.

```rust,noplayground
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

// Send an HTTP request to the mock server for which there is no mock definition.
let response = isahc::get(server.url("/does-no-exist")).unwrap();

// Ensure the specified mock was called exactly one time (or fail with a detailed error description).
hello_mock.assert();

// Ensure the mock server did respond as specified.
assert_eq!(response.status(), 200);
```

Because our request has URL path `/does-not-exist` rather than `/translate?word=hello`, the `hello_mock.assert()` method
raises an error during test execution with the following information:

```
0 of 1 expected requests matched the mock specification.
Here is a comparison with the most similar non-matching request (request number 1):

1: The path does not match
------------------------------------------------------------------------------------------
Expected:   [equals]        /translate
Actual:                     /does-not-exist

2: Expected query parameter with name 'word' and value 'hello' to be present in the request but it wasn't.
------------------------------------------------------------------------------------------

Left:  /translate
Right: /does-not-exist
<Click to see difference>
```

