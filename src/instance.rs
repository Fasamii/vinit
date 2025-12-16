use crate::base::BaseConfig;
use crate::command;
use crate::device;
use crate::families;
use crate::Apply;
use crate::{Absent, Present, Store};
use ash::vk;
use core::fmt;
use std::ffi::CString;

/// Trait for creating Vulkan instances.
///
/// This trait abstracts the creation of instances for both [`Present`] and [`Absent`] states,
/// allowing the build process to work generically over both cases.
///
/// # Type Parameters
///
/// * `S` - The presence marker indicating whether an instance should be created
pub trait CreateInstance<S>
where
    S: Store<Instance, InstanceInfo>,
{
    /// Creates an instance from the configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The instance configuration (or `()` if absent)
    /// * `entry` - The Vulkan entry point
    ///
    /// # Returns
    ///
    /// The created instance info (or `()` if absent)
    ///
    /// # Errors
    ///
    /// Returns a Vulkan error if instance creation fails.
    fn create(config: S::StoredConfig, entry: &ash::Entry) -> Result<S::StoredInfo, vk::Result>;
}

impl CreateInstance<Absent> for Absent {
    fn create(_config: (), _entry: &ash::Entry) -> Result<(), vk::Result> {
        Ok(())
    }
}

impl CreateInstance<Present> for Present {
    fn create(config: Instance, entry: &ash::Entry) -> Result<InstanceInfo, vk::Result> {
        config.create(entry)
    }
}

/// Runtime information for a created Vulkan instance.
///
/// This struct wraps the ash `Instance` handle and implements `Drop` to ensure
/// proper cleanup when the instance is no longer needed.
pub struct InstanceInfo(pub ash::Instance);

impl fmt::Debug for InstanceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstanceInfo")
            .field("handle", &self.0.handle())
            .finish()
    }
}

impl Drop for InstanceInfo {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy_instance(None);
        }
    }
}

/// Builder for configuring a Vulkan instance.
///
/// The `Instance` struct provides a fluent API for specifying instance creation parameters
/// including API version, application information, extensions, and validation layers.
pub struct Instance {
    api_version: (u32, u32, u32),
    app_name: Option<CString>,
    app_version: Option<(u32, u32, u32)>,
    engine_name: Option<CString>,
    engine_version: Option<(u32, u32, u32)>,
    extensions: Option<Vec<CString>>,
    validation: Option<Vec<CString>>,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            api_version: (1, 0, 0),
            app_name: None,
            app_version: None,
            engine_name: None,
            engine_version: None,
            extensions: None,
            validation: None,
        }
    }
}

impl Instance {
    // Sets the Vulkan API version to request.
    pub fn api_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.api_version = (major, minor, patch);
        self
    }

    /// Sets the application name.
    pub fn app_name(mut self, app_name: CString) -> Self {
        self.app_name = Some(app_name);
        self
    }

    /// Sets the application version.
    pub fn app_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.app_version = Some((patch, minor, major));
        self
    }

    /// Sets the engine name.
    pub fn engine_name(mut self, engine_name: CString) -> Self {
        self.engine_name = Some(engine_name);
        self
    }

    /// Sets the engine version.
    pub fn engine_version(mut self, major: u32, minor: u32, patch: u32) -> Self {
        self.engine_version = Some((patch, minor, major));
        self
    }


    /// Sets the instance extensions to enable.
    pub fn extensions(mut self, extensions: Vec<CString>) -> Self {
        self.extensions = Some(extensions);
        self
    }
    // TODO: think about disabling that in release build
    /// Enables validation layers.
    pub fn validation(mut self, layers: Vec<CString>) -> Self {
        self.validation = Some(layers);
        self
    }
}

impl Instance {
    fn make_version(version: (u32, u32, u32)) -> u32 {
        vk::make_api_version(0, version.0, version.1, version.2)
    }
}

impl Instance {
    fn create(self, entry: &ash::Entry) -> Result<InstanceInfo, vk::Result> {
        let app_info =
            vk::ApplicationInfo::default().api_version(Self::make_version(self.api_version));

        let app_info = if let Some(ref app_name) = self.app_name {
            app_info.application_name(app_name)
        } else {
            app_info
        };
        let app_info = if let Some(app_version) = self.app_version {
            app_info.application_version(Self::make_version(app_version))
        } else {
            app_info
        };
        let app_info = if let Some(ref engine_name) = self.engine_name {
            app_info.engine_name(engine_name)
        } else {
            app_info
        };
        let app_info = if let Some(engine_version) = self.engine_version {
            app_info.engine_version(Self::make_version(engine_version))
        } else {
            app_info
        };

        let extension_ptrs: Vec<*const i8> = self
            .extensions
            .as_ref()
            .map(|exts| exts.iter().map(|e| e.as_ptr()).collect())
            .unwrap_or_default();

        let layer_ptrs: Vec<*const i8> = self
            .validation
            .as_ref()
            .map(|layers| layers.iter().map(|l| l.as_ptr()).collect())
            .unwrap_or_default();

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_ptrs)
            .enabled_layer_names(&layer_ptrs);

        Ok(InstanceInfo(unsafe {
            entry.create_instance(&instance_create_info, None)?
        }))
    }
}

impl<D, CG, CC, CT, CS, CP> Apply<BaseConfig<Absent, D, CG, CC, CT, CS, CP>> for Instance
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
    type Out = BaseConfig<Present, D, CG, CC, CT, CS, CP>;

    fn apply(self, config: BaseConfig<Absent, D, CG, CC, CT, CS, CP>) -> Self::Out {
        BaseConfig {
            instance: self,
            device: config.device,
            required_queues: config.required_queues,
            pools: config.pools,
        }
    }
}
