use crate::command;
use crate::device;
use crate::families;
use crate::instance;
use crate::Base;
use crate::{Present, Store};

impl<D, CG, CC, CT, CS, CP> Base<Present, D, CG, CC, CT, CS, CP>
where
    D: Store<device::Device, device::DeviceInfo>,
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
    pub fn instance(&self) -> &instance::InstanceInfo {
        &self.instance
    }
    
    pub fn instance_mut(&mut self) -> &mut instance::InstanceInfo {
        &mut self.instance
    }
}

// Device accessors - only when D = Present
impl<I, CG, CC, CT, CS, CP> Base<I, Present, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
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
    pub fn device(&self) -> &device::DeviceInfo {
        &self.device
    }
    
    pub fn device_mut(&mut self) -> &mut device::DeviceInfo {
        &mut self.device
    }
}

impl<I, D, CC, CT, CS, CP> Base<I, D, Present, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
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
    pub fn graphics_pools(&self) -> &Vec<command::CommandPoolInfo<families::Graphics>> {
        &self.pools.graphics
    }
    
    pub fn graphics_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Graphics>> {
        &mut self.pools.graphics
    }
}

impl<I, D, CG, CT, CS, CP> Base<I, D, CG, Present, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    CG: Store<
        Vec<command::CommandPool<families::Graphics>>,
        Vec<command::CommandPoolInfo<families::Graphics>>,
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
    pub fn compute_pools(&self) -> &Vec<command::CommandPoolInfo<families::Compute>> {
        &self.pools.compute
    }
    
    pub fn compute_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Compute>> {
        &mut self.pools.compute
    }
}

impl<I, D, CG, CC, CS, CP> Base<I, D, CG, CC, Present, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    CG: Store<
        Vec<command::CommandPool<families::Graphics>>,
        Vec<command::CommandPoolInfo<families::Graphics>>,
    >,
    CC: Store<
        Vec<command::CommandPool<families::Compute>>,
        Vec<command::CommandPoolInfo<families::Compute>>,
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
    pub fn transfer_pools(&self) -> &Vec<command::CommandPoolInfo<families::Transfer>> {
        &self.pools.transfer
    }
    
    pub fn transfer_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Transfer>> {
        &mut self.pools.transfer
    }
}

impl<I, D, CG, CC, CT, CP> Base<I, D, CG, CC, CT, Present, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
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
    CP: Store<
        Vec<command::CommandPool<families::Protected>>,
        Vec<command::CommandPoolInfo<families::Protected>>,
    >,
{
    pub fn sparse_pools(&self) -> &Vec<command::CommandPoolInfo<families::Sparse>> {
        &self.pools.sparse
    }
    
    pub fn sparse_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Sparse>> {
        &mut self.pools.sparse
    }
}

impl<I, D, CG, CC, CT, CS> Base<I, D, CG, CC, CT, CS, Present>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
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
{
    pub fn protected_pools(&self) -> &Vec<command::CommandPoolInfo<families::Protected>> {
        &self.pools.protected
    }
    
    pub fn protected_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Protected>> {
        &mut self.pools.protected
    }
}

impl<I, D, CG, CC, CT, CS, CP> Base<I, D, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
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
    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }
    
    pub fn entry_mut(&mut self) -> &mut ash::Entry {
        &mut self.entry
    }
}
