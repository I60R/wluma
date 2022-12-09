use std::error::Error;

use super::object::Object;

pub mod opengl;
pub mod vulkan;

pub trait Processor {
    fn luma_percent(&self, frame: &Object) -> Result<u8, Box<dyn Error>>;
}