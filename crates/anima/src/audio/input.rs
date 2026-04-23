//! Microphone capture via cpal. Emits PCM16 mono frames on a channel.

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait};
use tracing::info;

/// List all available input devices on the default host, with their default
/// input configs. Used by the debug dashboard.
pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    let host = cpal::default_host();
    let mut out = Vec::new();
    for dev in host.input_devices().context("enumerating input devices")? {
        let name = dev.name().unwrap_or_else(|_| "<unnamed>".into());
        let sample_rate = dev
            .default_input_config()
            .ok()
            .map(|c| c.sample_rate().0);
        out.push(DeviceInfo {
            name,
            default_sample_rate_hz: sample_rate,
        });
    }
    Ok(out)
}

/// Summary of a host audio device, exposed via the debug dashboard.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub default_sample_rate_hz: Option<u32>,
}

/// Select the input device matching `preferred_name`, or the default input
/// device if the preference is `None` or unmatched.
pub fn select(preferred_name: Option<&str>) -> Result<cpal::Device> {
    let host = cpal::default_host();
    if let Some(want) = preferred_name {
        for dev in host.input_devices().context("enumerating input devices")? {
            if dev.name().ok().as_deref() == Some(want) {
                info!(device = %want, "selected input device by name");
                return Ok(dev);
            }
        }
        info!(
            requested = %want,
            "requested input device not found; falling back to default"
        );
    }
    host.default_input_device()
        .context("no default input device available")
}
