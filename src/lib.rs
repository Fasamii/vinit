#![allow(unused)]
#![allow(dead_code)]

use ash::{self, khr, vk};
use core::fmt;
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::sync::Arc;

pub mod instance;

pub mod command;
pub mod device;
pub mod families;
mod mass;
pub mod swapchain;

pub trait Store<C, I> {
    type StoredConfig;
    type StoredInfo;
}
pub struct Present;
impl<C, I> Store<C, I> for Present {
    type StoredConfig = C;
    type StoredInfo = I;
}
pub struct Absent;
impl<C, I> Store<C, I> for Absent {
    type StoredConfig = ();
    type StoredInfo = ();
}

type FieldConfig<S, C, I> = <S as Store<C, I>>::StoredConfig;
type FieldInfo<S, C, I> = <S as Store<C, I>>::StoredInfo;

pub trait Apply<For> {
    type Out;
    fn apply(self, config: For) -> Self::Out;
}

pub struct Satisfied;
pub struct Unsatisfied;
pub trait SatisfiesDeps<DepTuple> {
    type Satisfied;
}

impl SatisfiesDeps<()> for () {
    type Satisfied = Unsatisfied;
}

impl SatisfiesDeps<(instance::InstanceInfo)> for () {
    type Satisfied = Satisfied;
}

pub struct Base<I, D>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    // S: Store<swapchain::SwapchainInfo>,
    // CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    // CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    // CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    // CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    // CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    // swapchain: FieldInfo<S, swapchain::Swapchain, swapchain::SwapchainInfo>,
    // command_pools: command::CommandPools<CG, CC, CT, CS, CP>,
    device: FieldInfo<D, device::Device, device::DeviceInfo>,
    instance: FieldInfo<I, instance::Instance, instance::InstanceInfo>,
    entry: ash::Entry,
}

pub struct BaseConfig<I, D>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    // S: Store<swapchain::SwapchainInfo>,
    // CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    // CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    // CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    // CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    // CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    instance: FieldConfig<I, instance::Instance, instance::InstanceInfo>,
    device: FieldConfig<D, device::Device, device::DeviceInfo>,

    required_queues: families::Families<bool>,
    // device_extensions: Vec<CString>,
    // physical_device: Option<device::PhysicalDeviceSelector>,
    // swapchain: Option<swapchain::SwapchainConfig>,
    // command_pools: Vec<command::CommandPoolConfigFamily>,
    // _has_device: PhantomData<D>,
    // _has_swapchain: PhantomData<S>,
    // _has_cmd_graphics: PhantomData<CG>,
    // _has_cmd_compute: PhantomData<CC>,
    // _has_cmd_transfer: PhantomData<CT>,
    // _has_cmd_sparse: PhantomData<CS>,
    // _has_cmd_protected: PhantomData<CP>,
}

impl Default for BaseConfig<Absent, Absent> {
    fn default() -> Self {
        Self {
            instance: (),
            device: (),

            required_queues: Default::default(),
            // required_queues: Default::default(),
            // physical_device: None,
            // swapchain: None,
            // command_pools: Default::default(),
            // _has_device: PhantomData,
            // _has_swapchain: PhantomData,
            // _has_cmd_graphics: PhantomData,
            // _has_cmd_compute: PhantomData,
            // _has_cmd_transfer: PhantomData,
            // _has_cmd_sparse: PhantomData,
            // _has_cmd_protected: PhantomData,
        }
    }
}

impl<I, D> BaseConfig<I, D>
where
    I: Store<instance::Instance, instance::InstanceInfo> + instance::CreateInstance<I>,
    D: Store<device::Device, device::DeviceInfo> + device::CreateDevice<D, I>,
    // S: Store<swapchain::SwapchainInfo> + swapchain::BuildSwapchain<S>,
    // CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>
    //     + command::BuildCommandPools<families::Graphics, CG>,
    // CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>
    //     + command::BuildCommandPools<families::Compute, CC>,
    // CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>
    //     + command::BuildCommandPools<families::Transfer, CT>,
    // CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>
    //     + command::BuildCommandPools<families::Sparse, CS>,
    // CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>
    //     + command::BuildCommandPools<families::Protected, CP>,
{
    pub fn build(mut self) -> Result<Base<I, D>, vk::Result> {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Entry") };
        let instance = I::create(self.instance, &entry)?;
        let device = D::create(self.device, &instance, self.required_queues)?;

        // TODO: Pass self.device_extensions via reference and with array instead of vector also
        // convert into &CStr
        // let device = D::build_device(
        //     self.physical_device,
        //     &tmp_instance,
        //     self.device_extensions,
        //     self.required_queues,
        // )?;

        // let command_pools = Self::build_command_pools(&tmp_instance, &device, &self.command_pools)?;

        // let swapchain = S::build_swapchain(self.swapchain, &tmp_instance, &device)?;

        Ok(Base {
            device,
            instance,
            entry,
        })
    }

    // fn build_command_pools(
    //     instance: &ash::Instance,
    //     device: &device::DeviceInfo,
    //     command_pools_configs: &[command::CommandPoolConfigFamily],
    // ) -> Result<command::CommandPools<CG, CC, CT, CS, CP>, vk::Result> {
    //     let graphics_configs: Vec<_> = command_pools_configs
    //         .iter()
    //         .filter_map(|cfg| cfg.get_graphics())
    //         .collect();
    //
    //     let compute_configs: Vec<_> = command_pools_configs
    //         .iter()
    //         .filter_map(|cfg| cfg.get_compute())
    //         .collect();
    //
    //     let transfer_configs: Vec<_> = command_pools_configs
    //         .iter()
    //         .filter_map(|cfg| cfg.get_transfer())
    //         .collect();
    //
    //     let sparse_configs: Vec<_> = command_pools_configs
    //         .iter()
    //         .filter_map(|cfg| cfg.get_sparse())
    //         .collect();
    //
    //     let protected_configs: Vec<_> = command_pools_configs
    //         .iter()
    //         .filter_map(|cfg| cfg.get_protected())
    //         .collect();
    //
    //     let graphics = CG::build_pools(graphics_configs, device)?;
    //     let compute = CC::build_pools(compute_configs, device)?;
    //     let transfer = CT::build_pools(transfer_configs, device)?;
    //     let sparse = CS::build_pools(sparse_configs, device)?;
    //     let protected = CP::build_pools(protected_configs, device)?;
    //
    //     Ok(command::CommandPools {
    //         graphics,
    //         compute,
    //         transfer,
    //         sparse,
    //         protected,
    //     })
    // }
}

impl<I, D> BaseConfig<I, D>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    // S: Store<swapchain::SwapchainInfo>,
    // CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    // CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    // CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    // CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    // CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
{
    pub fn with<T: Apply<Self>>(self, opt: T) -> T::Out {
        opt.apply(self)
    }
}

//     pub fn with_device(
//         mut self,
//         configure: fn(device::PhysicalDeviceSelector) -> device::PhysicalDeviceSelector,
//     ) -> BaseConfig<Present, S, CG, CC, CT, CS, CP> {
//         self.physical_device = Some(configure(Default::default()));
//         self.cast()
//     }
//     pub fn with_validation_layers(mut self, extensions: Vec<CString>) -> Self {
//         self.instance_extensions
//             .push(CString::from(c"VK_EXT_debug_utils"));
//         self.layer_extensions = extensions;
//         self
//     }
// }
//
// impl<S, CG, CC, CT, CS, CP> BaseConfig<Present, S, CG, CC, CT, CS, CP>
// where
//     S: Store<swapchain::SwapchainInfo>,
//     CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
//     CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
//     CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
//     CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
//     CP: Store<Vec<command::CommandPoolInfo<families::Protected>>>,
// {
//     pub fn with_device_extensions(mut self, extensions: Vec<CString>) -> Self {
//         self.device_extensions = extensions;
//         self
//     }
//
//     pub fn with_graphics_pool(
//         mut self,
//         configure: fn(
//             command::CommandPoolConfig<families::Graphics>,
//         ) -> command::CommandPoolConfig<families::Graphics>,
//     ) -> BaseConfig<Present, S, Present, CC, CT, CS, CP> {
//         self.required_queues.set::<families::Graphics>(true);
//         let config = configure(command::CommandPoolConfig::default());
//         self.command_pools.push(config.into());
//         self.cast()
//     }
//
//     pub fn with_compute_pool(
//         mut self,
//         configure: fn(
//             command::CommandPoolConfig<families::Compute>,
//         ) -> command::CommandPoolConfig<families::Compute>,
//     ) -> BaseConfig<Present, S, CG, Present, CT, CS, CP> {
//         self.required_queues.set::<families::Compute>(true);
//         let config = configure(command::CommandPoolConfig::default());
//         self.command_pools.push(config.into());
//         self.cast()
//     }
//
//     pub fn with_transfer_pool(
//         mut self,
//         configure: fn(
//             command::CommandPoolConfig<families::Transfer>,
//         ) -> command::CommandPoolConfig<families::Transfer>,
//     ) -> BaseConfig<Present, S, CG, CC, Present, CS, CP> {
//         self.required_queues.set::<families::Transfer>(true);
//         let config = configure(command::CommandPoolConfig::default());
//         self.command_pools.push(config.into());
//         self.cast()
//     }
//
//     pub fn with_sparse_pool(
//         mut self,
//         configure: fn(
//             command::CommandPoolConfig<families::Sparse>,
//         ) -> command::CommandPoolConfig<families::Sparse>,
//     ) -> BaseConfig<Present, S, CG, CC, CT, Present, CP> {
//         self.required_queues.set::<families::Sparse>(true);
//         let config = configure(command::CommandPoolConfig::default());
//         self.command_pools.push(config.into());
//         self.cast()
//     }
//
//     pub fn with_protected_pool(
//         mut self,
//         configure: fn(
//             command::CommandPoolConfig<families::Protected>,
//         ) -> command::CommandPoolConfig<families::Protected>,
//     ) -> BaseConfig<Present, S, CG, CC, CT, CS, Present> {
//         self.required_queues.set::<families::Protected>(true);
//         let config = configure(command::CommandPoolConfig::default());
//         self.command_pools.push(config.into());
//         self.cast()
//     }
// }

impl<I, D> fmt::Debug for Base<I, D>
where
    I: Store<instance::Instance, instance::InstanceInfo, StoredInfo = dyn fmt::Debug>,
    D: Store<device::Device, device::DeviceInfo, StoredInfo = dyn fmt::Debug>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Base")
            .field("Instance", &self.instance)
            .field("Device", &self.device)
            .finish()
    }
}
