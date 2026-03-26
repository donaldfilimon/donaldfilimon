use mlx_rs::DeviceType;
use wdbx::{get_default_device, get_default_stream};

#[test]
fn test_default_device_initialization() {
    let device = get_default_device();
    // On Apple Silicon, we expect GPU by default if metal is enabled
    let device_type = device.get_type().expect("Failed to get device type");
    assert!(matches!(device_type, DeviceType::Gpu | DeviceType::Cpu));
}

#[test]
fn test_default_stream_initialization() {
    let _stream = get_default_stream();
}
