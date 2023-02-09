# viperserver

> Library for interacting with [viperserver](https://github.com/viperproject/viperserver). Has utilities spawning a server, and a client to connect and perform verifications.

## Generating the typed interface

This crate using code generation to create more ergonomic Rust interfaces to the CLI tools. It does so by calling `--help` on `viperserver`, `carbon`, and `silicon`, and parses the output. This happens in `src/tests/generate_cli.rs` and uses the `viperserver` submodule present in the crate root.

The result of generating these are committed into the repo, so you don't have to have anything but Rust installed to build!

If you want to generate these files again, the three projects are built from source, and thus [`sbt`](https://www.scala-sbt.org/) needs to be installed.

```bash
# Make sure the submodule is fetched
git submodule update --init --recursive

# Run the cargo test
cargo test generate_cli -- --ignored --nocapture
```

With this run, `src/opts/generated.rs` should contain options for the three CLI's!
