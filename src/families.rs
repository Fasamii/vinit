use ash::vk;
use std::marker::PhantomData;

pub trait QueueFamily: Sized {
    fn access<T>(families: &Families<T>) -> &T;
    fn access_mut<T>(families: &mut Families<T>) -> &mut T;
}

pub struct Graphics;
impl QueueFamily for Graphics {
    fn access<T>(families: &Families<T>) -> &T {
        &families.graphics
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.graphics
    }
}

pub struct Compute;
impl QueueFamily for Compute {
    fn access<T>(families: &Families<T>) -> &T {
        &families.compute
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.compute
    }
}

pub struct Transfer;
impl QueueFamily for Transfer {
    fn access<T>(families: &Families<T>) -> &T {
        &families.transfer
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.transfer
    }
}

pub struct Sparse;
impl QueueFamily for Sparse {
    fn access<T>(families: &Families<T>) -> &T {
        &families.sparse
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.sparse
    }
}

pub struct Protected;
impl QueueFamily for Protected {
    fn access<T>(families: &Families<T>) -> &T {
        &families.protected
    }
    fn access_mut<T>(families: &mut Families<T>) -> &mut T {
        &mut families.protected
    }
}

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
    pub fn get<Q: QueueFamily>(&self) -> &T {
        Q::access(self)
    }
    pub fn get_mut<Q: QueueFamily>(&mut self) -> &mut T {
        Q::access_mut(self)
    }
    pub fn set<Q: QueueFamily>(&mut self, value: T) {
        *Q::access_mut(self) = value;
    }
}

impl Families<Option<u32>> {
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
    pub fn any_required(&self) -> bool {
        self.graphics || self.compute || self.transfer || self.sparse || self.protected
    }

    pub fn is_required<Q: QueueFamily>(&self) -> bool {
        *self.get::<Q>()
    }
}

impl Families<Option<vk::Queue>> {
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
