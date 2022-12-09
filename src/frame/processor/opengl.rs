use crate::frame::object::Object;
use ash::{vk, Device, Entry, Instance};
use std::cell::RefCell;
use std::error::Error;
use std::ops::Drop;

use super::Processor;

const WLUMA_VERSION: u32 = vk::make_api_version(0, 4, 1, 2);
const VULKAN_VERSION: u32 = vk::make_api_version(0, 1, 2, 0);

const FINAL_MIP_LEVEL: u32 = 4; // Don't generate mipmaps beyond this level - GPU is doing too poor of a job averaging the colors
const BUFFER_PIXELS: u64 = 500 * 4; // Pre-allocated buffer size, should be enough to fit FINAL_MIP_LEVEL
const FENCES_TIMEOUT_NS: u64 = 1_000_000_000;

pub struct OpenGL {
    _entry: Entry, // must keep reference to prevent early memory release
    instance: Instance,
    device: Device,
    buffer: vk::Buffer,
    buffer_memory: vk::DeviceMemory,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    queue: vk::Queue,
    fence: vk::Fence,
    image: RefCell<Option<vk::Image>>,
    image_memory: RefCell<Option<vk::DeviceMemory>>,
    image_resolution: RefCell<Option<(u32, u32)>>,
}

impl OpenGL {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        todo!()
    }

    fn init_image(&self, frame: &Object) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn init_frame_image(
        &self,
        frame: &Object,
    ) -> Result<(vk::Image, vk::DeviceMemory), Box<dyn Error>> {
        todo!()
    }

    #[allow(clippy::too_many_arguments)]
    fn add_barrier(
        &self,
        image: &vk::Image,
        base_mip_level: u32,
        mip_levels: u32,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
        src_access_mask: vk::AccessFlags,
        dst_access_mask: vk::AccessFlags,
        src_stage_mask: vk::PipelineStageFlags,
    ) {
        todo!()
    }

    #[allow(clippy::too_many_arguments)]
    fn blit(
        &self,
        src_image: &vk::Image,
        src_width: u32,
        src_height: u32,
        src_mip_level: u32,
        dst_image: &vk::Image,
        dst_width: u32,
        dst_height: u32,
        dst_mip_level: u32,
    ) {
        todo!()
    }

    fn generate_mipmaps(
        &self,
        frame: &Object,
        frame_image: &vk::Image,
        image: &vk::Image,
    ) -> (u32, u32, u32) {
        todo!()
    }

    fn copy_mipmap(&self, image: &vk::Image, mip_level: u32, width: u32, height: u32) {
        todo!()
    }

    fn begin_commands(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn submit_commands(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl Processor for OpenGL {
    fn luma_percent(&self, frame: &Object) -> Result<u8, Box<dyn Error>> {
        todo!()
    }
}

impl Drop for OpenGL {
    fn drop(&mut self) {
        todo!()
    }
}

fn image_dimensions(frame: &Object) -> (u32, u32, u32) {
    let width = frame.width / 2;
    let height = frame.height / 2;
    let mip_levels = f64::max(width.into(), height.into()).log2().floor() as u32 + 1;
    (width, height, mip_levels)
}

fn find_memory_type_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _)| index as _)
}
