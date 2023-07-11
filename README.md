[![Actions Status](https://github.com/aws-cli-tools/aws-cli-lib/workflows/Code%20Gating/badge.svg?branch=main)](https://github.com/aws-cli-tools/aws-cli-lib/workflows/Code%20Gating/badge.svg?branch=main)

# aws-cli-lib
This Rust AWS library is a shared tool that facilitates the development of various CLI applications. It abstracts and encapsulates core functionalities that interact with AWS services, offering a consistent and reliable means of AWS resource management across different projects.


## Usage
To add using cargo use:
```bash
cargo add --git "https://github.com/aws-cli-tools/aws-cli-lib.git"
```
And in code:
```rust
use aws_cli_lib::get_region_provider
```

## Running locally
* You can always use `cargo` to manage the build and tests.
* We use [`just`](https://github.com/casey/just) as a command running.
* Use `just gate` to run all checks locally.

## Contributing
See our [CONTRIBUTION](CONTRIBUTION.md) page
