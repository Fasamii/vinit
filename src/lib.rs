#![allow(unused)]
#![allow(dead_code)]

use ash::{self, khr, vk};
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;

mod command;
mod families;
mod mass;

pub trait Store<T> {
    type Stored;
}
pub struct Present;
impl<T> Store<T> for Present {
    type Stored = T;
}
pub struct Absent;
impl<T> Store<T> for Absent {
    type Stored = ();
}

type Field<S, T> = <S as Store<T>>::Stored;

struct InstanceInfo(ash::Instance);

/* TODO: Make some universal command pools struct which holds optionally all the command
pools using Field type */
pub struct Base<
    D: Store<DeviceInfo>,
    S: Store<SwapchainInfo>,
    CG: Store<command::CommandPoolInfo<families::Graphics>>,
    CC: Store<command::CommandPoolInfo<families::Compute>>,
    CT: Store<command::CommandPoolInfo<families::Transfer>>,
    CS: Store<command::CommandPoolInfo<families::Sparse>>,
    CP: Store<command::CommandPoolInfo<families::Protected>>,
> {
    swapchain: Field<S, SwapchainInfo>,
    cmd_graphics: Field<CG, command::CommandPoolInfo<families::Graphics>>,
    cmd_compute: Field<CC, command::CommandPoolInfo<families::Compute>>,
    cmd_transfer: Field<CT, command::CommandPoolInfo<families::Transfer>>,
    cmd_sparse: Field<CS, command::CommandPoolInfo<families::Sparse>>,
    cmd_protected: Field<CP, command::CommandPoolInfo<families::Protected>>,
    device: Field<D, DeviceInfo>,
    instance: InstanceInfo,
    entry: ash::Entry,
}

pub struct BaseConfig<
    D: Store<DeviceInfo>,
    S: Store<SwapchainInfo>,
    CG: Store<command::CommandPoolInfo<families::Graphics>>,
    CC: Store<command::CommandPoolInfo<families::Compute>>,
    CT: Store<command::CommandPoolInfo<families::Transfer>>,
    CS: Store<command::CommandPoolInfo<families::Sparse>>,
    CP: Store<command::CommandPoolInfo<families::Protected>>,
> {
    app_name: CString,
    version: (u32, u32, u32),
    instance_extensions: Vec<CString>,
    layer_extensions: Vec<CString>,
    device_extensions: Vec<CString>,
    required_queues: families::Families<bool>,
    physical_device: Option<PhysicalDeviceSelector>,
    swapchain: Option<SwapchainConfig>,
    command_pools: Vec<command::CommandPoolConfigFamily>,
    _has_device: PhantomData<D>,
    _has_swapchain: PhantomData<S>,
    _has_cmd_graphics: PhantomData<CG>,
    _has_cmd_compute: PhantomData<CC>,
    _has_cmd_transfer: PhantomData<CT>,
    _has_cmd_sparse: PhantomData<CS>,
    _has_cmd_protected: PhantomData<CP>,
}

impl Default for BaseConfig<Absent, Absent, Absent, Absent, Absent, Absent, Absent> {
    fn default() -> Self {
        Self {
            app_name: CString::from(c"No Name"),
            version: (0, 0, 0),
            instance_extensions: Default::default(),
            layer_extensions: Default::default(),
            device_extensions: Default::default(),
            required_queues: Default::default(),
            physical_device: None,
            swapchain: None,
            command_pools: Default::default(),
            _has_device: PhantomData,
            _has_swapchain: PhantomData,
            _has_cmd_graphics: PhantomData,
            _has_cmd_compute: PhantomData,
            _has_cmd_transfer: PhantomData,
            _has_cmd_sparse: PhantomData,
            _has_cmd_protected: PhantomData,
        }
    }
}

impl Drop for InstanceInfo {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy_instance(None);
        }
    }
}

impl<
    D: Store<DeviceInfo>,
    S: Store<SwapchainInfo>,
    CG: Store<command::CommandPoolInfo<families::Graphics>>,
    CC: Store<command::CommandPoolInfo<families::Compute>>,
    CT: Store<command::CommandPoolInfo<families::Transfer>>,
    CS: Store<command::CommandPoolInfo<families::Sparse>>,
    CP: Store<command::CommandPoolInfo<families::Protected>>,
> BaseConfig<D, S, CG, CC, CT, CS, CP>
{
    fn cast<
        D2: Store<DeviceInfo>,
        S2: Store<SwapchainInfo>,
        CG2: Store<command::CommandPoolInfo<families::Graphics>>,
        CC2: Store<command::CommandPoolInfo<families::Compute>>,
        CT2: Store<command::CommandPoolInfo<families::Transfer>>,
        CS2: Store<command::CommandPoolInfo<families::Sparse>>,
        CP2: Store<command::CommandPoolInfo<families::Protected>>,
    >(
        self,
    ) -> BaseConfig<D2, S2, CG2, CC2, CT2, CS2, CP2> {
        BaseConfig {
            app_name: self.app_name,
            version: self.version,
            instance_extensions: self.instance_extensions,
            layer_extensions: self.layer_extensions,
            device_extensions: self.device_extensions,
            required_queues: self.required_queues,
            physical_device: self.physical_device,
            swapchain: self.swapchain,
            command_pools: self.command_pools,
            _has_device: PhantomData,
            _has_swapchain: PhantomData,
            _has_cmd_graphics: PhantomData,
            _has_cmd_compute: PhantomData,
            _has_cmd_transfer: PhantomData,
            _has_cmd_sparse: PhantomData,
            _has_cmd_protected: PhantomData,
        }
    }
}

impl<
    D: Store<DeviceInfo, Stored = DeviceInfo>,
    S: Store<SwapchainInfo>,
    CG: Store<command::CommandPoolInfo<families::Graphics>>,
    CC: Store<command::CommandPoolInfo<families::Compute>>,
    CT: Store<command::CommandPoolInfo<families::Transfer>>,
    CS: Store<command::CommandPoolInfo<families::Sparse>>,
    CP: Store<command::CommandPoolInfo<families::Protected>>,
> BaseConfig<D, S, CG, CC, CT, CS, CP>
where
    D: BuildDevice<D>,
    S: BuildSwapchain<S>,
{
    pub fn build(mut self) -> Result<Base<D, S, CG, CC, CT, CS, CP>, vk::Result> {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Entry") };
        let app_info = vk::ApplicationInfo::default()
            .application_name(self.app_name.as_c_str())
            .application_version(vk::make_api_version(
                0,
                self.version.0,
                self.version.1,
                self.version.2,
            ));
        let instance_extensions_raw: Vec<*const i8> = self
            .instance_extensions
            .iter()
            .map(|name| name.as_ptr())
            .collect();

        let layer_names_raw: Vec<*const i8> = self
            .layer_extensions
            .iter()
            .map(|name| name.as_ptr())
            .collect();

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&instance_extensions_raw)
            .enabled_layer_names(&layer_names_raw);

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };

        // TODO: Pass self.device_extensions via reference and with array instead of vector also
        // convert into &CStr
        let device = D::build_device(
            self.physical_device,
            &instance,
            self.device_extensions,
            self.required_queues,
        )?;

        let swapchain = S::build_swapchain(self.swapchain, &instance, &device).unwrap();

        Ok(Base {
            swapchain,
            device,
            instance: InstanceInfo(instance),
            entry,
            cmd_graphics: todo!(),
            cmd_compute: todo!(),
            cmd_transfer: todo!(),
            cmd_sparse: todo!(),
            cmd_protected: todo!(),
        })
    }
}

impl<
    D: Store<DeviceInfo>,
    S: Store<SwapchainInfo>,
    CG: Store<command::CommandPoolInfo<families::Graphics>>,
    CC: Store<command::CommandPoolInfo<families::Compute>>,
    CT: Store<command::CommandPoolInfo<families::Transfer>>,
    CS: Store<command::CommandPoolInfo<families::Sparse>>,
    CP: Store<command::CommandPoolInfo<families::Protected>>,
> BaseConfig<D, S, CG, CC, CT, CS, CP>
{
    pub fn with_app_name(mut self, name: CString) -> Self {
        self.app_name = name;
        self
    }
    pub fn with_app_version(mut self, version: (u32, u32, u32)) -> Self {
        self.version = version;
        self
    }
    pub fn with_instance_extensions(mut self, extensions: Vec<CString>) -> Self {
        self.instance_extensions = extensions;
        self
    }
    pub fn with_device(
        mut self,
        configure: fn(PhysicalDeviceSelector) -> PhysicalDeviceSelector,
    ) -> BaseConfig<Present, S, CG, CC, CT, CS, CP> {
        self.physical_device = Some(configure(Default::default()));
        self.cast()
    }
    pub fn with_validation_layers(mut self, extensions: Vec<CString>) -> Self {
        self.instance_extensions
            .push(CString::from(c"VK_EXT_debug_utils"));
        self.layer_extensions = extensions;
        self
    }
}

impl<
    S: Store<SwapchainInfo>,
    CG: Store<command::CommandPoolInfo<families::Graphics>>,
    CC: Store<command::CommandPoolInfo<families::Compute>>,
    CT: Store<command::CommandPoolInfo<families::Transfer>>,
    CS: Store<command::CommandPoolInfo<families::Sparse>>,
    CP: Store<command::CommandPoolInfo<families::Protected>>,
> BaseConfig<Present, S, CG, CC, CT, CS, CP>
{
    pub fn with_device_extensions(mut self, extensions: Vec<CString>) -> Self {
        self.device_extensions = extensions;
        self
    }

    // TODO: turn that into lambda function like in with_device and with_swapchain
    pub fn add_command_pool<Q: families::QueueFamily>(
        mut self,
        configure: fn(command::CommandPoolConfig<Q>) -> command::CommandPoolConfig<Q>,
    ) -> Self
    where
        command::CommandPoolConfigFamily: From<command::CommandPoolConfig<Q>>,
    {
        self.required_queues.set::<Q>(true);
        let config = configure(command::CommandPoolConfig::default());
        self.command_pools.push(config.into());
        self
    }
}
impl<
    CG: Store<command::CommandPoolInfo<families::Graphics>>,
    CC: Store<command::CommandPoolInfo<families::Compute>>,
    CT: Store<command::CommandPoolInfo<families::Transfer>>,
    CS: Store<command::CommandPoolInfo<families::Sparse>>,
    CP: Store<command::CommandPoolInfo<families::Protected>>,
> BaseConfig<Present, Absent, CG, CC, CT, CS, CP>
{
    pub fn with_swapchain(
        mut self,
        configure: fn(SwapchainConfig) -> SwapchainConfig,
    ) -> BaseConfig<Present, Present, CG, CC, CT, CS, CP> {
        self.swapchain = Some(configure(Default::default()));
        self.cast()
    }
}

pub trait BuildDevice<S: Store<DeviceInfo>> {
    fn build_device(
        config: Option<PhysicalDeviceSelector>,
        instance: &ash::Instance,
        extensions: Vec<CString>, // TODO: Convert that earlier into &CStr and pass with []
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
            .expect("Implement - error handling")
            .require_extensions(extensions)
            .require_queues(required_queues)
            .select(instance)
            .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
        let device_info = DeviceInfo::new(physical_device_info, required_queues, instance);
        Ok(device_info)
    }
}

// TODO: consider removing T and hardcoding vk::Queue
type QueueHandles<T> = families::Families<T>;

impl QueueHandles<Option<vk::Queue>> {
    fn new(device: &ash::Device, indices: families::Families<Option<u32>>) -> Self {
        let graphics = indices
            .graphics
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let compute = indices
            .compute
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let transfer = indices
            .transfer
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let sparse = indices
            .sparse
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let protected = indices
            .protected
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });

        Self {
            graphics,
            compute,
            transfer,
            sparse,
            protected,
        }
    }
}

pub struct DeviceInfo {
    device: ash::Device,
    physical_info: PhysicalDeviceInfo,
    queue_handles: QueueHandles<Option<vk::Queue>>,
}

impl DeviceInfo {
    fn new(
        physical_device_info: PhysicalDeviceInfo,
        required_queues: families::Families<bool>,
        instance: &ash::Instance,
    ) -> Self {
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
            instance
                .create_device(
                    physical_device_info.physical_device,
                    &device_create_info,
                    None,
                )
                .expect("Implement - error handling")
        };
        let queue_handles = QueueHandles::new(&device, required_queue_family_indices);
        Self {
            device,
            physical_info: physical_device_info,
            queue_handles,
        }
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
    fn select(&self, instance: &ash::Instance) -> Option<PhysicalDeviceInfo> {
        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
        let suitable_devices: Vec<PhysicalDeviceInfo> = physical_devices
            .into_iter()
            .map(|physical_device| {
                PhysicalDeviceInfo::new(
                    physical_device,
                    self.required_features,
                    self.required_extensions.clone(), // TODO: Remove that clone
                    instance,
                )
            })
            .filter(|info| !self.require_discrete || info.is_discrete())
            .filter(|info| info.satisfies_families(self.required_queues))
            .filter(|info| info.satisfies_extensions(&self.required_extensions))
            .filter(|info| info.satisfies_properties(self.required_properties))
            .filter(|info| info.satisfies_features(self.required_features))
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
    ) -> Self {
        Self {
            physical_device,
            queue_families_indices: families::Families::query(instance, physical_device),
            properties: Self::get_properties(instance, physical_device),
            memory_properties: Self::get_memory(instance, physical_device),
            enabled_features,
            supproted_features: Self::get_features(instance, physical_device),
            enabled_extensions,
            supported_extensions: Self::get_extensions(instance, physical_device),
        }
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
    ) -> Vec<vk::ExtensionProperties> {
        unsafe {
            instance
                .enumerate_device_extension_properties(physical_device)
                .unwrap()
        }
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

pub trait BuildSwapchain<S: Store<SwapchainInfo>> {
    fn build_swapchain(
        config: Option<SwapchainConfig>,
        instance: &ash::Instance,
        device: &DeviceInfo,
    ) -> Result<S::Stored, ()>;
}

impl BuildSwapchain<Absent> for Absent {
    fn build_swapchain(
        _config: Option<SwapchainConfig>,
        _instance: &ash::Instance,
        _device: &DeviceInfo,
    ) -> Result<(), ()> {
        Ok(())
    }
}

impl BuildSwapchain<Present> for Present {
    fn build_swapchain(
        config: Option<SwapchainConfig>,
        instance: &ash::Instance,
        device: &DeviceInfo,
    ) -> Result<SwapchainInfo, ()> {
        todo!("Implement building swapchain")
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
            min_image_count: 2,
            image_format: vk::Format::R8G8B8A8_SRGB,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            present_mode: vk::PresentModeKHR::FIFO,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            array_layers: 1,
            extent: vk::Extent2D {
                width: 1920,
                height: 1080,
            },
            clipped: true,
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

impl SwapchainConfig {
    fn build() -> SwapchainInfo {
        SwapchainInfo {
            swapchain: todo!(),
            images: todo!(),
            image_views: todo!(),
            format: todo!(),
            extent: todo!(),
            image_count: todo!(),
        }
    }
}

#[derive(Debug)]
pub struct SwapchainInfo {
    pub swapchain: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    pub image_count: u32,
}
