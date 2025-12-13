// use ash::vk;
// use std::marker::PhantomData;
// use std::sync::Arc;
//
// use crate::{Absent, device::DeviceInfo, Field, Present, Store, command, families};
//
// pub trait BuildCommandPools<Q: families::QueueFamily, S: Store<Vec<CommandPoolInfo<Q>>>> {
//     fn build_pools(
//         configs: Vec<&command::CommandPoolConfig<Q>>,
//         device: &DeviceInfo,
//     ) -> Result<S::Stored, vk::Result>;
// }
//
// impl<Q: families::QueueFamily> BuildCommandPools<Q, Absent> for Absent {
//     fn build_pools(
//         configs: Vec<&command::CommandPoolConfig<Q>>,
//         device: &DeviceInfo,
//     ) -> Result<(), vk::Result> {
//         if !configs.is_empty() {
//             panic!(
//                 "Attempted to create {:?} command pools, but type parameter is Absent",
//                 std::any::type_name::<Q>()
//             );
//         }
//         Ok(())
//     }
// }
//
// impl<Q: families::QueueFamily> BuildCommandPools<Q, Present> for Present {
//     fn build_pools(
//         configs: Vec<&command::CommandPoolConfig<Q>>,
//         device: &DeviceInfo,
//     ) -> Result<Vec<CommandPoolInfo<Q>>, vk::Result> {
//         configs
//             .into_iter()
//             .map(|config| CommandPoolInfo::new(config, device))
//             .collect::<Result<Vec<CommandPoolInfo<Q>>, vk::Result>>()
//     }
// }
//
// // TODO: get back to the handle idea but instead of storing all the data make it only point to the
// // data in Base struct
// pub struct CommandPoolInfo<Q: families::QueueFamily> {
//     pub pool: vk::CommandPool,
//     device: Arc<ash::Device>,
//     queue: vk::Queue,
//     _queue: PhantomData<Q>,
// }
//
// impl<Q: families::QueueFamily> CommandPoolInfo<Q> {
//     pub fn new(
//         config: &CommandPoolConfig<Q>,
//         device: &DeviceInfo,
//     ) -> Result<CommandPoolInfo<Q>, vk::Result> {
//         let queue_family_index = device
//             .physical_info
//             .queue_families_indices
//             .get::<Q>()
//             .expect("Queue handle must exist, this is a type system guarantee");
//         let command_pool_create_info = vk::CommandPoolCreateInfo::default()
//             .flags(config.flags)
//             .queue_family_index(queue_family_index);
//         let command_pool = unsafe {
//             device
//                 .device
//                 .create_command_pool(&command_pool_create_info, None)?
//         };
//         Ok(CommandPoolInfo {
//             pool: command_pool,
//             device: Arc::clone(&device.device),
//             queue: device
//                 .queue_handles
//                 .get::<Q>()
//                 .expect("Queue handle must exist, this is a type system guarantee"),
//             _queue: PhantomData,
//         })
//     }
// }
//
// impl<Q: families::QueueFamily> Drop for CommandPoolInfo<Q> {
//     fn drop(&mut self) {
//         unsafe {
//             self.device.queue_wait_idle(self.queue).ok();
//             self.device.destroy_command_pool(self.pool, None);
//         }
//     }
// }
//
// pub struct CommandPools<
//     CG: Store<Vec<CommandPoolInfo<families::Graphics>>>,
//     CC: Store<Vec<CommandPoolInfo<families::Compute>>>,
//     CT: Store<Vec<CommandPoolInfo<families::Transfer>>>,
//     CS: Store<Vec<CommandPoolInfo<families::Sparse>>>,
//     CP: Store<Vec<CommandPoolInfo<families::Protected>>>,
// > {
//     pub graphics: Field<CG, Vec<CommandPoolInfo<families::Graphics>>>,
//     pub compute: Field<CC, Vec<CommandPoolInfo<families::Compute>>>,
//     pub transfer: Field<CT, Vec<CommandPoolInfo<families::Transfer>>>,
//     pub sparse: Field<CS, Vec<CommandPoolInfo<families::Sparse>>>,
//     pub protected: Field<CP, Vec<CommandPoolInfo<families::Protected>>>,
// }
//
// pub struct CommandPoolConfig<Q: families::QueueFamily> {
//     flags: vk::CommandPoolCreateFlags,
//     _queue: PhantomData<Q>,
// }
//
// impl<Q: families::QueueFamily> Default for CommandPoolConfig<Q> {
//     fn default() -> Self {
//         Self {
//             flags: Default::default(),
//             _queue: PhantomData,
//         }
//     }
// }
//
// impl<Q: families::QueueFamily> CommandPoolConfig<Q> {
//     fn cast<Q2: families::QueueFamily>(self) -> CommandPoolConfig<Q2> {
//         CommandPoolConfig {
//             flags: self.flags,
//             _queue: PhantomData,
//         }
//     }
// }
//
// impl<Q: families::QueueFamily> CommandPoolConfig<Q> {
//     pub fn require_flags(mut self, flags: vk::CommandPoolCreateFlags) -> Self {
//         self.flags = flags;
//         self
//     }
// }
//
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
