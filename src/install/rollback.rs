pub struct RollbackManager {
    manifest: Vec<PathBuf>,
}

impl RollbackManager {
    pub fn new(install_dir: &Path) -> Self {
        let mut manifest = vec![install_dir.to_path_buf()];
        manifest.push(install_dir.join("conf/bitcoin.conf"));
        Self { manifest }
    }

    pub fn execute_rollback(&self) -> Result<()> {
        for path in self.manifest.iter().rev() {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
} 