use std::{path::Path, process::Stdio};

use derive_more::Display;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

use crate::error::{Result, ViperServerError};
use crate::opts::{ViperServerOpts, ViperServerOptsBuilder};

#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ViperServerLogLevel {
    #[display(fmt = "ALL")]
    All,
    #[display(fmt = "TRACE")]
    Trace,
    #[display(fmt = "DEBUG")]
    Debug,
    #[display(fmt = "INFO")]
    Info,
    #[display(fmt = "WARN")]
    Warn,
    #[display(fmt = "ERROR")]
    Error,
    #[display(fmt = "OFF")]
    Off,
}

#[derive(
    Debug,
    Display,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ServerMode {
    #[display(fmt = "LSP")]
    Lsp,
    #[display(fmt = "HTTP")]
    #[default]
    Http,
}

#[derive(Debug)]
enum OnlineAt {
    Waiting(tokio::sync::oneshot::Receiver<String>),
    Got(String),
    Errored,
}

#[derive(Debug)]
pub struct ViperServer {
    #[allow(unused)]
    child: Child,
    online_at: OnlineAt,
    pub stdout: tokio::sync::mpsc::UnboundedReceiver<String>,
    pub stderr: tokio::sync::mpsc::UnboundedReceiver<String>,
}

impl ViperServerOptsBuilder {
    async fn spawn(&self, viper_server_jar: impl AsRef<Path>) -> Result<ViperServer> {
        ViperServer::spawn(viper_server_jar, self.build()?).await
    }
    pub async fn spawn_http(&mut self, viper_server_jar: impl AsRef<Path>) -> Result<ViperServer> {
        self.server_mode("HTTP").spawn(viper_server_jar).await
    }
    pub async fn spawn_lsp(&mut self, viper_server_jar: impl AsRef<Path>) -> Result<ViperServer> {
        self.server_mode("LSP").spawn(viper_server_jar).await
    }
}

impl ViperServer {
    pub fn builder() -> ViperServerOptsBuilder {
        ViperServerOptsBuilder::default()
    }

    pub async fn spawn(viper_server_jar: impl AsRef<Path>, opts: ViperServerOpts) -> Result<Self> {
        let viper_server_jar = viper_server_jar.as_ref();
        let mut cmd = Command::new("java");
        cmd.arg("-Xss128m")
            .arg("-Xmx4g")
            .arg("-jar")
            .arg(viper_server_jar);

        opts.apply(|a| {
            cmd.arg(a);
        });

        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let mut child = cmd
            .spawn()
            .map_err(|source| ViperServerError::SpawnServer {
                source,
                viper_server_jar: viper_server_jar.display().to_string(),
            })?;

        let (stdout_tx, stdout_rx) = tokio::sync::mpsc::unbounded_channel();
        let (stderr_tx, stderr_rx) = tokio::sync::mpsc::unbounded_channel();

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let (online_at_tx, online_at_rx) = tokio::sync::oneshot::channel::<String>();

        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();

            let mut online_at_tx = Some(online_at_tx);

            while let Some(line) = lines.next_line().await.expect("failed to read line") {
                if online_at_tx.is_some() {
                    if let Some((_, url)) = line.split_once("ViperServer online at ") {
                        online_at_tx.take().unwrap().send(url.to_string()).unwrap();
                    }
                }

                stdout_tx
                    .send(line)
                    .expect("failed to send. did receiver close?");
            }
        });

        tokio::spawn(async move {
            let mut lines = BufReader::new(stderr).lines();

            while let Some(line) = lines.next_line().await.expect("failed to read line") {
                stderr_tx
                    .send(line)
                    .expect("failed to send. did receiver close?");
            }
        });

        Ok(Self {
            child,
            online_at: OnlineAt::Waiting(online_at_rx),
            stdout: stdout_rx,
            stderr: stderr_rx,
        })
    }

    pub async fn online_at(&mut self) -> Result<String> {
        match &self.online_at {
            OnlineAt::Waiting(_) => {}
            OnlineAt::Got(url) => return Ok(url.clone()),
            OnlineAt::Errored => todo!(),
        }
        if let OnlineAt::Waiting(rx) = std::mem::replace(&mut self.online_at, OnlineAt::Errored) {
            let url = rx
                .await
                .map_err(|source| ViperServerError::ServerDidNotReportPort { source })?;
            self.online_at = OnlineAt::Got(url.clone());
            Ok(url)
        } else {
            unreachable!()
        }
    }
}
