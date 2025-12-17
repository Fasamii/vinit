#![allow(unreachable_code)]

use std::sync::Arc;

use crate::base::BaseConfig;
use crate::command;
use crate::device;
use crate::families;
use crate::instance;
use crate::Apply;
use crate::{Absent, Present, Store};
use ash::{khr, vk};

pub trait CreateSwapchain<S, D, I>
where
    S: Store<Swapchain, SwapchainInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    I: Store<instance::Instance, instance::InstanceInfo>,
{
    fn create(
        config: S::StoredConfig,
        instance: &I::StoredInfo,
        device: &D::StoredInfo,
    ) -> Result<S::StoredInfo, vk::Result>;
}

impl<D, I> CreateSwapchain<Absent, D, I> for Absent
where
    D: Store<device::Device, device::DeviceInfo>,
    I: Store<instance::Instance, instance::InstanceInfo>,
{
    fn create(
        _config: (),
        _instance: &I::StoredInfo,
        _device: &D::StoredInfo,
    ) -> Result<(), vk::Result> {
        Ok(())
    }
}

#[allow(unused)]
impl CreateSwapchain<Present, Present, Present> for Present {
    fn create(
        config: Swapchain,
        instance: &instance::InstanceInfo,
        device: &device::DeviceInfo,
    ) -> Result<SwapchainInfo, vk::Result> {
        // config.create(&instance.0, device)
        todo!("Call create when arguments to it stablize");
    }
}

pub struct SwapchainInfo {
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_loader: khr::swapchain::Device,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    device: Arc<ash::Device>,
}

impl SwapchainInfo {}

impl Drop for SwapchainInfo {
    fn drop(&mut self) {
        unsafe {
            for &image_view in &self.image_views {
                self.device.destroy_image_view(image_view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }
}

pub struct Swapchain {

    min_image_count: u32,
    image_format: vk::Format,
    image_sharing_mode: vk::SharingMode,
    color_space: vk::ColorSpaceKHR,
    present_mode: vk::PresentModeKHR,
    image_usage_flags: vk::ImageUsageFlags,
    surface_transform_flags: vk::SurfaceTransformFlagsKHR,
    composite_alpha_flags: vk::CompositeAlphaFlagsKHR,
    array_layers: u32,
    extent: vk::Extent2D,
    clipped: bool,
}

impl Default for Swapchain {
    fn default() -> Self {
        Self {
            min_image_count: 2,
            image_format: vk::Format::R8G8B8A8_SRGB,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            present_mode: vk::PresentModeKHR::FIFO,
            image_usage_flags: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            surface_transform_flags: vk::SurfaceTransformFlagsKHR::IDENTITY,
            composite_alpha_flags: vk::CompositeAlphaFlagsKHR::OPAQUE,
            array_layers: 1,
            extent: vk::Extent2D {
                width: 1920,
                height: 1080,
            },
            clipped: true,
        }
    }
}

impl Swapchain {
    pub fn min_img_count(mut self, count: u32) -> Self {
        self.min_image_count = count;
        self
    }

    pub fn img_format(mut self, format: vk::Format) -> Self {
        self.image_format = format;
        self
    }

    pub fn image_sharing_mode(mut self, mode: vk::SharingMode) -> Self {
        self.image_sharing_mode = mode;
        self
    }

    pub fn color_space(mut self, color_space: vk::ColorSpaceKHR) -> Self {
        self.color_space = color_space;
        self
    }

    pub fn present_mode(mut self, mode: vk::PresentModeKHR) -> Self {
        self.present_mode = mode;
        self
    }

    pub fn image_usage_flags(mut self, flags: vk::ImageUsageFlags) -> Self {
        self.image_usage_flags = flags;
        self
    }

    pub fn surface_transform_flags(mut self, flags: vk::SurfaceTransformFlagsKHR) -> Self {
        self.surface_transform_flags = flags;
        self
    }

    pub fn composite_alpha_flags(mut self, flags: vk::CompositeAlphaFlagsKHR) -> Self {
        self.composite_alpha_flags = flags;
        self
    }

    pub fn array_layers(mut self, layers: u32) -> Self {
        self.array_layers = layers;
        self
    }

    pub fn extent(mut self, extesnt: vk::Extent2D) -> Self {
        self.extent = extesnt;
        self
    }

    pub fn clipped(mut self, clipped: bool) -> Self {
        self.clipped = clipped;
        self
    }
}

impl Swapchain {
    fn create(
        self,
        instance: &ash::Instance,
        device: &device::DeviceInfo,
        surface: vk::SurfaceKHR,
    ) -> Result<SwapchainInfo, vk::Result> {
        let swapchain_loader = khr::swapchain::Device::new(instance, &device.device);
        let surface_loader = khr::surface::Instance::new(todo!(), instance);

        let (window_width, window_height) = (1920, 1080);

        // TODO: You should move that to DeviceSelector.
        let surface_caps = unsafe {
            surface_loader.get_physical_device_surface_capabilities(
                device.physical.physical_device,
                surface,
            )?
        };

        let surface_formats = unsafe {
            surface_loader
                .get_physical_device_surface_formats(device.physical.physical_device, surface)?
        };

        let present_modes = unsafe {
            surface_loader.get_physical_device_surface_present_modes(
                device.physical.physical_device,
                surface,
            )?
        };

        let surface_format = surface_formats
            .iter()
            .find(|f| {
                f.format == vk::Format::B8G8R8A8_SRGB
                    && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or(&surface_formats[0]);

        let present_mode = present_modes
            .iter()
            .copied()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let extent = if surface_caps.current_extent.width != u32::MAX {
            surface_caps.current_extent
        } else {
            vk::Extent2D {
                width: window_width.clamp(
                    surface_caps.min_image_extent.width,
                    surface_caps.max_image_extent.width,
                ),
                height: window_height.clamp(
                    surface_caps.min_image_extent.height,
                    surface_caps.max_image_extent.height,
                ),
            }
        };

        let image_count = {
            let mut count = surface_caps.min_image_count + 1;
            if surface_caps.max_image_count > 0 {
                count = count.min(surface_caps.max_image_count);
            }
            count
        };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface) // TODO: Check if surface field for create info is required
            .min_image_count(self.min_image_count)
            .image_format(self.image_format)
            .image_color_space(self.color_space)
            .image_extent(self.extent)
            .image_array_layers(self.array_layers)
            .image_usage(self.image_usage_flags)
            .image_sharing_mode(self.image_sharing_mode)
            .pre_transform(self.surface_transform_flags)
            .composite_alpha(self.composite_alpha_flags)
            .present_mode(self.present_mode)
            .clipped(self.clipped);
        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None)? };
        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };

        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };
        let image_views = images
            .iter()
            .map(|&image| {
                let create_info = vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(surface_format.format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::IDENTITY,
                        g: vk::ComponentSwizzle::IDENTITY,
                        b: vk::ComponentSwizzle::IDENTITY,
                        a: vk::ComponentSwizzle::IDENTITY,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    });

                unsafe { device.device.create_image_view(&create_info, None) }
            })
            .collect::<ash::prelude::VkResult<Vec<_>>>()?;

        Ok(SwapchainInfo {
            swapchain,
            swapchain_loader,
            images: todo!(),
            image_views: todo!(),
            format: todo!(),
            extent: todo!(),
            device: Arc::clone(&device.device),
        })
    }
}

impl<CG, CC, CT, CS, CP> Apply<BaseConfig<Present, Present, Absent, CG, CC, CT, CS, CP>>
    for Swapchain
where
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
    type Out = BaseConfig<Present, Present, Present, CG, CC, CT, CS, CP>;
    fn apply(self, config: BaseConfig<Present, Present, Absent, CG, CC, CT, CS, CP>) -> Self::Out {
        let mut device_constraints = config.device_constraints;
        device_constraints.required_swapchain = true;
        BaseConfig {
            instance: config.instance,
            swapchain: self,
            device: config.device,
            device_constraints,
            pools: config.pools,
        }
    }
}
