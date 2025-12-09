use ash::vk;
use std::sync::Arc;
use std::{marker::PhantomData, process::Command};

use crate::{DeviceInfo, Field, Store, command, families};

// TODO: get back to the handle idea but instead of storing all the data make it only point to the
// data in Base struct
pub struct CommandPoolInfo<Q: families::QueueFamily> {
    pub pool: vk::CommandPool,
    device: Arc<ash::Device>,
    queue: vk::Queue,
    _queue: PhantomData<Q>,
}

impl<Q: families::QueueFamily> Drop for CommandPoolInfo<Q> {
    fn drop(&mut self) {
        unsafe {
            self.device.queue_wait_idle(self.queue).unwrap();
            self.device.destroy_command_pool(self.pool, None);
        }
    }
}

pub struct CommandPools<
    CG: Store<Vec<CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<CommandPoolInfo<families::Protected>>>,
> {
    graphics: Field<CG, Vec<CommandPoolInfo<families::Graphics>>>,
    compute: Field<CC, Vec<CommandPoolInfo<families::Compute>>>,
    transfer: Field<CT, Vec<CommandPoolInfo<families::Transfer>>>,
    sparse: Field<CS, Vec<CommandPoolInfo<families::Sparse>>>,
    protected: Field<CP, Vec<CommandPoolInfo<families::Protected>>>,
}

pub struct CommandPoolConfig<Q: families::QueueFamily> {
    flags: vk::CommandPoolCreateFlags,
    _queue: PhantomData<Q>,
}

impl<Q: families::QueueFamily> CommandPoolConfig<Q> {
    pub fn new(&self, device: &DeviceInfo) -> CommandPoolInfo<Q> {
        let command_pool_create_info = vk::CommandPoolCreateInfo::default().queue_family_index(0);
        let command_pool = unsafe {
            device
                .device
                .create_command_pool(&command_pool_create_info, None)
                .unwrap()
        };
        CommandPoolInfo {
            pool: command_pool,
            device: Arc::clone(&device.device),
            queue: device.queue_handles.get::<Q>().unwrap(),
            _queue: PhantomData,
        }
    }
}

impl<Q: families::QueueFamily> Default for CommandPoolConfig<Q> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            _queue: PhantomData,
        }
    }
}

impl<Q: families::QueueFamily> CommandPoolConfig<Q> {
    fn cast<Q2: families::QueueFamily>(self) -> CommandPoolConfig<Q2> {
        CommandPoolConfig {
            flags: self.flags,
            _queue: PhantomData,
        }
    }
}

impl<Q: families::QueueFamily> CommandPoolConfig<Q> {
    pub fn graphics_queue(mut self) -> CommandPoolConfig<families::Graphics> {
        self.cast()
    }
    pub fn compute_queue(mut self) -> CommandPoolConfig<families::Compute> {
        self.cast()
    }
    pub fn transfer_queue(mut self) -> CommandPoolConfig<families::Transfer> {
        self.cast()
    }
    pub fn sparse_queue(mut self) -> CommandPoolConfig<families::Sparse> {
        self.cast()
    }
    pub fn protected_queue(mut self) -> CommandPoolConfig<families::Protected> {
        self.cast()
    }
}

impl<Q: families::QueueFamily> CommandPoolConfig<Q> {
    pub fn require_flags(mut self, flags: vk::CommandPoolCreateFlags) -> Self {
        self.flags = flags;
        self
    }
}

pub enum CommandPoolConfigFamily {
    Graphics(CommandPoolConfig<families::Graphics>),
    Compute(CommandPoolConfig<families::Compute>),
    Transfer(CommandPoolConfig<families::Transfer>),
    Sparse(CommandPoolConfig<families::Sparse>),
    Protected(CommandPoolConfig<families::Protected>),
}

impl From<CommandPoolConfig<families::Graphics>> for CommandPoolConfigFamily {
    fn from(config: CommandPoolConfig<families::Graphics>) -> Self {
        CommandPoolConfigFamily::Graphics(config)
    }
}

impl From<CommandPoolConfig<families::Compute>> for CommandPoolConfigFamily {
    fn from(config: CommandPoolConfig<families::Compute>) -> Self {
        CommandPoolConfigFamily::Compute(config)
    }
}

impl From<CommandPoolConfig<families::Transfer>> for CommandPoolConfigFamily {
    fn from(config: CommandPoolConfig<families::Transfer>) -> Self {
        CommandPoolConfigFamily::Transfer(config)
    }
}

impl From<CommandPoolConfig<families::Sparse>> for CommandPoolConfigFamily {
    fn from(config: CommandPoolConfig<families::Sparse>) -> Self {
        CommandPoolConfigFamily::Sparse(config)
    }
}

impl From<CommandPoolConfig<families::Protected>> for CommandPoolConfigFamily {
    fn from(config: CommandPoolConfig<families::Protected>) -> Self {
        CommandPoolConfigFamily::Protected(config)
    }
}
