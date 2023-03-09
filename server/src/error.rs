use thiserror::Error;

pub type Result<T, E = ViperServerError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ViperServerError {
    #[error("failed to connect to server")]
    ConnectToServerFailed {
        stdout: String,
        stderr: String,
        source: Box<ViperServerError>,
    },
    #[error("building server from options")]
    BuildServer(#[from] crate::opts::ViperServerOptsBuilderError),
    #[error("building silicon from options")]
    SiliconOpts(#[from] crate::opts::SiliconOptsBuilderError),
    #[error("building carbon from options")]
    CarbonOpts(#[from] crate::opts::CarbonOptsBuilderError),
    #[error("failed to spawn server using `{viper_server_jar}`")]
    SpawnServer {
        source: std::io::Error,
        viper_server_jar: String,
    },
    #[error("server did not announce it's port")]
    ServerDidNotReportPort {
        source: tokio::sync::oneshot::error::RecvError,
    },
    #[error("failed to parse url")]
    InvalidUrl {
        #[from]
        source: url::ParseError,
    },
    #[error("io error")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("invalid utf-8")]
    InvalidUtf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },
    #[error("failed to parse json: {json:?}")]
    ParseJson {
        json: String,
        source: serde_json::Error,
    },
    #[error("network error")]
    Network {
        #[from]
        source: reqwest::Error,
    },
}
