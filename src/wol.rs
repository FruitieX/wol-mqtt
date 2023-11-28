use std::{collections::HashMap, time::Duration};

use eyre::{ContextCompat, Result};
use serde::Deserialize;

use crate::mqtt::MqttClient;

#[derive(Clone, Debug, Deserialize)]
pub struct WolDeviceConfig {
    pub name: String,
    pub id: String,
    pub mac: String,
    pub sleep_on_lan: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WolConfig {
    pub devices: HashMap<String, WolDeviceConfig>,
}

async fn wake_device(device_config: &WolDeviceConfig) -> Result<()> {
    let mac = device_config.mac.clone();
    tokio::task::spawn_blocking(move || wakey::WolPacket::from_string(mac, ':')?.send_magic())
        .await??;

    Ok(())
}

async fn sleep_device(device_config: &WolDeviceConfig) -> Result<()> {
    if let Some(sleep_on_lan) = &device_config.sleep_on_lan {
        tokio::time::timeout(Duration::from_secs(1), async move {
            surf::get(sleep_on_lan).await.map_err(|err| eyre!(err))
        })
        .await??;
    }

    Ok(())
}

pub async fn init_wol_loop(device_config: WolDeviceConfig, mqtt_client: MqttClient) -> Result<()> {
    let device_config = device_config.clone();

    tokio::task::spawn(async move {
        loop {
            let result = handle_next_msg(&device_config, &mqtt_client).await;

            if let Err(err) = result {
                error!("Error in WOL loop: {}", err);
            }
        }
    });

    Ok(())
}

pub async fn handle_next_msg(
    device_config: &WolDeviceConfig,
    mqtt_client: &MqttClient,
) -> Result<()> {
    let msg = {
        let mqtt_rx = mqtt_client.rx_map.get(&device_config.id).with_context(|| {
            format!(
                "Could not find configured MQTT device with id {}",
                device_config.id
            )
        })?;
        let mut mqtt_rx = mqtt_rx.write().await;
        mqtt_rx.changed().await?;
        let value = &*mqtt_rx.borrow();

        value
            .clone()
            .context("Expected to receive mqtt message from rx channel")?
    };

    if msg.power == Some(true) {
        wake_device(device_config).await
    } else if msg.power == Some(false) {
        sleep_device(device_config).await
    } else {
        Ok(())
    }
}
