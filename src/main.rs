#[macro_use]
extern crate eyre;

#[macro_use]
extern crate log;

use eyre::Result;
use wol::init_wol_loop;

use crate::{config::read_config_devices, mqtt::init_mqtt};

mod config;
mod mqtt;
mod wol;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();

    let (mqtt_config, wol_config) = read_config_devices()?;
    let mqtt_client = init_mqtt(&mqtt_config, &wol_config).await?;

    for device in wol_config.devices.into_values() {
        let mqtt_client = mqtt_client.clone();
        init_wol_loop(device, mqtt_client).await?;
    }

    tokio::signal::ctrl_c().await?;

    Ok(())
}
