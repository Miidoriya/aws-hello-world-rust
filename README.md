## Following the guide for the aws sdk for Rust:
https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html &&  
https://docs.aws.amazon.com/sdk-for-rust/latest/dg/getting-started.html  

_Further Reading_:
- Using profiles with AWS SDK: https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html
- tokio: https://crates.io/crates/tokio
- Rust based lambdas: https://github.com/awslabs/aws-lambda-rust-runtime#deployment
- Using the AWS SDK for Rust in AWS Lambda
- Cargo Lambda - https://www.cargo-lambda.info/guide/getting-started.html
```
Required on Mac

brew tap messense/macos-cross-toolchains
# install x86_64-unknown-linux-gnu toolchain
brew install x86_64-unknown-linux-musl

export CC_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-gcc
export CXX_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-g++
export AR_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-musl-gcc
```

Local development:
```
cargo lambda watch
cargo lambda invoke --data-ascii '{"body": "hi"}'
```