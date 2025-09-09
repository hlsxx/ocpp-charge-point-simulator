# OCPP Charge Point Simulator ⚡

A simple **OCPP charge point simulator** written in Rust.

<div align="center">
  <img src="https://github.com/hlsxx/ocpp-charge-point-simulator/blob/master/blob/example.png" alt="Example" style="width:80%; max-height:400px; border-radius:10px; box-shadow:0 0 10px rgba(0,0,0,0.3);" />
</div>

---

## 🚀 Features
- Supports **OCPP 1.6** ✅
- Future support for **OCPP 2.0.1** and **2.1** coming soon
- Simulate multiple charge points (implicit or explicit)
- Fully configurable intervals for boot, heartbeat, status, and transactions

## 📦 Installation
```bash
git clone https://github.com/hlsxx/ocpp-charge-point-simulator.git
cd ocpp-charge-point-simulator
```

## ⚙️ Usage
1. Copy and rename **config.toml.example** to **config.toml** or create your own.
2. Edit **config.toml** to configure your simulator.
3. Run the simulator:
```bash
cargo run
```

## 📝 Example Configuration
```toml
[general]
debug_mode = true
server_url = "ws://localhost:3000/charge-point"
ocpp_version = "ocpp1.6"

# Implicit charge points (optional)
#[implicit_charge_points]
#count = 5
#prefix = "CP"
#boot_delay_range = [1, 60]
#heartbeat_interval_range = [30, 90]
#status_interval_range = [10, 60]
#start_tx_after_range = [5, 15]
#stop_tx_after_range = [20, 60]

# Explicitly defined charge points
[[charge_points]]
id = "CP100001"
boot_delay_interval = 0
heartbeat_interval = 60
status_interval = 10
start_tx_after = 5
stop_tx_after = 20

[[charge_points]]
id = "CP100002"
boot_delay_interval = 10
heartbeat_interval = 30
status_interval = 15
start_tx_after = 10
stop_tx_after = 30
```

## 💡 Tips
- Use **implicit_charge_points** to quickly spin up multiple simulated charge points with randomized intervals.
- Use **charge_points** to manually configure each charge point with precise settings.

## 📌 Supported Versions
- ✅ v1.6
- ⬜ v2.0.1
- ⬜ v2.1

---

Made with ❤️  in Rust.
