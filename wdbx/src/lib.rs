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

pub fn benchmark_matmul(size: usize, iters: usize) -> f64 {
    let shape = [size as i32, size as i32];
    // mlx_rs::random::uniform::<Lower, T>(lower, upper, shape, key)
    // Using default key (None) and Float32
    let a = mlx_rs::random::uniform::<f32, f32>(0.0, 1.0, &shape, None).expect("Failed to create random array");
    let b = mlx_rs::random::uniform::<f32, f32>(0.0, 1.0, &shape, None).expect("Failed to create random array");
    
    // Warm-up
    let c = matmul(&a, &b);
    c.eval().expect("Failed to evaluate");

    let start = std::time::Instant::now();
    for _ in 0..iters {
        let c = matmul(&a, &b);
        c.eval().expect("Failed to evaluate");
    }
    let duration = start.elapsed();
    duration.as_secs_f64() / iters as f64
}
