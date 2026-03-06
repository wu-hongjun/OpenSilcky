//! Slicky USB Light device driver.
//!
//! Wraps [`HidSlickyDevice`] as a [`DeviceDriver`] for use with the
//! [`DeviceRegistry`](crate::DeviceRegistry).

use crate::device::HidSlickyDevice;
use crate::{DeviceDriver, DeviceInfo, Result, StatusLightDevice, SupportedDevice};

/// Driver for Lexcelon Slicky-1.0 USB status lights.
pub struct SlickyDriver;

impl DeviceDriver for SlickyDriver {
    fn id(&self) -> &str {
        "slicky"
    }

    fn display_name(&self) -> &str {
        "Slicky USB Light"
    }

    fn supported_hardware(&self) -> Vec<SupportedDevice> {
        vec![SupportedDevice {
            name: "Slicky-1.0".into(),
            vid: 0x04d8,
            pid: 0xec24,
        }]
    }

    fn enumerate(&self, api: &hidapi::HidApi) -> Result<Vec<DeviceInfo>> {
        HidSlickyDevice::enumerate(api)
    }

    fn open(&self, api: &hidapi::HidApi) -> Result<Box<dyn StatusLightDevice>> {
        Ok(Box::new(HidSlickyDevice::open(api)?))
    }

    fn open_serial(
        &self,
        api: &hidapi::HidApi,
        serial: &str,
    ) -> Result<Box<dyn StatusLightDevice>> {
        Ok(Box::new(HidSlickyDevice::open_serial(api, serial)?))
    }
}
