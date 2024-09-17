# Axelar GMP

A simple library for using the [Axelar GMP REST API](). Right now only the `searchGMP` method is implemented, and only a subset of the available fields are deserialized in the `GMPTransaction` type contained in the response. Feel free to submit a PR to make it more exhaustive.

## Usage

The library uses the builder pattern to construct a request and send it to the GMP API:

```rust
let result = SearchGMPRequestBuilder::default()
    // any number of fields can be set
    .from_time(1715400000)
    .to_time(1715462400)
    // # of results in the page
    .size(100)
    // constructs the request type
    .build()
    // call the GMP API with the request
    .send()
    .await;

println!("{result:?}");
```
