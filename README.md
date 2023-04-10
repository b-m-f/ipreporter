# ipreporter

Simple web server in Rust which will return the callers IP address and runs on port 80.

## Configuration

If desired the server can be started with HTTPS.

Set the environment variables `CERT_PATH` and `KEY_PATH` that point to the correct files necessary to set up TLS.

## Logging
`pretty_env_logger` is being used. Check out the docs on how to enable logging [here](https://docs.rs/pretty_env_logger/0.4.0/pretty_env_logger/#enable-logging).
