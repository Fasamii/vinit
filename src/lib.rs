#![allow(unused)]
#![allow(dead_code)]

use ash::{self, khr, vk};
use std::ffi::CStr;

pub struct Base {
    physical_device_info: PhysicalDeviceInfo,
}

impl Base {}

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
    pub fn create(mut self) -> Base {
        todo!("")
    }

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
    properties: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
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
        self.required_queues.graphics = true;
        self
    }
    pub fn require_compute_queue(mut self) -> Self {
        self.required_queues.compute = true;
        self
    }
    pub fn require_transfer_queue(mut self) -> Self {
        self.required_queues.transfer = true;
        self
    }
    pub fn require_sparse_queue(mut self) -> Self {
        self.required_queues.sparse = true;
        self
    }
    pub fn require_protected_queue(mut self) -> Self {
        self.required_queues.protected = true;
        self
    }
    pub fn prefer_bset(mut self, prefer: bool) -> Self {
        self.prefer_best = prefer;
        self
    }
    pub fn require_discrete(mut self, require: bool) -> Self {
        self.require_discrete = require;
        self
    }

    pub fn require_properties(mut self, properties: vk::PhysicalDeviceProperties) -> Self {
        self.properties = properties;
        self
    }
    pub fn require_features(mut self, features: vk::PhysicalDeviceFeatures) -> Self {
        self.features = features;
        self
    }
}

impl PhysicalDeviceSelector {
    fn select(&self, instance: &ash::Instance) -> PhysicalDeviceInfo {
        todo!("Implement that")
    }
}

pub struct PhysicalDeviceInfo {
    pub physical_device: vk::PhysicalDevice,
    pub properties: vk::PhysicalDeviceProperties,
    pub features: vk::PhysicalDeviceFeatures,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
}

impl PhysicalDeviceInfo {
    fn new(physical_device: vk::PhysicalDevice, instance: &ash::Instance) -> Self {
        Self {
            physical_device,
            properties: Self::get_properties(instance, physical_device),
            features: Self::get_features(instance, physical_device),
            memory_properties: Self::get_memory(instance, physical_device),
        }
    }

    fn get_properties(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        unsafe { instance.get_physical_device_properties(physical_device) }
    }
    fn get_features(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures {
        unsafe { instance.get_physical_device_features(physical_device) }
    }

    fn get_memory(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        unsafe { instance.get_physical_device_memory_properties(physical_device) }
    }
}
