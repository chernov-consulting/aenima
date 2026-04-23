//! Speaker playback via cpal. Accepts PCM16 mono frames on a channel.

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait};
use tracing::info;

use super::input::DeviceInfo;

pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    let host = cpal::default_host();
    let mut out = Vec::new();
    for dev in host.output_devices().context("enumerating output devices")? {
        let name = dev.name().unwrap_or_else(|_| "<unnamed>".into());
        let sample_rate = dev
            .default_output_config()
            .ok()
            .map(|c| c.sample_rate().0);
        out.push(DeviceInfo {
            name,
            default_sample_rate_hz: sample_rate,
        });
    }
    Ok(out)
}

pub fn select(preferred_name: Option<&str>) -> Result<cpal::Device> {
    let host = cpal::default_host();
    if let Some(want) = preferred_name {
        for dev in host.output_devices().context("enumerating output devices")? {
            if dev.name().ok().as_deref() == Some(want) {
                info!(device = %want, "selected output device by name");
                return Ok(dev);
            }
        }
        info!(
            requested = %want,
            "requested output device not found; falling back to default"
        );
    }
    host.default_output_device()
        .context("no default output device available")
}
