use ash::vk;
use std::marker::PhantomData;

use crate::{command, families};

// TODO: get back to the handle idea but instead of storing all the data make it only point to the
// data in Base struct
pub struct CommandPoolInfo<Q: families::QueueFamily> {
    pub pool: vk::CommandPool,
    device: ash::Device,
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

pub struct CommandPoolConfig<Q: families::QueueFamily> {
    flags: vk::CommandPoolCreateFlags,
    _queue: PhantomData<Q>,
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

pub enum PoolConfig {
    Graphics(CommandPoolConfig<families::Graphics>),
    Compute(CommandPoolConfig<families::Compute>),
    Transfer(CommandPoolConfig<families::Transfer>),
    Sparse(CommandPoolConfig<families::Sparse>),
    Protected(CommandPoolConfig<families::Protected>),
}

impl From<CommandPoolConfig<families::Graphics>> for PoolConfig {
    fn from(config: CommandPoolConfig<families::Graphics>) -> Self {
        PoolConfig::Graphics(config)
    }
}

impl From<CommandPoolConfig<families::Compute>> for PoolConfig {
    fn from(config: CommandPoolConfig<families::Compute>) -> Self {
        PoolConfig::Compute(config)
    }
}

impl From<CommandPoolConfig<families::Transfer>> for PoolConfig {
    fn from(config: CommandPoolConfig<families::Transfer>) -> Self {
        PoolConfig::Transfer(config)
    }
}

impl From<CommandPoolConfig<families::Sparse>> for PoolConfig {
    fn from(config: CommandPoolConfig<families::Sparse>) -> Self {
        PoolConfig::Sparse(config)
    }
}

impl From<CommandPoolConfig<families::Protected>> for PoolConfig {
    fn from(config: CommandPoolConfig<families::Protected>) -> Self {
        PoolConfig::Protected(config)
    }
}
