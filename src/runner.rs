use crate::errors::Error;
use crate::monitors;
use std::error;

pub async fn run() -> Result<(), Box<dyn error::Error>> {
  monitors::run().await?;

  Ok(())
}
