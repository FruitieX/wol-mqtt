# wol-mqtt

This program sends Wake-on-LAN/[Sleep-on-LAN](https://github.com/SR-G/sleep-on-lan) packets to configured devices when it receives MQTT messages.

## Running

Make sure you have a recent version of Rust installed.

1. Clone this repo
2. Copy Settings.toml.example -> Settings.toml
3. Configure Settings.toml to match your setup (see below)
4. `cargo run`

## Configuration

For each device, you will need to retrieve and note down:

- Device network interface MAC address
- Sleep on LAN endpoint if you wish to use that functionality

## MQTT protocol

MQTT messages use the following JSON format:

```
{
  "id": "<device_id>",
  "power": true
}
```