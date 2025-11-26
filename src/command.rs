use ash::vk;
use std::marker::PhantomData;

use crate::families;

pub struct CommandPoolHandle<Q: families::QueueFamily, S: super::InitState> {
    pub command_pool: vk::CommandPool,

    // NOTE: Needed only for cleanup
    // TODO: make sure you preserver right order of dropping data
    device: ash::Device,

    _queue: PhantomData<Q>,
    _state: PhantomData<S>,
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
