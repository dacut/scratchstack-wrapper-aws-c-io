# scratchstack-wrapper-aws-c-io
Rust wrappers for the [`aws-c-io`](https://github.com/awslabs/aws-c-io) I/O library.

Currently, this only provided the minimum set of bindings necessary for Scratchstack to verify interoperability
with AWS CRT authentication libraries.  This is not intended to be a complete set of bindings for the CRT. You
probably *do not want to use these bindings* in your own projects. If you need to communicate with AWS services,
use the [official AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust) instead.

Documentation: https://docs.rs/scratchstack-wrapper-aws-c-io

