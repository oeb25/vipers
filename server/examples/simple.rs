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
