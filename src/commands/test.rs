//! Test command - Run local tests for a problem

use std::path::PathBuf;

use anyhow::Result;

use crate::test_runner::TestRunner;

/// Run local tests for a problem
pub async fn execute(id: u32, test_file: Option<PathBuf>) -> Result<()> {
    let runner = TestRunner::new(id, test_file)?;
    runner.run().await?;
    Ok(())
}
