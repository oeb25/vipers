use std::{fs, path::PathBuf, process::Command};

use crate::{client, server, verification::VerificationStatus};

mod generate_cli;

#[tokio::test]
async fn basic_test() -> color_eyre::Result<()> {
    use futures::StreamExt;

    color_eyre::install()?;

    #[cfg(feature = "bundle-viperserver")]
    let viperserver = crate::bundled::ViperServerJar::create()?;
    #[cfg(not(feature = "bundle-viperserver"))]
    let viperserver = {
        let viperserver_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/viperserver/"));
        let target_path = fs::read_dir(viperserver_path.join("target/"))?
            .map(|p| p.unwrap())
            .find(|p| p.file_name().to_str().unwrap().starts_with("scala-"))
            .unwrap()
            .path();
        target_path.join("viperserver.jar")
    };

    eprintln!("Using viperserver at {viperserver:?}");

    let s = server::ViperServer::builder()
        .log_file("test-viper.log")
        .cache_file("test-viper.cache")
        .log_level("ALL")
        .spawn_http(&viperserver)
        .await?;

    let mut client = client::Client::new(s).await?;

    let res = client
        .post(
            client::VerificationRequest::silicon()
                .detect_z3()?
                .verify_file(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/src/tests/sample-programs/fib_rec.vpr"
                ))?,
        )
        .await?;

    let mut stream = client.check_on_verification(&res).await?;

    while let Some(r) = stream.next().await {
        let r = r?;
        if let VerificationStatus::ExceptionReport { .. } = &r {
            dbg!(r);
        }
    }

    while let Ok(r) = client.server.stdout.try_recv() {
        eprintln!("stdout: {r:?}");
    }
    while let Ok(r) = client.server.stderr.try_recv() {
        eprintln!("stderr: {r:?}");
    }

    Ok(())
}
