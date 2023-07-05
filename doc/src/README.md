# Introduction

`httpmock` offers a simple solution for simulating HTTP requests and responses during development and testing. 
It empowers you to replicate the behavior of real HTTP servers in controlled environments, enabling thorough testing 
of client applications and integration scenarios. 

Within this user guide, you will find detailed explanations of `httpmock`'s concepts and features. Topics covered 
include configuring mock servers in your tests, defining routes and request matchers, generating custom responses, 
and verifying requests. 

Please refer to the API reference documentation for detailed documentation how to interact with the API. 

## Why mocking HTTP servers?
There are several compelling reasons why you would want to utilize an HTTP mocking library:

- **Isolation and Control**: By using an HTTP mocking library, you can isolate the specific functionality or module you are testing from external dependencies, such as third-party APIs or services. Mocking allows you to control the behavior and responses of the external services, ensuring predictable and consistent test scenarios.

- **Test Coverage and Edge Cases**: With an HTTP mocking library, you can easily simulate different scenarios, error conditions, and edge cases that might be challenging to replicate with real services. This enables thorough test coverage, ensuring your code handles a wide range of situations effectively.

- **Independent Testing**: Mocking enables you to test your application components independently, without relying on the availability or stability of external services. This promotes parallel testing and allows different teams or developers to work in isolation, speeding up development and reducing dependencies.

- **Controlled Data**: HTTP mocking libraries provide the ability to define specific responses, payloads, or error conditions for different requests. This allows you to test your application's behavior under various data scenarios, ensuring robustness and handling of different responses from external services.

- **Debugging and Error Handling**: With an HTTP mocking library, you can capture and analyze the requests and responses exchanged between your application and the mock server. This facilitates effective debugging and error handling by providing detailed insights into how your application interacts with external services.

- **Continuous Integration and Deployment**: HTTP mocking libraries are invaluable in continuous integration and deployment pipelines. By mocking external dependencies, you can run tests without the need for live services, ensuring faster feedback cycles and smoother integration with automated build and deployment processes.

## Design Goals

### Simple API
The primary design objective of this library is to minimize the cognitive load associated with creating mocks. 
The intention is to make the process intuitive and straightforward, requiring minimal knowledge and effort. 
This is achieved by leveraging the methods provided by the `When` and `Then` variables, as exemplified in the 
`server.mock(|when, then| { ... })` syntax.

By utilizing your IDE, you can easily explore the available functionality, minimizing the need to 
refer to online documentation. The library aims to provide a user-friendly experience where you 
are mostly guided by your IDE. There is no requirement to learn and provide 
specific matching functions as parameters, nor are any complex syntactical maneuvers necessary. The library 
strives for simplicity and transparency, aiming to be unassuming and easily understandable.

### Easy Debugging
One of the key design goals of `httpmock` is to provide helpful information when requests do not match the expected mocks, 
which is a common scenario during test execution.

In such cases, `httpmock` generates detailed error messages that offer valuable insights into why the expected requests 
did not match the actual requests received by the mock server. These error messages play a crucial role in debugging 
and resolving issues within your tests.

### Versatile and Useful

The `httpmock` library goes beyond its application in automated tests, offering standalone utility with the following features:

- **Built-in Matchers and Response Creation**: `httpmock` provides a wide range of built-in matchers and functions for creating HTTP responses. These matchers allow precise criteria definition, including methods, paths, headers, query parameters, and request bodies, while enabling flexible response construction.

- **Extensible Custom Matchers**: You can extend `httpmock`'s matching capabilities by implementing custom matchers. This empowers you to define specific criteria and patterns tailored to your application's behavior or testing requirements.

- **Real HTTP Mock Server**: `httpmock` provides a genuine mock server implementation for realistic HTTP communication. It tries to optimize resource usage (such as threads and ports), but prioritizes accurate simulation of real-world HTTP behavior.

Furthermore, `httpmock` proves a **standalone mode** that allows you to run a mock server outside of the context of a 
test function and to receive HTTP requests from various applications or services, not just the test functions where 
a mock server is typically created.
