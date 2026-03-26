use mlx_rs::{Array, Device, Stream};

pub fn get_default_device() -> Device {
    Device::default()
}

pub fn get_default_stream() -> Stream {
    Stream::default()
}

pub fn add(a: &Array, b: &Array) -> Array {
    a + b
}

pub fn sub(a: &Array, b: &Array) -> Array {
    a - b
}

pub fn mul(a: &Array, b: &Array) -> Array {
    a * b
}

pub fn div(a: &Array, b: &Array) -> Array {
    a / b
}

pub fn matmul(a: &Array, b: &Array) -> Array {
    mlx_rs::ops::matmul(a, b).expect("Matmul failed")
}

pub struct Linear {
    pub weight: Array,
    pub bias: Array,
}

impl Linear {
    pub fn new(weight: Array, bias: Array) -> Self {
        Self { weight, bias }
    }

    pub fn forward(&self, x: &Array) -> Array {
        add(&matmul(x, &self.weight), &self.bias)
    }
}
