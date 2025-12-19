use crate::command;
use crate::device;
use crate::families;
use crate::instance;
use crate::swapchain;
use crate::{Absent, Apply, FieldConfig, FieldInfo, Present, Store};
use ash::vk;
use std::fmt;

/// The main runtime structure containing initialized Vulkan resources.
///
/// `Base` is the result of building a [`BaseConfig`]. It contains all the created
/// Vulkan resources with RAII-based cleanup. Resources are automatically destroyed
/// in the correct order when `Base` is dropped.
///
/// # Type Parameters
///
/// The type parameters match those of [`BaseConfig`] and indicate which resources
/// are present. All type parameters will be either [`Present`] or [`Absent`].
///
/// # Resource Access
///
/// Resources can only be accessed if they're marked as [`Present`] in the type.
/// This is enforced through impl blocks with specific type parameter requirements.
#[allow(unused)]
pub struct Base<I, D, S, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    swapchain: FieldInfo<S, swapchain::Swapchain, swapchain::SwapchainInfo>,
    device: FieldInfo<D, device::Device, device::DeviceInfo>,
    instance: FieldInfo<I, instance::Instance, instance::InstanceInfo>,
    entry: ash::Entry,
}

/// The main configuration structure during the building phase.
///
/// `BaseConfig` uses the type-state pattern to track which resources have been configured.
/// As you add resources using the `with` method, the type parameters change from [`Absent`]
/// to [`Present`], ensuring compile-time safety.
///
/// # Type Parameters
///
/// * `I` - Instance presence marker
/// * `D` - Device presence marker
/// * `CG` - Graphics command pools presence marker
/// * `CC` - Compute command pools presence marker
/// * `CT` - Transfer command pools presence marker
/// * `CS` - Sparse command pools presence marker
/// * `CP` - Protected command pools presence marker
pub struct BaseConfig<I, D, S, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    pub pools: command::CommandPools<CG, CC, CT, CS, CP>,
    pub swapchain: FieldConfig<S, swapchain::Swapchain, swapchain::SwapchainInfo>,
    pub device: FieldConfig<D, device::Device, device::DeviceInfo>,
    pub device_constraints: device::DeviceConstraints,
    pub instance: FieldConfig<I, instance::Instance, instance::InstanceInfo>,
}

impl Default for BaseConfig<Absent, Absent, Absent, Absent, Absent, Absent, Absent, Absent> {
    fn default() -> Self {
        Self {
            pools: Default::default(),
            swapchain: (),
            device: (),
            device_constraints: Default::default(),
            instance: (),
        }
    }
}

impl<I, D, S, CG, CC, CT, CS, CP> BaseConfig<I, D, S, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo> + instance::CreateInstance<I>,
    D: Store<device::Device, device::DeviceInfo> + device::CreateDevice<D, I>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo> + swapchain::CreateSwapchain<S, D, I>,
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
    /// Builds the configuration into a runtime [`Base`] instance.
    ///
    /// This method consumes the configuration and creates all the Vulkan resources
    ///
    /// # Returns
    ///
    /// A [`Base`] instance containing all the created Vulkan resources.
    ///
    /// # Errors
    ///
    /// Returns a Vulkan error if any resource creation fails. Common errors include:
    /// - `ERROR_INITIALIZATION_FAILED` - Failed to load Vulkan entry point
    /// - `ERROR_FEATURE_NOT_PRESENT` - No suitable device found
    #[allow(clippy::type_complexity)]
    pub fn build(self) -> Result<Base<I, D, S, CG, CC, CT, CS, CP>, vk::Result> {
        let entry =
            unsafe { ash::Entry::load().map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)? };
        let instance = I::create(self.instance, &entry)?;
        let device = D::create(self.device, &entry, &instance, self.device_constraints)?;
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
        let swapchain = S::create(self.swapchain, &entry, &instance, &device)?;

        Ok(Base {
            pools,
            swapchain,
            device,
            instance,
            entry,
        })
    }

    /// Applies a configuration item to this configuration.
    ///
    /// This is the primary method for building up the configuration. Each call to
    /// `with` potentially changes the type of the configuration as resources
    /// transition from [`Absent`] to [`Present`].
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of configuration item to apply (must implement [`Apply`])
    ///
    /// # Arguments
    ///
    /// * `opt` - The configuration item to apply
    ///
    /// # Returns
    ///
    /// A new configuration with the item applied and potentially different types.
    pub fn with<T: Apply<Self>>(self, opt: T) -> T::Out {
        opt.apply(self)
    }
}

impl<I, D, S, CG, CC, CT, CS, CP> fmt::Debug for Base<I, D, S, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    FieldInfo<S, swapchain::Swapchain, swapchain::SwapchainInfo>: fmt::Debug,
    command::CommandPoolInfos<CG, CC, CT, CS, CP>: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Base")
            .field("Instance", &self.instance)
            .field("Device", &self.device)
            .field("Swapchain", &self.swapchain)
            .field("Pools", &self.pools)
            .finish()
    }
}

impl<D, S, CG, CC, CT, CS, CP> Base<Present, D, S, CG, CC, CT, CS, CP>
where
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets a reference to the Vulkan instance.
    ///
    /// This method is only available when the instance is [`Present`].
    pub fn instance(&self) -> &instance::InstanceInfo {
        &self.instance
    }

    /// Gets a mutable reference to the Vulkan instance.
    ///
    /// This method is only available when the instance is [`Present`].
    pub fn instance_mut(&mut self) -> &mut instance::InstanceInfo {
        &mut self.instance
    }
}

// Device accessors - only when D = Present
impl<I, S, CG, CC, CT, CS, CP> Base<I, Present, S, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets a reference to the logical device.
    ///
    /// This method is only available when the device is [`Present`].
    pub fn device(&self) -> &device::DeviceInfo {
        &self.device
    }

    /// Gets a mutable reference to the logical device.
    ///
    /// This method is only available when the device is [`Present`].
    pub fn device_mut(&mut self) -> &mut device::DeviceInfo {
        &mut self.device
    }
}

impl<I, D, CG, CC, CT, CS, CP> Base<I, D, Present, CG, CC, CT, CS, CP>
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
    /// Gets a mutable reference to the swapchain.
    ///
    /// This method is only available when the swapchain is [`Present`].
    pub fn swapchain(&self) -> &swapchain::SwapchainInfo {
        &self.swapchain
    }

    /// Gets a mutable reference to the swapchain.
    ///
    /// This method is only available when the swapchain is [`Present`].
    pub fn swapchain_mut(&mut self) -> &mut swapchain::SwapchainInfo {
        &mut self.swapchain
    }
}

impl<I, D, S, CC, CT, CS, CP> Base<I, D, S, Present, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets references to all graphics command pools.
    ///
    /// This method is only available when graphics pools are [`Present`].
    pub fn graphics_pools(&self) -> &Vec<command::CommandPoolInfo<families::Graphics>> {
        &self.pools.graphics
    }

    /// Gets mutable references to all graphics command pools.
    ///
    /// This method is only available when graphics pools are [`Present`].
    pub fn graphics_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Graphics>> {
        &mut self.pools.graphics
    }
}

impl<I, D, S, CG, CT, CS, CP> Base<I, D, S, CG, Present, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets references to all compute command pools.
    ///
    /// This method is only available when compute pools are [`Present`].
    pub fn compute_pools(&self) -> &Vec<command::CommandPoolInfo<families::Compute>> {
        &self.pools.compute
    }

    /// Gets mutable references to all compute command pools.
    ///
    /// This method is only available when compute pools are [`Present`].
    pub fn compute_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Compute>> {
        &mut self.pools.compute
    }
}

impl<I, D, S, CG, CC, CS, CP> Base<I, D, S, CG, CC, Present, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets references to all transfer command pools.
    ///
    /// This method is only available when compute pools are [`Present`].
    pub fn transfer_pools(&self) -> &Vec<command::CommandPoolInfo<families::Transfer>> {
        &self.pools.transfer
    }

    /// Gets mutable references to all transfer command pools.
    ///
    /// This method is only available when transfer pools are [`Present`].
    pub fn transfer_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Transfer>> {
        &mut self.pools.transfer
    }
}

impl<I, D, S, CG, CC, CT, CP> Base<I, D, S, CG, CC, CT, Present, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets mutable references to all transfer command pools.
    ///
    /// This method is only available when transfer pools are [`Present`].
    pub fn sparse_pools(&self) -> &Vec<command::CommandPoolInfo<families::Sparse>> {
        &self.pools.sparse
    }

    /// Gets mutable references to all sparse command pools.
    ///
    /// This method is only available when sparse pools are [`Present`].
    pub fn sparse_pools_mut(&mut self) -> &mut Vec<command::CommandPoolInfo<families::Sparse>> {
        &mut self.pools.sparse
    }
}

impl<I, D, S, CG, CC, CT, CS> Base<I, D, S, CG, CC, CT, CS, Present>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets references to all protected command pools.
    ///
    /// This method is only available when protected pools are [`Present`].
    pub fn protected_pools(&self) -> &Vec<command::CommandPoolInfo<families::Protected>> {
        &self.pools.protected
    }

    /// Gets mutable references to all protected command pools.
    ///
    /// This method is only available when protected pools are [`Present`].
    pub fn protected_pools_mut(
        &mut self,
    ) -> &mut Vec<command::CommandPoolInfo<families::Protected>> {
        &mut self.pools.protected
    }
}

impl<I, D, S, CG, CC, CT, CS, CP> Base<I, D, S, CG, CC, CT, CS, CP>
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    S: Store<swapchain::Swapchain, swapchain::SwapchainInfo>,
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
    /// Gets a reference to the Vulkan entry point.
    ///
    /// The entry point is always available regardless of configuration.
    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }

    /// Gets a mutable reference to the Vulkan entry point.
    ///
    /// The entry point is always available regardless of configuration.
    pub fn entry_mut(&mut self) -> &mut ash::Entry {
        &mut self.entry
    }
}
