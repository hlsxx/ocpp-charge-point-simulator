# OCPP Charge Point Simulator

A charge point simulator written in Rust, built for testing OCPP backends without physical hardware.

![Example](https://github.com/hlsxx/ocpp-charge-point-simulator/blob/master/blob/example.png)

---

## Modes

### Dynamic

Runs a fully automated charge cycle — boot, authorize, start transaction, send meter values, and stop — on a configurable schedule. Useful for load testing or verifying your CSMS handles a continuous stream of charging sessions correctly.

### Idle

Connects and waits for commands from the CSMS. Responds to `RemoteStartTransaction` and `RemoteStopTransaction` as a real charge point would. Useful for integration testing where you want to drive the session manually from the backend side.

---

## Features

- OCPP 1.6 support
- Simulate multiple charge points, either explicitly configured or spun up implicitly in bulk
- Configurable intervals for boot delay, heartbeat, meter values, and transaction timing
- OCPP 2.0.1 and 2.1 support planned

---

## Installation
```bash
git clone https://github.com/hlsxx/ocpp-charge-point-simulator.git
cd ocpp-charge-point-simulator
```

---

## Usage

1. Copy `config.toml.example` to `config.toml` and edit it to match your setup.
2. Run:
```bash
cargo run
```

---

## Configuration
```toml
[general]
debug_mode = true
server_url = "ws://localhost:3000/charge-point"
ocpp_version = "ocpp1.6"

# Spin up multiple charge points with randomized intervals
#[implicit_charge_points]
#count = 5
#prefix = "CP"
#boot_delay_range = [1, 60]
#heartbeat_interval_range = [30, 90]
#start_tx_after_range = [5, 15]
#stop_tx_after_range = [20, 60]

# Or define each charge point explicitly
[[charge_points]]
id = "CP100001"
boot_delay_interval = 0
heartbeat_interval = 60
start_tx_after = 5
stop_tx_after = 20

[[charge_points]]
id = "CP100002"
boot_delay_interval = 10
heartbeat_interval = 30
start_tx_after = 10
stop_tx_after = 30
```

Use `implicit_charge_points` when you want to flood-test your CSMS with many simultaneous sessions. Use `charge_points` when you need predictable, repeatable behavior from specific charge point IDs.

---

## OCPP Version Support

| Version | Status |
|---------|--------|
| 1.6     | Supported |
| 2.0.1   | Planned |
| 2.1     | Planned |
