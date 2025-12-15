use crate::command;
use crate::families;
use crate::instance;
use crate::mass;
use crate::{Absent, Present, Store};
use crate::{Apply, BaseConfig};
use crate::{SatisfiesDeps, Unsatisfied};
use ash::vk;
use core::fmt;
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::sync::Arc;

pub trait CreateDevice<D, I>
where
    D: Store<Device, DeviceInfo>,
    I: Store<instance::Instance, instance::InstanceInfo>,
{
    fn create(
        config: D::StoredConfig,
        instance: &I::StoredInfo,
        required_queues: families::Families<bool>,
    ) -> Result<D::StoredInfo, vk::Result>;
}

impl<I> CreateDevice<Absent, I> for Absent
where
    I: Store<instance::Instance, instance::InstanceInfo>,
{
    fn create(
        _config: (),
        _instance: &I::StoredInfo,
        _required_queues: families::Families<bool>,
    ) -> Result<(), vk::Result> {
        Ok(())
    }
}

// NOTE: Removed in order to keep errors compile time
// impl<I> CreateDevice<Present, I> for Present
// where
//     I: Store<instance::Instance, instance::InstanceInfo>,
//     (): SatisfiesDeps<I, Satisfied = Unsatisfied>,
// {
//     fn create(
//         _config: Device,
//         _instance: &I::StoredInfo,
//         _required_queues: families::Families<bool>,
//     ) -> Result<DeviceInfo, vk::Result> {
//         Err(vk::Result::ERROR_INITIALIZATION_FAILED)
//     }
// }

impl CreateDevice<Present, Present> for Present {
    fn create(
        config: Device,
        instance: &instance::InstanceInfo,
        required_queues: families::Families<bool>,
    ) -> Result<DeviceInfo, vk::Result> {
        config.create(required_queues, &instance.0)
    }
}

pub struct DeviceInfo {
    pub device: Arc<ash::Device>,
    pub physical: PhysicalDeviceInfo,
    pub queue_handles: families::Families<Option<vk::Queue>>,
}

impl DeviceInfo {
    fn new(
        physical: PhysicalDeviceInfo,
        required_queues: families::Families<bool>,
        instance: &ash::Instance,
    ) -> Result<Self, vk::Result> {
        let required_queue_family_indices = physical
            .queue_families_indices
            .filter_required(&required_queues);
        let queue_create_info = required_queue_family_indices.make_create_info(&[1.0f32]);

        let extension_ptrs: Vec<*const i8> = physical
            .enabled_extensions
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        let device_create_info = vk::DeviceCreateInfo::default()
            .enabled_features(&physical.enabled_features)
            .enabled_extension_names(&extension_ptrs)
            .queue_create_infos(&queue_create_info);

        log::info!(
            "device_create_info = {device_create_info:#?}\n queue_create_info = {queue_create_info:#?}"
        );

        let device =
            unsafe { instance.create_device(physical.physical_device, &device_create_info, None)? };
        let queue_handles: families::Families<Option<vk::Queue>> =
            families::Families::new(&device, required_queue_family_indices);
        Ok(Self {
            device: Arc::new(device),
            physical,
            queue_handles,
        })
    }
}

impl fmt::Debug for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceInfo")
            .field("device", &self.device.handle())
            .field("physical", &self.physical)
            .finish()
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

pub struct Device {
    prefer_best: bool,
    require_discrete: bool,
    required_properties: vk::PhysicalDeviceProperties,
    required_features: vk::PhysicalDeviceFeatures,
    required_extensions: HashSet<CString>,

    required_queues: families::Families<bool>,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            prefer_best: true,
            require_discrete: false,
            required_queues: Default::default(),
            required_properties: Default::default(),
            required_features: Default::default(),
            required_extensions: HashSet::new(),
        }
    }
}

impl Device {
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

    pub fn require_extensions(mut self, extensions: HashSet<CString>) -> Self {
        self.required_extensions = extensions;
        self
    }

    fn require_queues(mut self, queues: families::Families<bool>) -> Self {
        self.required_queues = queues;
        self
    }
}

impl Device {
    fn create(
        self,
        required_queues: families::Families<bool>,
        instance: &ash::Instance,
    ) -> Result<DeviceInfo, vk::Result> {
        let physical_device = self
            .require_queues(required_queues)
            .select(instance)?
            .ok_or(vk::Result::ERROR_FEATURE_NOT_PRESENT)?;

        DeviceInfo::new(physical_device, required_queues, instance)
    }

    fn select(&self, instance: &ash::Instance) -> Result<Option<PhysicalDeviceInfo>, vk::Result> {
        let physical_device_handles = unsafe { instance.enumerate_physical_devices()? };
        let physical_device_infos: Vec<PhysicalDeviceInfo> = physical_device_handles
            .into_iter()
            .map(|physical_device| {
                PhysicalDeviceInfo::new(
                    physical_device,
                    self.required_properties,
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

    pub properties: vk::PhysicalDeviceProperties,
    pub enabled_properties: vk::PhysicalDeviceProperties,
    pub supported_features: vk::PhysicalDeviceFeatures,
    pub enabled_features: vk::PhysicalDeviceFeatures,
    pub supported_extensions: HashSet<CString>,
    pub enabled_extensions: HashSet<CString>,

    pub limits: vk::PhysicalDeviceLimits,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,

    pub queue_families_indices: families::Families<Option<u32>>,
}

impl PhysicalDeviceInfo {
    fn new(
        physical_device: vk::PhysicalDevice,
        enabled_properties: vk::PhysicalDeviceProperties,
        enabled_features: vk::PhysicalDeviceFeatures,
        enabled_extensions: HashSet<CString>,
        instance: &ash::Instance,
    ) -> Result<Self, vk::Result> {
        let properties = Self::get_properties(instance, physical_device);
        let limits = properties.limits;
        Ok(Self {
            physical_device,

            properties,
            enabled_properties,

            supported_features: Self::get_features(instance, physical_device),
            enabled_features,

            supported_extensions: Self::get_extensions(instance, physical_device)?,
            enabled_extensions,

            limits,
            memory_properties: Self::get_memory(instance, physical_device),

            queue_families_indices: families::Families::query(instance, physical_device),
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
    ) -> Result<HashSet<CString>, vk::Result> {
        let extension_properties =
            unsafe { instance.enumerate_device_extension_properties(physical_device)? };
        let mut extensions = HashSet::with_capacity(extension_properties.len());

        for extension in extension_properties {
            let name = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) };

            extensions.insert(name.to_owned());
        }
        Ok(extensions)
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
        mass::satisifes_features(&self.supported_features, &features)
    }

    fn satisfies_extensions(&self, extensions: &HashSet<CString>) -> bool {
        let available: HashSet<&CStr> = self
            .supported_extensions
            .iter()
            .map(|extension| unsafe { CStr::from_ptr(extension.as_ptr()) })
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

        if self.supported_features.geometry_shader == vk::TRUE {
            score += 50;
        }
        if self.supported_features.tessellation_shader == vk::TRUE {
            score += 50;
        }
        if self.supported_features.multi_draw_indirect == vk::TRUE {
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

impl<CG, CC, CT, CS, CP> Apply<BaseConfig<Present, Absent, CG, CC, CT, CS, CP>> for Device
where
    CG: Store<
        Vec<command::CommandPool<families::Graphics>>,
        Vec<command::CommandPoolInfo<families::Graphics>>,
    >,
    CC: Store<
        Vec<command::CommandPool<families::Compute>>,
        Vec<command::CommandPoolInfo<families::Compute>>,
    >,
    CT: Store<
        Vec<command::CommandPool<families::Transfer>>,
        Vec<command::CommandPoolInfo<families::Transfer>>,
    >,
    CS: Store<
        Vec<command::CommandPool<families::Sparse>>,
        Vec<command::CommandPoolInfo<families::Sparse>>,
    >,
    CP: Store<
        Vec<command::CommandPool<families::Protected>>,
        Vec<command::CommandPoolInfo<families::Protected>>,
    >,
{
    type Out = BaseConfig<Present, Present, CG, CC, CT, CS, CP>;
    fn apply(self, config: BaseConfig<Present, Absent, CG, CC, CT, CS, CP>) -> Self::Out {
        BaseConfig {
            instance: config.instance,
            device: self,
            required_queues: config.required_queues,
            pools: config.pools,
        }
    }
}
