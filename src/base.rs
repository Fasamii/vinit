use crate::command;
use crate::device;
use crate::families;
use crate::instance;
use crate::{Absent, Apply, FieldConfig, FieldInfo, Present, Store};
use ash::vk;
use std::fmt;

#[allow(unused)]
pub struct Base<I, D, CG, CC, CT, CS, CP>
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
    pools: command::CommandPoolInfos<CG, CC, CT, CS, CP>,
    device: FieldInfo<D, device::Device, device::DeviceInfo>,
    instance: FieldInfo<I, instance::Instance, instance::InstanceInfo>,
    entry: ash::Entry,
}

pub struct BaseConfig<I, D, CG, CC, CT, CS, CP>
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
    pub instance: FieldConfig<I, instance::Instance, instance::InstanceInfo>,
    pub device: FieldConfig<D, device::Device, device::DeviceInfo>,

    pub required_queues: families::Families<bool>,
    pub pools: command::CommandPools<CG, CC, CT, CS, CP>,
}

impl Default for BaseConfig<Absent, Absent, Absent, Absent, Absent, Absent, Absent> {
    fn default() -> Self {
        Self {
            instance: (),
            device: (),
            required_queues: Default::default(),
            pools: Default::default(),
        }
    }
}

impl<I, D, CG, CC, CT, CS, CP> BaseConfig<I, D, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo> + instance::CreateInstance<I>,
    D: Store<device::Device, device::DeviceInfo> + device::CreateDevice<D, I>,
    CG: Store<
            Vec<command::CommandPool<families::Graphics>>,
            Vec<command::CommandPoolInfo<families::Graphics>>,
        > + command::CreateCommandPool<families::Graphics, CG, D>,
    CC: Store<
            Vec<command::CommandPool<families::Compute>>,
            Vec<command::CommandPoolInfo<families::Compute>>,
        > + command::CreateCommandPool<families::Compute, CC, D>,
    CT: Store<
            Vec<command::CommandPool<families::Transfer>>,
            Vec<command::CommandPoolInfo<families::Transfer>>,
        > + command::CreateCommandPool<families::Transfer, CT, D>,
    CS: Store<
            Vec<command::CommandPool<families::Sparse>>,
            Vec<command::CommandPoolInfo<families::Sparse>>,
        > + command::CreateCommandPool<families::Sparse, CS, D>,
    CP: Store<
            Vec<command::CommandPool<families::Protected>>,
            Vec<command::CommandPoolInfo<families::Protected>>,
        > + command::CreateCommandPool<families::Protected, CP, D>,
{
    pub fn build(self) -> Result<Base<I, D, CG, CC, CT, CS, CP>, vk::Result> {
        let entry =
            unsafe { ash::Entry::load().map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)? };
        let instance = I::create(self.instance, &entry)?;
        let device = D::create(self.device, &instance, self.required_queues)?;
        let pools_graphics = CG::create(self.pools.graphics, &device)?;
        let pools_compute = CC::create(self.pools.compute, &device)?;
        let pools_transfer = CT::create(self.pools.transfer, &device)?;
        let pools_sparse = CS::create(self.pools.sparse, &device)?;
        let pools_protected = CP::create(self.pools.protected, &device)?;
        let pools = command::CommandPoolInfos {
            graphics: pools_graphics,
            compute: pools_compute,
            transfer: pools_transfer,
            sparse: pools_sparse,
            protected: pools_protected,
        };

        Ok(Base {
            pools,
            device,
            instance,
            entry,
        })
    }

    pub fn with<T: Apply<Self>>(self, opt: T) -> T::Out {
        opt.apply(self)
    }
}

impl<I, D, CG, CC, CT, CS, CP> fmt::Debug for Base<I, D, CG, CC, CT, CS, CP>
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
    // Add these bounds to require the StoredInfo types to implement Debug
    FieldInfo<I, instance::Instance, instance::InstanceInfo>: fmt::Debug,
    FieldInfo<D, device::Device, device::DeviceInfo>: fmt::Debug,
    command::CommandPoolInfos<CG, CC, CT, CS, CP>: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Base")
            .field("Instance", &self.instance)
            .field("Device", &self.device)
            .field("Pools", &self.pools)
            .finish()
    }
}

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

    pub fn protected_pools_mut(
        &mut self,
    ) -> &mut Vec<command::CommandPoolInfo<families::Protected>> {
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
