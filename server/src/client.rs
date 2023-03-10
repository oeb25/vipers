use std::path::{Path, PathBuf};

use futures::{Stream, TryStreamExt};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncBufReadExt;
use tokio_stream::{wrappers::LinesStream, StreamExt};
use tokio_util::io::StreamReader;

use crate::{
    error::Result,
    error::ViperServerError,
    opts::{CarbonOpts, CarbonOptsBuilder, SiliconOpts, SiliconOptsBuilder},
    server::ViperServer,
};

pub use crate::verification::VerificationStatus;

#[derive(Debug)]
pub struct Client {
    #[allow(unused)]
    pub server: ViperServer,
    base: Url,
}

impl Client {
    pub async fn new(mut server: ViperServer) -> Result<Client> {
        match server.online_at().await {
            Ok(base) => Ok(Self {
                server,
                base: Url::parse(&base)?,
            }),
            Err(err) => {
                let mut stdout = "stdout:".to_string();
                let mut stderr = "stderr:".to_string();

                while let Ok(r) = server.stdout.try_recv() {
                    stdout += "\n";
                    stdout += &r;
                }
                while let Ok(r) = server.stderr.try_recv() {
                    stdout += "\n";
                    stderr += &r;
                }

                Err(ViperServerError::ConnectToServerFailed {
                    stdout,
                    stderr,
                    source: Box::new(err),
                })

                // Err(err).wrap_err(stdout).wrap_err(stderr)?
            }
        }
    }

    async fn get_lines_streaming<Res: for<'a> Deserialize<'a>>(
        &self,
        url: impl AsRef<str>,
    ) -> Result<impl Stream<Item = Result<Res>>> {
        let client = reqwest::Client::new();
        let res = client.get(self.base.join(url.as_ref())?).send().await?;

        let reader = StreamReader::new(
            res.bytes_stream()
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)),
        );

        let lines = LinesStream::new(reader.lines());

        Ok(lines.map(|l| {
            let l = l?;
            serde_json::from_str(&l).map_err(|source| ViperServerError::ParseJson {
                json: l.to_string(),
                source,
            })
        }))
    }

    pub async fn check_on_verification(
        &self,
        v: &VerificationResponse,
    ) -> Result<impl Stream<Item = Result<VerificationStatus>>> {
        self.get_lines_streaming(format!("verify/{}", v.id)).await
    }

    pub async fn post<R: ViperRequest>(&self, body: R) -> Result<R::Response> {
        let client = reqwest::Client::new();
        let res = client
            .post(self.base.join(body.url())?)
            .json(&serde_json::json!({
                "type": body.ty(),
                "arg": body,
            }))
            .send()
            .await?;

        let s = res.text().await?;
        serde_json::from_str(&s).map_err(|source| ViperServerError::ParseJson {
            json: s.to_string(),
            source,
        })
    }
}

pub trait ViperRequest: Serialize {
    type Response: for<'a> Deserialize<'a>;
    fn url(&self) -> &'static str;
    fn ty(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub enum VerificationRequest {
    Carbon {
        opts: Box<CarbonOpts>,
        file: PathBuf,
    },
    Silicon {
        opts: Box<SiliconOpts>,
        file: PathBuf,
    },
}

impl Serialize for VerificationRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let cmd = match self {
            Self::Carbon { opts, file } => format!("carbon {opts} {file:?}"),
            Self::Silicon { opts, file } => format!("silicon {opts} {file:?}"),
        };
        serializer.serialize_str(&cmd)
    }
}

impl VerificationRequest {
    pub fn silicon() -> SiliconOptsBuilder {
        SiliconOptsBuilder::default()
    }
    pub fn carbon() -> CarbonOptsBuilder {
        CarbonOptsBuilder::default()
    }
}

impl SiliconOptsBuilder {
    /// Try to detect `z3` from `PATH` using `which z3`
    pub fn detect_z3(&mut self) -> Result<&mut Self> {
        let z3 = String::from_utf8(
            std::process::Command::new("which")
                .arg("z3")
                .output()?
                .stdout,
        )?;
        Ok(self.z3_exe(z3))
    }
    pub fn verify_file(&self, file: impl AsRef<Path>) -> Result<VerificationRequest> {
        Ok(VerificationRequest::Silicon {
            opts: Box::new(self.build()?),
            file: file.as_ref().into(),
        })
    }
}
impl CarbonOptsBuilder {
    /// Try to detect `z3` from `PATH` using `which z3`
    pub fn detect_z3(&mut self) -> Result<&mut Self> {
        let z3 = String::from_utf8(
            std::process::Command::new("which")
                .arg("z3")
                .output()?
                .stdout,
        )?;
        Ok(self.z3_exe(z3))
    }
    pub fn verify_file(&self, file: impl AsRef<Path>) -> Result<VerificationRequest> {
        Ok(VerificationRequest::Carbon {
            opts: Box::new(self.build()?),
            file: file.as_ref().into(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerificationResponse {
    pub ast_id: u64,
    pub id: u64,
}

impl ViperRequest for VerificationRequest {
    type Response = VerificationResponse;

    fn ty(&self) -> &'static str {
        "verify"
    }

    fn url(&self) -> &'static str {
        "verify"
    }
}
