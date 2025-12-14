use ash::{self, vk};
use core::fmt;

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
        > + command::CreateCommandPool<families::Graphics, CG, D, I>,
    CC: Store<
            Vec<command::CommandPool<families::Compute>>,
            Vec<command::CommandPoolInfo<families::Compute>>,
        > + command::CreateCommandPool<families::Compute, CC, D, I>,
    CT: Store<
            Vec<command::CommandPool<families::Transfer>>,
            Vec<command::CommandPoolInfo<families::Transfer>>,
        > + command::CreateCommandPool<families::Transfer, CT, D, I>,
    CS: Store<
            Vec<command::CommandPool<families::Sparse>>,
            Vec<command::CommandPoolInfo<families::Sparse>>,
        > + command::CreateCommandPool<families::Sparse, CS, D, I>,
    CP: Store<
            Vec<command::CommandPool<families::Protected>>,
            Vec<command::CommandPoolInfo<families::Protected>>,
        > + command::CreateCommandPool<families::Protected, CP, D, I>,
{
    pub fn build(self) -> Result<Base<I, D, CG, CC, CT, CS, CP>, vk::Result> {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Entry") };
        let instance = I::create(self.instance, &entry)?;
        let device = D::create(self.device, &instance, self.required_queues)?;
        let pools_graphics = CG::create(self.pools.graphics, &device, &instance)?;
        let pools_compute = CC::create(self.pools.compute, &device, &instance)?;
        let pools_transfer = CT::create(self.pools.transfer, &device, &instance)?;
        let pools_sparse = CS::create(self.pools.sparse, &device, &instance)?;
        let pools_protected = CP::create(self.pools.protected, &device, &instance)?;
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

    pub fn with<T: Apply<Self>>(self, opt: T) -> T::Out {
        opt.apply(self)
    }
}

impl<I, D, CG, CC, CT, CS, CP> fmt::Debug for Base<I, D, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo, StoredInfo = dyn fmt::Debug>,
    D: Store<device::Device, device::DeviceInfo, StoredInfo = dyn fmt::Debug>,
    CG: Store<
            Vec<command::CommandPool<families::Graphics>>,
            Vec<command::CommandPoolInfo<families::Graphics>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CC: Store<
            Vec<command::CommandPool<families::Compute>>,
            Vec<command::CommandPoolInfo<families::Compute>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CT: Store<
            Vec<command::CommandPool<families::Transfer>>,
            Vec<command::CommandPoolInfo<families::Transfer>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CS: Store<
            Vec<command::CommandPool<families::Sparse>>,
            Vec<command::CommandPoolInfo<families::Sparse>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CP: Store<
            Vec<command::CommandPool<families::Protected>>,
            Vec<command::CommandPoolInfo<families::Protected>>,
            StoredInfo = dyn fmt::Debug,
        >,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Base")
            .field("Instance", &self.instance)
            .field("Device", &self.device)
            .field("Pools", &&self.pools)
            .finish()
    }
}
