#![allow(unused)]
#![allow(dead_code)]

use ash::{self, khr, vk};
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::sync::Arc;

pub mod command;
pub mod device;
pub mod families;
mod mass;
pub mod swapchain;

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

pub struct Base<D, S, CG, CC, CT, CS, CP>
where
    D: Store<device::DeviceInfo>,
    S: Store<swapchain::SwapchainInfo>,
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    swapchain: Field<S, swapchain::SwapchainInfo>,
    command_pools: command::CommandPools<CG, CC, CT, CS, CP>,
    device: Field<D, device::DeviceInfo>,
    instance: InstanceInfo,
    entry: ash::Entry,
}

pub struct BaseConfig<D, S, CG, CC, CT, CS, CP>
where
    D: Store<device::DeviceInfo>,
    S: Store<swapchain::SwapchainInfo>,
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    app_name: CString,
    version: (u32, u32, u32),
    instance_extensions: Vec<CString>,
    layer_extensions: Vec<CString>,
    device_extensions: Vec<CString>,
    required_queues: families::Families<bool>,
    physical_device: Option<device::PhysicalDeviceSelector>,
    swapchain: Option<swapchain::SwapchainConfig>,
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

impl<D, S, CG, CC, CT, CS, CP> BaseConfig<D, S, CG, CC, CT, CS, CP>
where
    D: Store<device::DeviceInfo>,
    S: Store<swapchain::SwapchainInfo>,
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    fn cast<
        D2: Store<device::DeviceInfo>,
        S2: Store<swapchain::SwapchainInfo>,
        CG2: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
        CC2: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
        CT2: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
        CS2: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
        CP2: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
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

impl<D, S, CG, CC, CT, CS, CP> BaseConfig<D, S, CG, CC, CT, CS, CP>
where
    // TODO: You cant create Base without device because Stored = DeviceInfo, fix that
    D: Store<device::DeviceInfo, Stored = device::DeviceInfo> + device::BuildDevice<D>,
    S: Store<swapchain::SwapchainInfo> + swapchain::BuildSwapchain<S>,
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>
        + command::BuildCommandPools<families::Graphics, CG>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>
        + command::BuildCommandPools<families::Compute, CC>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>
        + command::BuildCommandPools<families::Transfer, CT>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>
        + command::BuildCommandPools<families::Sparse, CS>,
    CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>
        + command::BuildCommandPools<families::Protected, CP>,
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

        let instance = unsafe { entry.create_instance(&instance_create_info, None)? };

        // TODO: Pass self.device_extensions via reference and with array instead of vector also
        // convert into &CStr
        let device = D::build_device(
            self.physical_device,
            &instance,
            self.device_extensions,
            self.required_queues,
        )?;

        let command_pools = Self::build_command_pools(&instance, &device, &self.command_pools)?;

        let swapchain = S::build_swapchain(self.swapchain, &instance, &device)?;

        Ok(Base {
            swapchain,
            command_pools,
            device,
            instance: InstanceInfo(instance),
            entry,
        })
    }

    fn build_command_pools(
        instance: &ash::Instance,
        device: &device::DeviceInfo,
        command_pools_configs: &[command::CommandPoolConfigFamily],
    ) -> Result<command::CommandPools<CG, CC, CT, CS, CP>, vk::Result> {
        let graphics_configs: Vec<_> = command_pools_configs
            .iter()
            .filter_map(|cfg| cfg.get_graphics())
            .collect();

        let compute_configs: Vec<_> = command_pools_configs
            .iter()
            .filter_map(|cfg| cfg.get_compute())
            .collect();

        let transfer_configs: Vec<_> = command_pools_configs
            .iter()
            .filter_map(|cfg| cfg.get_transfer())
            .collect();

        let sparse_configs: Vec<_> = command_pools_configs
            .iter()
            .filter_map(|cfg| cfg.get_sparse())
            .collect();

        let protected_configs: Vec<_> = command_pools_configs
            .iter()
            .filter_map(|cfg| cfg.get_protected())
            .collect();

        let graphics = CG::build_pools(graphics_configs, device)?;
        let compute = CC::build_pools(compute_configs, device)?;
        let transfer = CT::build_pools(transfer_configs, device)?;
        let sparse = CS::build_pools(sparse_configs, device)?;
        let protected = CP::build_pools(protected_configs, device)?;

        Ok(command::CommandPools {
            graphics,
            compute,
            transfer,
            sparse,
            protected,
        })
    }
}

impl<D, S, CG, CC, CT, CS, CP> BaseConfig<D, S, CG, CC, CT, CS, CP>
where
    D: Store<device::DeviceInfo>,
    S: Store<swapchain::SwapchainInfo>,
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
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
        configure: fn(device::PhysicalDeviceSelector) -> device::PhysicalDeviceSelector,
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

impl<S, CG, CC, CT, CS, CP> BaseConfig<Present, S, CG, CC, CT, CS, CP>
where
    S: Store<swapchain::SwapchainInfo>,
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    pub fn with_device_extensions(mut self, extensions: Vec<CString>) -> Self {
        self.device_extensions = extensions;
        self
    }

    pub fn with_graphics_pool(
        mut self,
        configure: fn(
            command::CommandPoolConfig<families::Graphics>,
        ) -> command::CommandPoolConfig<families::Graphics>,
    ) -> BaseConfig<Present, S, Present, CC, CT, CS, CP> {
        self.required_queues.set::<families::Graphics>(true);
        let config = configure(command::CommandPoolConfig::default());
        self.command_pools.push(config.into());
        self.cast()
    }

    pub fn with_compute_pool(
        mut self,
        configure: fn(
            command::CommandPoolConfig<families::Compute>,
        ) -> command::CommandPoolConfig<families::Compute>,
    ) -> BaseConfig<Present, S, CG, Present, CT, CS, CP> {
        self.required_queues.set::<families::Compute>(true);
        let config = configure(command::CommandPoolConfig::default());
        self.command_pools.push(config.into());
        self.cast()
    }

    pub fn with_transfer_pool(
        mut self,
        configure: fn(
            command::CommandPoolConfig<families::Transfer>,
        ) -> command::CommandPoolConfig<families::Transfer>,
    ) -> BaseConfig<Present, S, CG, CC, Present, CS, CP> {
        self.required_queues.set::<families::Transfer>(true);
        let config = configure(command::CommandPoolConfig::default());
        self.command_pools.push(config.into());
        self.cast()
    }

    pub fn with_sparse_pool(
        mut self,
        configure: fn(
            command::CommandPoolConfig<families::Sparse>,
        ) -> command::CommandPoolConfig<families::Sparse>,
    ) -> BaseConfig<Present, S, CG, CC, CT, Present, CP> {
        self.required_queues.set::<families::Sparse>(true);
        let config = configure(command::CommandPoolConfig::default());
        self.command_pools.push(config.into());
        self.cast()
    }

    pub fn with_protected_pool(
        mut self,
        configure: fn(
            command::CommandPoolConfig<families::Protected>,
        ) -> command::CommandPoolConfig<families::Protected>,
    ) -> BaseConfig<Present, S, CG, CC, CT, CS, Present> {
        self.required_queues.set::<families::Protected>(true);
        let config = configure(command::CommandPoolConfig::default());
        self.command_pools.push(config.into());
        self.cast()
    }
}
impl<CG, CC, CT, CS, CP> BaseConfig<Present, Absent, CG, CC, CT, CS, CP>
where
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    pub fn with_swapchain(
        mut self,
        configure: fn(swapchain::SwapchainConfig) -> swapchain::SwapchainConfig,
    ) -> BaseConfig<Present, Present, CG, CC, CT, CS, CP> {
        self.swapchain = Some(configure(Default::default()));
        self.cast()
    }
}
