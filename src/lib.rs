#![allow(unused)]
#![allow(dead_code)]

use ash::{self, khr, vk};
use std::ffi::CStr;

#[derive(Debug)]
pub struct Base {}
impl Base {
    pub fn new(config: BaseConfig) -> Self {
        Self {}
    }
}

pub struct BaseConfig<'a> {
    app_info: vk::ApplicationInfo<'a>,
    physical_device: PhysicalDeviceSelector,
    swapchain: SwapchainConfig,
}

impl Default for BaseConfig<'_> {
    fn default() -> Self {
        Self {
            app_info: Default::default(),
            physical_device: Default::default(),
            swapchain: Default::default(),
        }
    }
}

impl<'a> BaseConfig<'a> {
    pub fn with_app_info(mut self, name: &'a CStr, major: u32, minor: u32, patch: u32) -> Self {
        self.app_info = vk::ApplicationInfo::default()
            .application_name(name)
            .application_version(vk::make_api_version(0, major, minor, patch));
        self
    }
    pub fn with_device(
        mut self,
        physical_device_selector: fn(PhysicalDeviceSelector) -> PhysicalDeviceSelector,
    ) -> Self {
        self.physical_device = physical_device_selector(Default::default());
        self
    }
    pub fn with_swapchain(
        mut self,
        swapchain_config: fn(SwapchainConfig) -> SwapchainConfig,
    ) -> Self {
        self.swapchain = swapchain_config(Default::default());
        self
    }
}

pub struct SwapchainConfig {
    min_image_count: u32,
    image_format: vk::Format,
    image_sharing_mode: vk::SharingMode,
    color_space: vk::ColorSpaceKHR,
    present_mode: vk::PresentModeKHR,
    image_usage: vk::ImageUsageFlags,
    transforms: vk::SurfaceTransformFlagsKHR,
    composite_alpha: vk::CompositeAlphaFlagsKHR,
    array_layers: u32,
    extent: vk::Extent2D,
    clipped: bool,
}

impl Default for SwapchainConfig {
    fn default() -> Self {
        Self {
            min_image_count: todo!(
                "Try to acquire min img count supported by the swapchain and set that + 1"
            ),
            image_format: todo!("Set most common supported"),
            image_sharing_mode: todo!("Set most common supported"),
            color_space: todo!("same"),
            present_mode: todo!("same"),
            image_usage: todo!("same"),
            transforms: todo!("same"),
            composite_alpha: todo!("same"),
            array_layers: todo!("same"),
            extent: todo!("same"),
            clipped: todo!("same"),
        }
    }
}

impl SwapchainConfig {
    pub fn min_img_count(mut self, count: u32) -> Self {
        self.min_image_count = count;
        self
    }

    pub fn img_format(mut self, format: vk::Format) -> Self {
        self.image_format = format;
        self
    }
}

struct QueueFamilies<T> {
    graphics: T,
    compute: T,
    transfer: T,
    sparse: T,
    protected: T,
}

impl Default for QueueFamilies<bool> {
    fn default() -> Self {
        Self {
            graphics: false,
            compute: false,
            transfer: false,
            sparse: false,
            protected: false,
        }
    }
}

impl<T> Default for QueueFamilies<Option<T>> {
    fn default() -> Self {
        Self {
            graphics: None,
            compute: None,
            transfer: None,
            sparse: None,
            protected: None,
        }
    }
}

pub struct PhysicalDeviceSelector {
    prefer_best: bool,
    require_discrete: bool,
    required_queues: QueueFamilies<bool>,
    properties: Option<vk::PhysicalDeviceProperties>,
    features: Option<vk::PhysicalDeviceFeatures>,
}

impl Default for PhysicalDeviceSelector {
    fn default() -> Self {
        Self {
            prefer_best: true,
            require_discrete: false,
            required_queues: Default::default(),
            properties: Default::default(),
            features: Default::default(),
        }
    }
}

impl PhysicalDeviceSelector {
    pub fn require_graphics_queue(mut self) -> Self {
        todo!()
    }
    pub fn require_compute_queue(mut self) -> Self {
        todo!()
    }
    pub fn require_transfer_queue(mut self) -> Self {
        todo!()
    }
    pub fn require_sparse_queue(mut self) -> Self {
        todo!()
    }
    pub fn require_protected_queue(mut self) -> Self {
        todo!()
    }
}
