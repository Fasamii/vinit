use ash::{self, khr, vk};
use std::ffi::CStr;

pub struct Renderer {}
impl Renderer {}

pub struct RendererConfig<'a> {
    app_info: vk::ApplicationInfo<'a>,
    physical_device: PhysicalDeviceSelector,
}

impl Default for RendererConfig<'_> {
    fn default() -> Self {
        Self {
            app_info: Default::default(),
            physical_device: Default::default(),
        }
    }
}

impl<'a> RendererConfig<'a> {
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
