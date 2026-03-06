//! Pluggable device driver abstraction.
//!
//! The [`DeviceDriver`] trait allows different USB status light hardware
//! to be discovered and opened through a uniform interface.

use crate::{DeviceInfo, Result, StatusLightDevice};

/// A driver that can discover and open devices of a specific type.
pub trait DeviceDriver: Send + Sync {
    /// Unique driver identifier (e.g. "slicky", "arduino-rgb").
    fn id(&self) -> &str;

    /// Human-readable name (e.g. "Slicky USB Light").
    fn display_name(&self) -> &str;

    /// Enumerate all connected devices this driver supports.
    fn enumerate(&self) -> Result<Vec<DeviceInfo>>;

    /// Open the first available device.
    fn open(&self) -> Result<Box<dyn StatusLightDevice>>;

    /// Open a device by serial number.
    fn open_serial(&self, serial: &str) -> Result<Box<dyn StatusLightDevice>>;
}
