use rand::Rng;
use std::borrow::{Borrow, BorrowMut};

pub mod chart {
    pub const WIDTH: u32 = 1280;
    pub const HEIGHT: u32 = 960;
    pub const LABEL_AREA_SIZE: u32 = 60;
    pub const MARGIN : u32 = 5;
    pub const DEFAULT_FONT: (&str, u32) = ("sans-serif", 50);
    pub(crate) const DEFAULT_DIR: &str = "plots-output";
    // pub const ERROR_PRESENTING: &str = "ERROR: Unable to write result to file please make sure directory 'plots-output' exists under root project dir";
}

pub mod interactive_chart {
    pub const W: usize = 800;
    pub const H: usize = 600;
    pub const MARGIN: u32 = 20;
    pub const LABEL: u32 = 50;
    pub const SAMPLE_RATE: f64 = 60.0;
    pub const DEFAULT_FONT: (&str, u32) = ("sans-serif", 40);
}

pub fn f32_max(v: &[f32]) -> f32 {
    v.iter().copied().fold(f32::NAN, f32::max)
}

pub fn f32_min(v: &[f32]) -> f32 {
    v.iter().copied().fold(f32::NAN, f32::min)
}

pub fn get_file_path<'a>(file_name: &str) -> String {
    format!("{}/{}.png", chart::DEFAULT_DIR, file_name)
}

pub fn generate_data_series(start_value: f32, len: usize, min_change: f32, max_change: f32) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    let mut v = vec![start_value];
    let mut prev = start_value;
    for _ in 0..len - 1 {
        let curr = prev + rng.gen_range(min_change..max_change) * prev;
        v.push(curr);
        prev = curr;
    };
    v
}

// -----------------------------------
// Code from: https://github.com/plotters-rs/plotters-minifb-demo/blob/master/src/main.rs#L15

pub struct BufferWrapper(Vec<u32>);

impl BufferWrapper {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![0u32; width * height])
    }
}

impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe {
            std::slice::from_raw_parts(
                self.0.as_ptr() as *const u8,
                self.0.len() * 4,
            )
        }
    }
}

impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.as_mut_ptr() as *mut u8,
                self.0.len() * 4,
            )
        }
    }
}

impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        self.0.as_slice()
    }
}

impl BorrowMut<[u32]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u32] {
        self.0.as_mut_slice()
    }
}
