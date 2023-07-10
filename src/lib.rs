use std::{
  env,
  error::Error,
  fs::File,
  io::Read,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  thread,
  time::Duration,
};

use log::{debug, info};
use rppal::gpio::Gpio;
use signal_hook::{consts::SIGTERM, flag::register};

pub struct Environment {
  temperature_file: String,
  fan_pin: u8,
  fan_on_temp: u32,
  fan_off_temp: u32,
  check_temp_interval: Duration,
}

impl Environment {
  pub fn new() -> Result<Environment, Box<dyn Error>> {
    let temperature_file = env::var("TEMPERATURE_FILE")?;
    let fan_pin: u8 = env::var("FAN_PIN")?.parse()?;
    let fan_on_temp: u32 = env::var("FAN_ON_TEMP")?.parse()?;
    let fan_off_temp: u32 = env::var("FAN_OFF_TEMP")?.parse()?;
    let check_temp_interval = Duration::from_secs(env::var("CHECK_TEMP_INTERVAL")?.parse()?);

    Ok(Environment {
      temperature_file,
      fan_pin,
      fan_on_temp,
      fan_off_temp,
      check_temp_interval,
    })
  }
}

pub fn run(env: Environment) -> Result<(), Box<dyn Error>> {
  let term = Arc::new(AtomicBool::new(false));
  register(SIGTERM, Arc::clone(&term))?;

  let mut pin = Gpio::new()?.get(env.fan_pin)?.into_output();
  let mut is_high = pin.is_set_high();

  while !term.load(Ordering::Relaxed) {
    let temp = match get_temp(&env.temperature_file) {
      Ok(it) => it,
      Err(err) => {
        pin.set_low();
        return Err(err);
      }
    };

    debug!("Current temp: {}", temp);

    if temp >= env.fan_on_temp && !is_high {
      info!("Fan on");
      is_high = true;
      pin.set_high();
    } else if temp <= env.fan_off_temp && is_high {
      info!("Fan off");
      is_high = false;
      pin.set_low();
    }
    thread::sleep(env.check_temp_interval);
  }

  pin.set_low();

  Ok(())
}

fn get_temp(filename: &str) -> Result<u32, Box<dyn Error>> {
  let mut f = File::open(filename)?;
  let mut temp = String::new();
  f.read_to_string(&mut temp)?;
  Ok(temp.trim().parse::<u32>()? / 1000)
}
