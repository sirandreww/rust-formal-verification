# rust-formal-verification

A rust library that makes it easier to develop, prototype and test new algorithms for formal verification like IC3, PDR, AVY and others.

# Publishing a new version

To publish a new version of the library :
1. run `cargo fmt --check` (you may run `cargo fmt` to fix changes quickly)
2. run `cargo clippy` (you may run `cargo clippy --fix` to fix changes quickly)
3. run `cargo test`