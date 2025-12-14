use crate::device;
use crate::families;
use crate::{Absent, FieldConfig, FieldInfo, Present, Store};
use crate::{Apply, BaseConfig};
use crate::{SatisfiesDeps, Unsatisfied};
use ash::vk;
use core::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait CreateCommandPool<Q: families::QueueFamily, P, D>
where
    P: Store<Vec<CommandPool<Q>>, Vec<CommandPoolInfo<Q>>>,
    D: Store<device::Device, device::DeviceInfo>,
{
    fn create(
        configs: P::StoredConfig,
        device: &D::StoredInfo,
    ) -> Result<P::StoredInfo, vk::Result>;
}

impl<Q: families::QueueFamily, D> CreateCommandPool<Q, Absent, D> for Absent
where
    D: Store<device::Device, device::DeviceInfo>,
{
    fn create(_configs: (), _device: &D::StoredInfo) -> Result<(), vk::Result> {
        log::debug!("Creating () for {}", std::any::type_name::<Q>());
        Ok(())
    }
}

impl<Q: families::QueueFamily, D> CreateCommandPool<Q, Present, D> for Present
where
    D: Store<device::Device, device::DeviceInfo>,
    (): SatisfiesDeps<D, Satisfied = Unsatisfied>,
{
    fn create(
        _configs: Vec<CommandPool<Q>>,
        _device: &D::StoredInfo,
    ) -> Result<Vec<CommandPoolInfo<Q>>, vk::Result> {
        Err(vk::Result::ERROR_INITIALIZATION_FAILED)
    }
}

impl<Q: families::QueueFamily> CreateCommandPool<Q, Present, Present> for Present {
    fn create(
        configs: Vec<CommandPool<Q>>,
        device: &device::DeviceInfo,
    ) -> Result<Vec<CommandPoolInfo<Q>>, vk::Result> {
        log::debug!("Creating CommandPoolInfo");
        configs
            .into_iter()
            .map(|config| config.create(device))
            .collect()
    }
}

pub struct CommandPoolInfo<Q: families::QueueFamily> {
    pub pool: vk::CommandPool,
    device: Arc<ash::Device>,
    queue: vk::Queue,
    _queue: PhantomData<Q>,
}

impl<Q: families::QueueFamily> Drop for CommandPoolInfo<Q> {
    fn drop(&mut self) {
        unsafe {
            self.device.queue_wait_idle(self.queue).ok();
            self.device.destroy_command_pool(self.pool, None);
        }
    }
}

pub struct CommandPool<Q: families::QueueFamily> {
    flags: vk::CommandPoolCreateFlags,
    _queue: PhantomData<Q>,
}

impl<Q: families::QueueFamily> CommandPool<Q> {
    pub fn graphics() -> CommandPool<families::Graphics> {
        CommandPool {
            flags: Default::default(),
            _queue: PhantomData,
        }
    }
    pub fn compute() -> CommandPool<families::Compute> {
        CommandPool {
            flags: Default::default(),
            _queue: PhantomData,
        }
    }
    pub fn transfer() -> CommandPool<families::Transfer> {
        CommandPool {
            flags: Default::default(),
            _queue: PhantomData,
        }
    }
    pub fn sparse() -> CommandPool<families::Sparse> {
        CommandPool {
            flags: Default::default(),
            _queue: PhantomData,
        }
    }
    pub fn protected() -> CommandPool<families::Protected> {
        CommandPool {
            flags: Default::default(),
            _queue: PhantomData,
        }
    }
}

impl<Q: families::QueueFamily> CommandPool<Q> {
    pub fn flags(mut self, flags: vk::CommandPoolCreateFlags) -> Self {
        self.flags = flags;
        self
    }
}

impl<Q: families::QueueFamily> CommandPool<Q> {
    fn create(self, device: &device::DeviceInfo) -> Result<CommandPoolInfo<Q>, vk::Result> {
        let queue_family_index = device.physical.queue_families_indices.get::<Q>().unwrap();
        let command_pool_create_info = vk::CommandPoolCreateInfo::default()
            .flags(self.flags)
            .queue_family_index(queue_family_index);
        let command_pool = unsafe {
            device
                .device
                .create_command_pool(&command_pool_create_info, None)?
        };

        Ok(CommandPoolInfo {
            pool: command_pool,
            device: Arc::clone(&device.device),
            queue: device.queue_handles.get::<Q>().unwrap(),
            _queue: PhantomData,
        })
    }
}

pub struct CommandPools<
    CG: Store<Vec<CommandPool<families::Graphics>>, Vec<CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<CommandPool<families::Compute>>, Vec<CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<CommandPool<families::Transfer>>, Vec<CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<CommandPool<families::Sparse>>, Vec<CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<CommandPool<families::Protected>>, Vec<CommandPoolInfo<families::Protected>>>,
> {
    pub graphics: FieldConfig<
        CG,
        Vec<CommandPool<families::Graphics>>,
        Vec<CommandPoolInfo<families::Graphics>>,
    >,
    pub compute: FieldConfig<
        CC,
        Vec<CommandPool<families::Compute>>,
        Vec<CommandPoolInfo<families::Compute>>,
    >,
    pub transfer: FieldConfig<
        CT,
        Vec<CommandPool<families::Transfer>>,
        Vec<CommandPoolInfo<families::Transfer>>,
    >,
    pub sparse:
        FieldConfig<CS, Vec<CommandPool<families::Sparse>>, Vec<CommandPoolInfo<families::Sparse>>>,
    pub protected: FieldConfig<
        CP,
        Vec<CommandPool<families::Protected>>,
        Vec<CommandPoolInfo<families::Protected>>,
    >,
}

impl Default for CommandPools<Absent, Absent, Absent, Absent, Absent> {
    fn default() -> Self {
        CommandPools {
            graphics: (),
            compute: (),
            transfer: (),
            sparse: (),
            protected: (),
        }
    }
}

pub struct CommandPoolInfos<
    CG: Store<Vec<CommandPool<families::Graphics>>, Vec<CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<CommandPool<families::Compute>>, Vec<CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<CommandPool<families::Transfer>>, Vec<CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<CommandPool<families::Sparse>>, Vec<CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<CommandPool<families::Protected>>, Vec<CommandPoolInfo<families::Protected>>>,
> {
    pub graphics: FieldInfo<
        CG,
        Vec<CommandPool<families::Graphics>>,
        Vec<CommandPoolInfo<families::Graphics>>,
    >,
    pub compute:
        FieldInfo<CC, Vec<CommandPool<families::Compute>>, Vec<CommandPoolInfo<families::Compute>>>,
    pub transfer: FieldInfo<
        CT,
        Vec<CommandPool<families::Transfer>>,
        Vec<CommandPoolInfo<families::Transfer>>,
    >,
    pub sparse:
        FieldInfo<CS, Vec<CommandPool<families::Sparse>>, Vec<CommandPoolInfo<families::Sparse>>>,
    pub protected: FieldInfo<
        CP,
        Vec<CommandPool<families::Protected>>,
        Vec<CommandPoolInfo<families::Protected>>,
    >,
}

impl<CG, CC, CT, CS, CP> fmt::Debug for CommandPoolInfos<CG, CC, CT, CS, CP>
where
    CG: Store<
            Vec<CommandPool<families::Graphics>>,
            Vec<CommandPoolInfo<families::Graphics>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CC: Store<
            Vec<CommandPool<families::Compute>>,
            Vec<CommandPoolInfo<families::Compute>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CT: Store<
            Vec<CommandPool<families::Transfer>>,
            Vec<CommandPoolInfo<families::Transfer>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CS: Store<
            Vec<CommandPool<families::Sparse>>,
            Vec<CommandPoolInfo<families::Sparse>>,
            StoredInfo = dyn fmt::Debug,
        >,
    CP: Store<
            Vec<CommandPool<families::Protected>>,
            Vec<CommandPoolInfo<families::Protected>>,
            StoredInfo = dyn fmt::Debug,
        >,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandPoolInfos").finish()
    }
}

// NOTE: May be useful later but keep commented out for now
// pub enum CommandPoolConfigFamily {
//     Graphics(CommandPoolConfig<families::Graphics>),
//     Compute(CommandPoolConfig<families::Compute>),
//     Transfer(CommandPoolConfig<families::Transfer>),
//     Sparse(CommandPoolConfig<families::Sparse>),
//     Protected(CommandPoolConfig<families::Protected>),
// }
//
// impl From<CommandPoolConfig<families::Graphics>> for CommandPoolConfigFamily {
//     fn from(config: CommandPoolConfig<families::Graphics>) -> Self {
//         CommandPoolConfigFamily::Graphics(config)
//     }
// }
//
// impl From<CommandPoolConfig<families::Compute>> for CommandPoolConfigFamily {
//     fn from(config: CommandPoolConfig<families::Compute>) -> Self {
//         CommandPoolConfigFamily::Compute(config)
//     }
// }
//
// impl From<CommandPoolConfig<families::Transfer>> for CommandPoolConfigFamily {
//     fn from(config: CommandPoolConfig<families::Transfer>) -> Self {
//         CommandPoolConfigFamily::Transfer(config)
//     }
// }
//
// impl From<CommandPoolConfig<families::Sparse>> for CommandPoolConfigFamily {
//     fn from(config: CommandPoolConfig<families::Sparse>) -> Self {
//         CommandPoolConfigFamily::Sparse(config)
//     }
// }
//
// impl From<CommandPoolConfig<families::Protected>> for CommandPoolConfigFamily {
//     fn from(config: CommandPoolConfig<families::Protected>) -> Self {
//         CommandPoolConfigFamily::Protected(config)
//     }
// }
//
// impl CommandPoolConfigFamily {
//     pub fn get_graphics(&self) -> Option<&CommandPoolConfig<families::Graphics>> {
//         match self {
//             CommandPoolConfigFamily::Graphics(command_pool_config) => Some(command_pool_config),
//             _ => None,
//         }
//     }
//     pub fn get_compute(&self) -> Option<&CommandPoolConfig<families::Compute>> {
//         match self {
//             CommandPoolConfigFamily::Compute(command_pool_config) => Some(command_pool_config),
//             _ => None,
//         }
//     }
//     pub fn get_transfer(&self) -> Option<&CommandPoolConfig<families::Transfer>> {
//         match self {
//             CommandPoolConfigFamily::Transfer(command_pool_config) => Some(command_pool_config),
//             _ => None,
//         }
//     }
//     pub fn get_sparse(&self) -> Option<&CommandPoolConfig<families::Sparse>> {
//         match self {
//             CommandPoolConfigFamily::Sparse(command_pool_config) => Some(command_pool_config),
//             _ => None,
//         }
//     }
//     pub fn get_protected(&self) -> Option<&CommandPoolConfig<families::Protected>> {
//         match self {
//             CommandPoolConfigFamily::Protected(command_pool_config) => Some(command_pool_config),
//             _ => None,
//         }
//     }
// }

trait AppendToField<T> {
    fn append_or_create(self, item: T) -> Vec<T>;
}

impl<T> AppendToField<T> for () {
    fn append_or_create(self, item: T) -> Vec<T> {
        vec![item]
    }
}

impl<T> AppendToField<T> for Vec<T> {
    fn append_or_create(mut self, item: T) -> Vec<T> {
        self.push(item);
        self
    }
}

impl<CG, CC, CT, CS, CP> Apply<BaseConfig<Present, Present, CG, CC, CT, CS, CP>>
    for CommandPool<families::Graphics>
where
    CG: Store<Vec<CommandPool<families::Graphics>>, Vec<CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<CommandPool<families::Compute>>, Vec<CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<CommandPool<families::Transfer>>, Vec<CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<CommandPool<families::Sparse>>, Vec<CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<CommandPool<families::Protected>>, Vec<CommandPoolInfo<families::Protected>>>,
    <CG as Store<Vec<CommandPool<families::Graphics>>, Vec<CommandPoolInfo<families::Graphics>>>>::StoredConfig: AppendToField<CommandPool<families::Graphics>>,
{
    type Out = BaseConfig<Present, Present, Present, CC, CT, CS, CP>;
    fn apply(self, config: BaseConfig<Present, Present, CG, CC, CT, CS, CP>) -> Self::Out {
        let mut required_queues = config.required_queues;
        required_queues.set::<families::Graphics>(true);
        let graphics = config.pools.graphics.append_or_create(self);
        BaseConfig {
            instance: config.instance,
            device: config.device,
            required_queues,
            pools: CommandPools {
                graphics,
                compute: config.pools.compute,
                transfer: config.pools.transfer,
                sparse: config.pools.sparse,
                protected: config.pools.protected,
            },
        }
    }
}
