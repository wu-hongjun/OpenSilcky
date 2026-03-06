//! Shared HID utilities for device drivers.
//!
//! Provides reusable enumerate/open functions so each driver doesn't
//! duplicate HID discovery logic.

use crate::device::DeviceInfo;
use crate::error::{Result, StatusLightError};

/// Enumerate all HID devices matching any of the given VID/PID pairs.
pub fn enumerate_hid(vid_pid_pairs: &[(u16, u16)], driver_id: &str) -> Result<Vec<DeviceInfo>> {
    let api = hidapi::HidApi::new()?;
    let devices = api
        .device_list()
        .filter(|d| {
            vid_pid_pairs
                .iter()
                .any(|&(vid, pid)| d.vendor_id() == vid && d.product_id() == pid)
        })
        .map(|d| DeviceInfo {
            path: d.path().to_string_lossy().to_string(),
            serial: d.serial_number().map(|s| s.to_string()),
            manufacturer: d.manufacturer_string().map(|s| s.to_string()),
            product: d.product_string().map(|s| s.to_string()),
            vid: d.vendor_id(),
            pid: d.product_id(),
            driver_id: driver_id.to_string(),
        })
        .collect();
    Ok(devices)
}

/// Open the first HID device matching any of the given VID/PID pairs.
///
/// Returns the opened device and its serial number (if available).
/// Returns [`StatusLightError::DeviceNotFound`] if no matching device is found.
pub fn open_first_hid(vid_pid_pairs: &[(u16, u16)]) -> Result<(hidapi::HidDevice, Option<String>)> {
    let api = hidapi::HidApi::new()?;
    let info = api
        .device_list()
        .find(|d| {
            vid_pid_pairs
                .iter()
                .any(|&(vid, pid)| d.vendor_id() == vid && d.product_id() == pid)
        })
        .ok_or(StatusLightError::DeviceNotFound)?;

    let serial = info.serial_number().map(|s| s.to_string());
    let device = info.open_device(&api)?;
    Ok((device, serial))
}

/// Open a HID device by serial number, matching any of the given VID/PID pairs.
///
/// Returns [`StatusLightError::DeviceNotFound`] if no matching device is found.
pub fn open_hid_by_serial(
    vid_pid_pairs: &[(u16, u16)],
    serial: &str,
) -> Result<(hidapi::HidDevice, Option<String>)> {
    let api = hidapi::HidApi::new()?;
    let info = api
        .device_list()
        .find(|d| {
            vid_pid_pairs
                .iter()
                .any(|&(vid, pid)| d.vendor_id() == vid && d.product_id() == pid)
                && d.serial_number().is_some_and(|s| s == serial)
        })
        .ok_or(StatusLightError::DeviceNotFound)?;

    let serial = info.serial_number().map(|s| s.to_string());
    let device = info.open_device(&api)?;
    Ok((device, serial))
}
