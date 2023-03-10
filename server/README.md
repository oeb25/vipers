# viperserver

> Library for interacting with [viperserver](https://github.com/viperproject/viperserver). Has utilities spawning a server, and a client to connect and perform verifications.

```rs
use futures::StreamExt;
use viperserver::{
    client::{Client, VerificationRequest, VerificationStatus},
    server::ViperServer,
};

async fn run() -> color_eyre::Result<()> {
    // Launch the server. This should point to your local `viperserver.jar`
    let server = ViperServer::builder().spawn_http("viperserver.jar").await?;

    // Attach the client. This keeps the server alive for as long as the client is alive
    let client = Client::new(server).await?;

    // Build a new request
    let request = VerificationRequest::silicon()
        .timeout("10")
        .detect_z3()?
        .verify_file("hello.vpr")?;
    // Send the request, and receive a verification response, which contains the
    // id of the current verification
    let response = client.post(request).await?;

    // The server streams the results of verification over a long-running HTTP
    // request
    let mut stream = client.check_on_verification(&response).await?;

    // We can read the individual lines to get progress reports!
    while let Some(status) = stream.next().await {
        let status = status?;
        match &status {
            VerificationStatus::VerificationResult { .. } => {
                println!("Verification result: {status:?}")
            }
            VerificationStatus::ExceptionReport { .. } => {
                println!("Exception hit: {status:?}")
            }
            _ => println!("Status: {status:?}"),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // Optional, but makes errors look a little nicer :)
    color_eyre::install()?;

    run().await?;

    Ok(())
}
```

## Generating the typed interface

This crate using code generation to create more ergonomic Rust interfaces to the CLI tools. It does so by calling `--help` on `viperserver`, `carbon`, and `silicon`, and parses the output. This happens in `src/tests/generate_cli.rs` and uses the `viperserver` submodule present in the crate root.

The result of generating these are committed into the repo, so you don't have to have anything but Rust installed to build!

If you want to generate these files again, the three projects are built from source, and thus [`sbt`](https://www.scala-sbt.org/) needs to be installed.

```bash
# Make sure the submodule is fetched
git submodule update --init --recursive

# The generation code is marked with #[test] and is run with
cargo test generate_cli -- --ignored --nocapture
```

With this run, `src/opts/generated.rs` should contain options for the three CLI's!
