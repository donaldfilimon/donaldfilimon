use mlx_rs::{Device, Stream};

pub fn get_default_device() -> Device {
    Device::default()
}

pub fn get_default_stream() -> Stream {
    Stream::default()
}
