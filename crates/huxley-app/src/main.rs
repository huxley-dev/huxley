use anyhow::Result;
use crate::huxley_config::HuxleyConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(HuxleyConfig::from_env()?);

    Ok(())
}
