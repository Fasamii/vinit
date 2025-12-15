use ash::{self, vk};
use core::fmt;

pub mod instance;

pub mod command;
pub mod device;
pub mod families;
pub mod getters;
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

impl SatisfiesDeps<instance::InstanceInfo> for () {
    type Satisfied = Satisfied;
}

impl SatisfiesDeps<(device::DeviceInfo, instance::InstanceInfo)> for () {
    type Satisfied = Satisfied;
}

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
    instance: FieldConfig<I, instance::Instance, instance::InstanceInfo>,
    device: FieldConfig<D, device::Device, device::DeviceInfo>,

    required_queues: families::Families<bool>,
    pools: command::CommandPools<CG, CC, CT, CS, CP>,
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
