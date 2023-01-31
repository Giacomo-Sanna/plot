
pub mod graph {
    pub const WIDTH: u32 = 1280;
    pub const HEIGHT: u32 = 960;
    pub const LABEL_AREA_SIZE: u32 = 60;
    pub const DEFAULT_FONT: (&str, u32) = ("sans-serif", 50);
    pub(crate) const DEFAULT_DIR: &str = "plots-output";
    pub const ERROR_PRESENTING: &str = "Unable to write result to file please make sure directory 'plots-output' exists under root project dir";
}

pub fn f32_max(v: &[f32]) -> f32 {
    v.iter().copied().fold(f32::NAN, f32::max)
}

pub fn f32_min(v: &[f32]) -> f32 {
    v.iter().copied().fold(f32::NAN, f32::min)
}

pub fn get_file_path(file_name: &str) -> String {
    format!("{}/{}.png", graph::DEFAULT_DIR , file_name)
}

