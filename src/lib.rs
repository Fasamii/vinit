//! Vulkan configuration and initialization library (vinit)
//!
//! A type-safe, compile-time checked Vulkan initialization library using the type-state pattern.
//!
//! ## Overview
//!
//! This library provides a builder-style API for configuring and initializing Vulkan resources
//! with compile-time guarantees about initialization order and resource availability. It uses
//! Rust's type system to ensure that resources can only be accessed after they've been properly
//! configured, elimination entire classes of runtime errors.
//!
//! ## Features
//!
//! - Type-safe initialization
//! - Zero-cost abstractions: Type-state encoding has no runtime overhead
//! - Flexible configuration: Builder pattern allows progressive configuration
//! - Automatic cleanup: Using Drop trait and Base structure
//! - Queue family management: Type-safe handling of different queue families
//!
//! ## Examples
//!
//! ### Basic Setup
//!
//! ```rust,no_run
//! # use vinit::*;
//! # use vinit::base::BaseConfig;
//! # use vinit::instance::Instance;
//! # use vinit::device::Device;
//! # use std::ffi::CString;
//! # fn main() -> Result<(), ash::vk::Result> {
//! let config = BaseConfig::default()
//!     .with(Instance::default())
//!     .with(Device::default())
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### With Validation Layers
//!
//! ```rust,no_run
//! # use vinit::*;
//! # use vinit::base::BaseConfig;
//! # use vinit::instance::Instance;
//! # use vinit::device::Device;
//! # use std::ffi::CString;
//! # fn main() -> Result<(), ash::vk::Result> {
//! let config = BaseConfig::default()
//!     .with(Instance::default()
//!         .validation(vec![
//!             CString::new("VK_LAYER_KHRONOS_validation").unwrap()
//!         ]))
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Multiple Command Pools
//!
//! ```rust,no_run
//! # use vinit::*;
//! # use vinit::base::BaseConfig;
//! # use vinit::instance::Instance;
//! # use vinit::device::Device;
//! # use vinit::command::CommandPool;
//! # use ash::vk;
//! # fn main() -> Result<(), ash::vk::Result> {
//! let config = BaseConfig::default()
//!     .with(Instance::default())
//!     .with(Device::default())
//!     .with(CommandPool::graphics())
//!     .with(CommandPool::compute()
//!         .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER))
//!     .with(CommandPool::transfer())
//!     .build()?;
//! # Ok(())
//! # }
//! ```

pub mod base;
pub mod command;
pub mod device;
pub mod families;
pub mod instance;
mod mass;
pub mod swapchain;

/// Trait for encoding presence/absence of configuration in the type system.
///
/// This trait is the foundation of the type-state pattern used through the library.
/// It allows types to represent either a concrete value ([`Present`]) or the absence of a value
/// ([`Absent`]) at the type level.
///
/// # Type Parameters
///
/// `C` - The configuration type when present
/// `I` - The information type when present (typically the result of building `C`)
///
/// # Associated Types
///
/// `StoredConfig` - Either `C` (when [`Present`]) or `()` (when [`Absent`])
/// `StoredInfo` - Either `I` (when [`Present`]) or `()` (when [`Absent`])
pub trait Store<C, I> {
    type StoredConfig;
    type StoredInfo;
}

/// Marker type indicating a resource is present.
pub struct Present;
impl<C, I> Store<C, I> for Present {
    type StoredConfig = C;
    type StoredInfo = I;
}

/// Marker type indicating a resource is absent.
pub struct Absent;
impl<C, I> Store<C, I> for Absent {
    type StoredConfig = ();
    type StoredInfo = ();
}

/// Type alias for the configuration field type based on presence/absence.
///
/// This resolves to either the concrete configuration type `C` (when `S = Present`)
/// or `()` (when `S = Absent`).
///
/// # Type Parameters
///
/// * `S` - The presence marker ([`Present`] or [`Absent`])
/// * `C` - The configuration type
/// * `I` - The information type
type FieldConfig<S, C, I> = <S as Store<C, I>>::StoredConfig;

/// Type alias for the information field type based on presence/absence.
///
/// This resolves to either the concrete info type `I` (when `S = Present`)
/// or `()` (when `S = Absent`).
///
/// # Type Parameters
///
/// * `S` - The presence marker ([`Present`] or [`Absent`])
/// * `C` - The configuration type
/// * `I` - The information type
type FieldInfo<S, C, I> = <S as Store<C, I>>::StoredInfo;

/// Trait for applying configuration items to a base configuration.
///
/// This trait enables the builder pattern used throughout the library. Each configuration
/// item (instance, device, command pools, ...) implements `Apply` to transform the base
/// configuration from one state to another, typically transitioning a resource from
/// [`Absent`] to [`Present`].
///
/// # Type Parameters
///
/// * `For` - The type of configuration being modified
///
/// # Associated Types
///
/// * `Out` - The resulting configuration type after applying this item
///
/// # Examples
///
/// ```rust,ignore
/// // Instance transitions from Absent to Present
/// impl Apply<BaseConfig<Absent, D, ...>> for Instance {
///     type Out = BaseConfig<Present, D, ...>;
///     fn apply(self, config: BaseConfig<Absent, D, ...>) -> Self::Out {
///         // Transform configuration
///     }
/// }
/// ```
pub trait Apply<For> {
    /// The resulting configuration type after applying this item.
    type Out;
    /// Applies this configuration item to the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to modify
    ///
    /// # Returns
    ///
    /// A new configuration with this item applied
    fn apply(self, config: For) -> Self::Out;
}
