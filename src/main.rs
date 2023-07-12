extern crate raspi_fancon;

use std::error::Error;

use log::info;
use raspi_fancon::{run, Environment};
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();

  info!("Device: {}", DeviceInfo::new()?.model());

  let env = Environment::new()?;

  if let Err(err) = run(env) {
    println!("{}", err);
  }

  Ok(())
}
