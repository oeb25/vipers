use std::path::{Path, PathBuf};

const VIPERSERVER_JAR_BYTES: &[u8] =
    include_bytes!("../viperserver/target/scala-2.13/viperserver.jar");

#[derive(Debug)]
pub struct ViperServerJar {
    _dir: tempfile::TempDir,
    path: PathBuf,
}

impl ViperServerJar {
    pub fn create() -> std::io::Result<ViperServerJar> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("viperserver.jar");

        std::fs::write(&path, VIPERSERVER_JAR_BYTES)?;

        Ok(ViperServerJar { _dir: dir, path })
    }
}

impl AsRef<Path> for ViperServerJar {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
