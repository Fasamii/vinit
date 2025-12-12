use crate::families::Families;
use crate::{Absent, Present, Store};
use crate::{families, mass};
use ash::{khr, vk};
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::sync::Arc;

pub trait BuildDevice<S: Store<DeviceInfo>> {
    fn build_device(
        config: Option<PhysicalDeviceSelector>,
        instance: &ash::Instance,
        extensions: Vec<CString>, // TODO: Convert that later into &CStr and pass with []
        required_queues: families::Families<bool>,
    ) -> Result<S::Stored, vk::Result>;
}

impl BuildDevice<Absent> for Absent {
    fn build_device(
        _config: Option<PhysicalDeviceSelector>,
        _instance: &ash::Instance,
        _extensions: Vec<CString>,
        _required_queues: families::Families<bool>,
    ) -> Result<(), vk::Result> {
        Ok(())
    }
}

impl BuildDevice<Present> for Present {
    fn build_device(
        config: Option<PhysicalDeviceSelector>,
        instance: &ash::Instance,
        extensions: Vec<CString>,
        required_queues: families::Families<bool>,
    ) -> Result<DeviceInfo, vk::Result> {
        let physical_device_info = config
            .unwrap_or_else(|| {
                panic!("Attemt to select phyiscal device withot specyfing selector");
            })
            .require_extensions(extensions)
            .require_queues(required_queues)
            .select(instance)?
            .ok_or(vk::Result::ERROR_FEATURE_NOT_PRESENT)?;
        DeviceInfo::new(physical_device_info, required_queues, instance)
    }
}

pub struct DeviceInfo {
    pub device: Arc<ash::Device>,
    pub physical_info: PhysicalDeviceInfo,
    pub queue_handles: families::Families<Option<vk::Queue>>,
}

impl DeviceInfo {
    fn new(
        physical_device_info: PhysicalDeviceInfo,
        required_queues: families::Families<bool>,
        instance: &ash::Instance,
    ) -> Result<Self, vk::Result> {
        let required_queue_family_indices = physical_device_info
            .queue_families_indices
            .filter_required(&required_queues);
        let queue_create_info = required_queue_family_indices.make_create_info(&[1.0f32]);

        let device_extensions_raw: Vec<*const i8> = physical_device_info
            .enabled_extensions
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        let device_create_info = vk::DeviceCreateInfo::default()
            .enabled_features(&physical_device_info.enabled_features)
            .enabled_extension_names(&device_extensions_raw)
            .queue_create_infos(&queue_create_info);
        println!("device_create_info = {device_create_info:#?}");

        let device = unsafe {
            instance.create_device(
                physical_device_info.physical_device,
                &device_create_info,
                None,
            )?
        };
        let queue_handles: families::Families<Option<vk::Queue>> =
            families::Families::new(&device, required_queue_family_indices);
        Ok(Self {
            device: Arc::new(device),
            physical_info: physical_device_info,
            queue_handles,
        })
    }
}

impl Drop for DeviceInfo {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().ok();
            self.device.destroy_device(None);
        }
    }
}

pub struct PhysicalDeviceSelector {
    prefer_best: bool,
    require_discrete: bool,
    required_queues: families::Families<bool>,
    required_properties: vk::PhysicalDeviceProperties,
    required_features: vk::PhysicalDeviceFeatures,
    required_extensions: Vec<CString>,
}

impl Default for PhysicalDeviceSelector {
    fn default() -> Self {
        Self {
            prefer_best: true,
            require_discrete: false,
            required_queues: Default::default(),
            required_properties: Default::default(),
            required_features: Default::default(),
            required_extensions: Default::default(),
        }
    }
}

impl PhysicalDeviceSelector {
    fn require_extensions(mut self, extensions: Vec<CString>) -> Self {
        self.required_extensions = extensions;
        self
    }
}

impl PhysicalDeviceSelector {
    pub fn prefer_best(mut self, prefer: bool) -> Self {
        self.prefer_best = prefer;
        self
    }
    pub fn require_discrete(mut self, require: bool) -> Self {
        self.require_discrete = require;
        self
    }
    pub fn require_properties(mut self, properties: vk::PhysicalDeviceProperties) -> Self {
        self.required_properties = properties;
        self
    }
    pub fn require_features(mut self, features: vk::PhysicalDeviceFeatures) -> Self {
        self.required_features = features;
        self
    }
    fn require_queues(mut self, queues: families::Families<bool>) -> Self {
        self.required_queues = queues;
        self
    }
}

// TODO: Add swapchain properties filter for device to make sure it is suitable.
impl PhysicalDeviceSelector {
    fn select(&self, instance: &ash::Instance) -> Result<Option<PhysicalDeviceInfo>, vk::Result> {
        let physical_device_handles = unsafe { instance.enumerate_physical_devices()? };
        let physical_device_infos: Vec<PhysicalDeviceInfo> = physical_device_handles
            .into_iter()
            .map(|physical_device| {
                PhysicalDeviceInfo::new(
                    physical_device,
                    self.required_features,
                    self.required_extensions.clone(),
                    instance,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let candidates: Vec<PhysicalDeviceInfo> = physical_device_infos
            .into_iter()
            .filter(|info| !self.require_discrete || info.is_discrete())
            .filter(|info| info.satisfies_families(self.required_queues))
            .filter(|info| info.satisfies_extensions(&self.required_extensions))
            .filter(|info| info.satisfies_properties(self.required_properties))
            .filter(|info| info.satisfies_features(self.required_features))
            .collect();

        if self.prefer_best {
            Ok(candidates.into_iter().max_by_key(|info| info.score()))
        } else {
            Ok(candidates.into_iter().min_by_key(|info| info.score()))
        }
    }
}

pub struct PhysicalDeviceInfo {
    pub physical_device: vk::PhysicalDevice,
    pub queue_families_indices: families::Families<Option<u32>>,
    pub properties: vk::PhysicalDeviceProperties,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub enabled_features: vk::PhysicalDeviceFeatures,
    pub supproted_features: vk::PhysicalDeviceFeatures,
    pub enabled_extensions: Vec<CString>,
    pub supported_extensions: Vec<vk::ExtensionProperties>,
}

impl PhysicalDeviceInfo {
    fn new(
        physical_device: vk::PhysicalDevice,
        enabled_features: vk::PhysicalDeviceFeatures,
        enabled_extensions: Vec<CString>,
        instance: &ash::Instance,
    ) -> Result<Self, vk::Result> {
        Ok(Self {
            physical_device,
            queue_families_indices: families::Families::query(instance, physical_device),
            properties: Self::get_properties(instance, physical_device),
            memory_properties: Self::get_memory(instance, physical_device),
            enabled_features,
            supproted_features: Self::get_features(instance, physical_device),
            enabled_extensions,
            supported_extensions: Self::get_extensions(instance, physical_device)?,
        })
    }
}

impl PhysicalDeviceInfo {
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

    fn get_extensions(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        unsafe { instance.enumerate_device_extension_properties(physical_device) }
    }
}

impl PhysicalDeviceInfo {
    fn is_discrete(&self) -> bool {
        self.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
    }

    fn satisfies_families(&self, queue_families: families::Families<bool>) -> bool {
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
        mass::satisfies_properties(&self.properties, &propertes)
    }

    fn satisfies_features(&self, features: vk::PhysicalDeviceFeatures) -> bool {
        mass::satisifes_features(&self.supproted_features, &features)
    }

    fn satisfies_extensions(&self, extensions: &[CString]) -> bool {
        let available: HashSet<&CStr> = self
            .supported_extensions
            .iter()
            .map(|extension| unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) })
            .collect();
        extensions
            .iter()
            .all(|required| available.contains(required.as_c_str()))
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
        score += (limits.max_compute_shared_memory_size / 1024).min(100);
        score += (limits.max_compute_work_group_invocations / 100).min(100);

        score += (limits.max_image_dimension2_d / 1000).min(100);
        score += (limits.max_framebuffer_width / 1000).min(100);

        if self.supproted_features.geometry_shader == vk::TRUE {
            score += 50;
        }
        if self.supproted_features.tessellation_shader == vk::TRUE {
            score += 50;
        }
        if self.supproted_features.multi_draw_indirect == vk::TRUE {
            score += 50;
        }
        score
    }
}

impl std::fmt::Debug for PhysicalDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PhysicalDeviceInfo (\x1b[38;5;3m{:?}\x1b[0m) - \"\x1b[38;5;2m{:?}\x1b[0m\"",
            self.properties.device_type,
            unsafe { CStr::from_ptr(self.properties.device_name.as_ptr()) }
        )
    }
}
