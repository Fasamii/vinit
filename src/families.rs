use ash::vk;

/// Trait defining a Vulkan queue family type.
///
/// This trait is implemented by zero-sized marker types representing different
/// queue families.
///
/// # Implementors
///
/// - [`Graphics`]
/// - [`Compute`]
/// - [`Transfer`]
/// - [`Sparse`]
/// - [`Protected`]
pub trait QueueFamily: Sized {
    /// Accesses the field for this queue family.
    fn access<T>(families: &Families<T>) -> &T;
    /// Mutably accesses the field for this queue family.
    fn access_mut<T>(families: &mut Families<T>) -> &mut T;
}

/// Marker type for graphics queue family.
pub struct Graphics;
impl QueueFamily for Graphics {
    fn access<T>(families: &Families<T>) -> &T {
        &families.graphics
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.graphics
    }
}

/// Marker type for compute queue family.
pub struct Compute;
impl QueueFamily for Compute {
    fn access<T>(families: &Families<T>) -> &T {
        &families.compute
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.compute
    }
}

/// Marker type for transfer queue family.
pub struct Transfer;
impl QueueFamily for Transfer {
    fn access<T>(families: &Families<T>) -> &T {
        &families.transfer
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.transfer
    }
}

/// Marker type for sparse binding queue family
pub struct Sparse;
impl QueueFamily for Sparse {
    fn access<T>(families: &Families<T>) -> &T {
        &families.sparse
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.sparse
    }
}

/// Marker type for protected memory queue family.
pub struct Protected;
impl QueueFamily for Protected {
    fn access<T>(families: &Families<T>) -> &T {
        &families.protected
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.protected
    }
}

/// Container for data associated with each queue family type.
///
/// This struct stores one value for each queue family type (graphics, compute,
/// transfer, sparse, and protected). It provides type-safe access through the
/// [`QueueFamily`] trait.
#[derive(Clone, Copy, Debug)]
pub struct Families<T> {
    pub graphics: T,
    pub compute: T,
    pub transfer: T,
    pub sparse: T,
    pub protected: T,
}

impl<T: Default> Default for Families<T> {
    fn default() -> Self {
        Self {
            graphics: Default::default(),
            compute: Default::default(),
            transfer: Default::default(),
            sparse: Default::default(),
            protected: Default::default(),
        }
    }
}

impl<T> Families<T> {
    /// Gets a reference to the data for a specific queue family.
    ///
    /// # Type Parameters
    ///
    /// `Q` - The queue family type to access
    pub fn get<Q: QueueFamily>(&self) -> &T {
        Q::access(self)
    }

    /// Gets a mutable reference to the data for a specific queue family.
    ///
    /// # Type Parameters
    ///
    /// `Q` - The queue family type to access
    pub fn get_mut<Q: QueueFamily>(&mut self) -> &mut T {
        Q::access_mut(self)
    }

    /// Sets the value for a specific queue family.
    ///
    /// # Type Parameters
    ///
    /// `Q` - The queue family type to set
    ///
    /// # Arguments
    ///
    /// `value` - The value to set
    pub fn set<Q: QueueFamily>(&mut self, value: T) {
        *Q::access_mut(self) = value;
    }
}

impl Families<Option<u32>> {
    /// Queries available queue families from a physical device.
    ///
    /// This function returns which queue families are available and their indices for passed
    /// device.
    pub fn query(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        let mut families: Self = Default::default();
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        for (idx, properties) in queue_family_properties.iter().enumerate() {
            let idx = idx as u32;
            let flags = properties.queue_flags;

            if flags.contains(vk::QueueFlags::GRAPHICS) && families.graphics.is_none() {
                families.graphics = Some(idx);
            }
            if flags.contains(vk::QueueFlags::COMPUTE) && families.compute.is_none() {
                families.compute = Some(idx);
            }
            if flags.contains(vk::QueueFlags::TRANSFER) && families.transfer.is_none() {
                families.transfer = Some(idx);
            }
            if flags.contains(vk::QueueFlags::SPARSE_BINDING) && families.sparse.is_none() {
                families.sparse = Some(idx);
            }
            if flags.contains(vk::QueueFlags::PROTECTED) && families.protected.is_none() {
                families.protected = Some(idx);
            }
        }

        families
    }

    /// Returns a vector of unique queue family indices.
    pub fn unique_indices(&self) -> Vec<u32> {
        let mut set = std::collections::HashSet::new();

        if let Some(idx) = self.graphics {
            set.insert(idx);
        }
        if let Some(idx) = self.compute {
            set.insert(idx);
        }
        if let Some(idx) = self.transfer {
            set.insert(idx);
        }
        if let Some(idx) = self.sparse {
            set.insert(idx);
        }
        if let Some(idx) = self.protected {
            set.insert(idx);
        }

        set.into_iter().collect()
    }

    /// Creates Vulkan queue create info structures.
    pub fn make_create_info<'a>(
        &'a self,
        priorities: &'a [f32; 1],
    ) -> Vec<vk::DeviceQueueCreateInfo<'a>> {
        self.unique_indices()
            .into_iter()
            .map(|idx| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(idx)
                    .queue_priorities(priorities)
            })
            .collect()
    }

    /// Filters queue families to only those that are required.
    ///
    /// Sets indices to `None` for queue families not marked as required.
    ///
    /// # Arguments
    ///
    /// `required` - Boolean flags indicating which families are required
    pub fn filter_required(mut self, required: &Families<bool>) -> Self {
        if !required.graphics {
            self.graphics = None;
        }
        if !required.compute {
            self.compute = None;
        }
        if !required.transfer {
            self.transfer = None;
        }
        if !required.sparse {
            self.sparse = None;
        }
        if !required.protected {
            self.protected = None;
        }
        self
    }
}

impl Families<bool> {
    // TODO: Consider removing that function.
    /// Checks if any queue family is required.
    ///
    /// # Returns
    ///
    /// `true` if at least one queue family is marked as required
    pub fn any_required(&self) -> bool {
        self.graphics || self.compute || self.transfer || self.sparse || self.protected
    }

    // TODO: Consider removing that function.
    /// Checks if a specific queue family is required.
    ///
    /// # Type Parameters
    ///
    /// `Q` - The queue family type to check
    pub fn is_required<Q: QueueFamily>(&self) -> bool {
        *self.get::<Q>()
    }
}

impl Families<Option<vk::Queue>> {
    /// Creates queue handles from queue family indices.
    pub fn new(device: &ash::Device, indices: Families<Option<u32>>) -> Self {
        let graphics = indices
            .graphics
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let compute = indices
            .compute
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let transfer = indices
            .transfer
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let sparse = indices
            .sparse
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });
        let protected = indices
            .protected
            .map(|idx| unsafe { device.get_device_queue(idx, 0) });

        Self {
            graphics,
            compute,
            transfer,
            sparse,
            protected,
        }
    }
}
