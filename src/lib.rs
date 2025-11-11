#![allow(unused)]
#![allow(dead_code)]

use ash::{self, ext::queue_family_foreign, khr, vk};
use std::ffi::CStr;

pub struct Base {
    physical_device_info: PhysicalDeviceInfo,
}

impl Base {}

pub struct BaseConfig<'a> {
    app_info: vk::ApplicationInfo<'a>,
    instance_extensions: Vec<&'a CStr>,
    physical_device: PhysicalDeviceSelector,
    swapchain: SwapchainConfig,
}

impl Default for BaseConfig<'_> {
    fn default() -> Self {
        Self {
            app_info: Default::default(),
            instance_extensions: Default::default(),
            physical_device: Default::default(),
            swapchain: Default::default(),
        }
    }
}

impl<'a> BaseConfig<'a> {
    pub fn build(mut self) -> Base {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Entry") };
        let instance_extensions_raw: Vec<*const i8> = self
            .instance_extensions
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&self.app_info)
            .enabled_extension_names(&instance_extensions_raw);
        let instance = unsafe { entry.create_instance(&instance_create_info, None) };
        todo!();
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

#[derive(Clone, Copy)]
pub struct QueueFamilies<T> {
    pub graphics: T,
    pub compute: T,
    pub transfer: T,
    pub sparse: T,
    pub protected: T,
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

impl QueueFamilies<Option<u32>> {
    fn query_queues(&mut self, instance: &ash::Instance, physical_device: vk::PhysicalDevice) {
        let queues =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        for (idx, family) in queues.iter().enumerate() {
            let idx = idx as u32;

            if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) && self.graphics.is_none() {
                self.graphics = Some(idx);
            }

            if family.queue_flags.contains(vk::QueueFlags::COMPUTE) && self.compute.is_none() {
                self.compute = Some(idx);
            }

            if family.queue_flags.contains(vk::QueueFlags::TRANSFER) && self.transfer.is_none() {
                self.transfer = Some(idx);
            }

            if family.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING) && self.sparse.is_none()
            {
                self.sparse = Some(idx);
            }

            if family.queue_flags.contains(vk::QueueFlags::PROTECTED) && self.protected.is_none() {
                self.protected = Some(idx);
            }
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

// TODO: Add swapchain properties filter for device to make sure it is suitable.
impl PhysicalDeviceSelector {
    fn select(&self, instance: &ash::Instance) -> Option<PhysicalDeviceInfo> {
        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
        let suitable_devices: Vec<PhysicalDeviceInfo> = physical_devices
            .into_iter()
            .map(|physical_device| PhysicalDeviceInfo::new(physical_device, instance))
            .filter(|info| !self.require_discrete || info.is_discrete())
            .filter(|info| info.satisfies_families(self.required_queues))
            .filter(|info| info.satisfies_properties(self.properties))
            .filter(|info| info.satisfies_features(self.features))
            .collect();

        if self.prefer_best {
            suitable_devices.into_iter().max_by_key(|info| info.score())
        } else {
            suitable_devices.into_iter().min_by_key(|info| info.score())
        }
    }
}

pub struct PhysicalDeviceInfo {
    pub physical_device: vk::PhysicalDevice,
    pub queue_families_indices: QueueFamilies<Option<u32>>,
    pub properties: vk::PhysicalDeviceProperties,
    pub features: vk::PhysicalDeviceFeatures,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
}

impl PhysicalDeviceInfo {
    fn new(physical_device: vk::PhysicalDevice, instance: &ash::Instance) -> Self {
        let mut queue_family_indices: QueueFamilies<Option<u32>> = Default::default();
        queue_family_indices.query_queues(instance, physical_device);
        Self {
            physical_device,
            queue_families_indices: Default::default(),
            properties: Self::get_properties(&instance, physical_device),
            features: Self::get_features(&instance, physical_device),
            memory_properties: Self::get_memory(&instance, physical_device),
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

    fn is_discrete(&self) -> bool {
        self.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
    }

    fn satisfies_families(&self, queue_families: QueueFamilies<bool>) -> bool {
        if queue_families.graphics && self.queue_families_indices.graphics.is_none() {
            return false;
        }
        if queue_families.compute && self.queue_families_indices.compute.is_none() {
            return false;
        }
        if queue_families.transfer && self.queue_families_indices.transfer.is_none() {
            return false;
        }
        if queue_families.sparse && self.queue_families_indices.sparse.is_none() {
            return false;
        }
        if queue_families.protected && self.queue_families_indices.protected.is_none() {
            return false;
        }

        true
    }

    fn satisfies_properties(&self, propertes: vk::PhysicalDeviceProperties) -> bool {
        todo!()
    }

    fn satisfies_features(&self, features: vk::PhysicalDeviceFeatures) -> bool {
        todo!()
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        let vram_mb = self
            .memory_properties
            .memory_heaps
            .iter()
            .take(self.memory_properties.memory_heap_count as usize)
            .filter(|heap| heap.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL))
            .map(|heap| heap.size / (1024 * 1024)) // Convert to MB
            .sum::<u64>();
        score += ((vram_mb as f64).log2() as u32).min(1000);

        let limits = &self.properties.limits;
        score += (limits.max_compute_shared_memory_size / 1024).min(100) as u32;
        score += (limits.max_compute_work_group_invocations / 100).min(100) as u32;

        score += (limits.max_image_dimension2_d / 1000).min(100) as u32;
        score += (limits.max_framebuffer_width / 1000).min(100) as u32;

        if self.features.geometry_shader == vk::TRUE {
            score += 50;
        }
        if self.features.tessellation_shader == vk::TRUE {
            score += 50;
        }
        if self.features.multi_draw_indirect == vk::TRUE {
            score += 50;
        }
        score
    }
}
