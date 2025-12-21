use crate::base::BaseConfig;
use crate::command;
use crate::device;
use crate::families;
use crate::instance;
use crate::Apply;
use crate::{Absent, Present, Store};
use ash::{khr, vk};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::Arc;

pub trait CreateSwapchain<S, D, I>
where
    S: Store<Swapchain, SwapchainInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    I: Store<instance::Instance, instance::InstanceInfo>,
{
    fn create(
        config: S::StoredConfig,
        entry: &ash::Entry,
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
        _entry: &ash::Entry,
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
        entry: &ash::Entry,
        instance: &instance::InstanceInfo,
        device: &device::DeviceInfo,
    ) -> Result<SwapchainInfo, vk::Result> {
        config.create(entry, &instance.0, device)
    }
}

pub struct SwapchainInfo {
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_loader: khr::swapchain::Device,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: khr::surface::Instance,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    device: Arc<ash::Device>,
}

impl Drop for SwapchainInfo {
    fn drop(&mut self) {
        unsafe {
            for &image_view in &self.image_views {
                self.device.destroy_image_view(image_view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}

struct SurfaceSource {
    window: raw_window_handle::RawWindowHandle,
    display: raw_window_handle::RawDisplayHandle,
}

pub struct Swapchain {
    surface: SurfaceSource,

    min_image_count: u32,
    image_formats: Vec<vk::Format>,
    image_sharing_mode: vk::SharingMode,
    color_spaces: Vec<vk::ColorSpaceKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
    image_usage_flags: vk::ImageUsageFlags,
    surface_transform_flags: vk::SurfaceTransformFlagsKHR,
    composite_alpha_flags: Vec<vk::CompositeAlphaFlagsKHR>,
    array_layers: u32,
    extent: Option<vk::Extent2D>,
    clipped: bool,
}

impl Swapchain {
    pub fn from_window<W: HasDisplayHandle + HasWindowHandle>(window: &W) -> Swapchain {
        Swapchain {
            surface: SurfaceSource {
                window: window.window_handle().unwrap().as_raw(),
                display: window.display_handle().unwrap().as_raw(),
            },
            min_image_count: 2,
            image_formats: vec![
                vk::Format::B8G8R8A8_SRGB,
                vk::Format::R8G8B8A8_SRGB,
                vk::Format::B8G8R8A8_UNORM,
                vk::Format::R8G8B8A8_UNORM,
            ],
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            color_spaces: vec![vk::ColorSpaceKHR::SRGB_NONLINEAR],
            present_modes: vec![
                vk::PresentModeKHR::MAILBOX,
                vk::PresentModeKHR::IMMEDIATE,
                vk::PresentModeKHR::FIFO,
                vk::PresentModeKHR::FIFO_RELAXED,
            ],
            image_usage_flags: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            surface_transform_flags: vk::SurfaceTransformFlagsKHR::IDENTITY,
            composite_alpha_flags: vec![
                vk::CompositeAlphaFlagsKHR::OPAQUE,
                vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
                vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
                vk::CompositeAlphaFlagsKHR::INHERIT,
            ],
            array_layers: 1,
            extent: None,
            clipped: true,
        }
    }
}

impl Swapchain {
    pub fn min_img_count(mut self, count: u32) -> Self {
        self.min_image_count = count;
        self
    }

    pub fn img_format(mut self, format: Vec<vk::Format>) -> Self {
        self.image_formats = format;
        self
    }

    pub fn image_sharing_mode(mut self, mode: vk::SharingMode) -> Self {
        self.image_sharing_mode = mode;
        self
    }

    pub fn color_space(mut self, color_space: Vec<vk::ColorSpaceKHR>) -> Self {
        self.color_spaces = color_space;
        self
    }

    pub fn present_mode(mut self, mode: Vec<vk::PresentModeKHR>) -> Self {
        self.present_modes = mode;
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

    pub fn composite_alpha_flags(mut self, flags: Vec<vk::CompositeAlphaFlagsKHR>) -> Self {
        self.composite_alpha_flags = flags;
        self
    }

    pub fn array_layers(mut self, layers: u32) -> Self {
        self.array_layers = layers;
        self
    }

    pub fn extent(mut self, extesnt: vk::Extent2D) -> Self {
        self.extent = Some(extesnt);
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
        entry: &ash::Entry,
        instance: &ash::Instance,
        device: &device::DeviceInfo,
    ) -> Result<SwapchainInfo, vk::Result> {
        let swapchain_loader = khr::swapchain::Device::new(instance, &device.device);
        let surface_loader = khr::surface::Instance::new(entry, instance);

        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                self.surface.display,
                self.surface.window,
                None,
            )?
        };

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
            .filter_map(|sf| {
                let format_idx = self.image_formats.iter().position(|&f| f == sf.format)?;
                let color_idx = self
                    .color_spaces
                    .iter()
                    .position(|&c| c == sf.color_space)?;
                Some((format_idx, color_idx, *sf))
            })
            .min_by_key(|(f_idx, c_idx, _)| (*f_idx, *c_idx))
            .map(|(_, _, sf)| sf)
            .ok_or(vk::Result::ERROR_FORMAT_NOT_SUPPORTED)?;
        // TODO: make that return the mode that contains most specified modes
        let present_mode = self
            .present_modes
            .iter()
            .find(|&&mode| present_modes.contains(&mode))
            .copied()
            .expect("make that error propagate");

        let extent = if let Some(fixed_extent) = self.extent {
            vk::Extent2D {
                width: fixed_extent.width.clamp(
                    surface_caps.min_image_extent.width,
                    surface_caps.max_image_extent.width,
                ),
                height: fixed_extent.height.clamp(
                    surface_caps.min_image_extent.height,
                    surface_caps.max_image_extent.height,
                ),
            }
        } else if surface_caps.current_extent.width != u32::MAX {
            surface_caps.current_extent
        } else {
            surface_caps.min_image_extent
        };

        // TODO: consider returning error instead of using .max()
        let mut image_count = self.min_image_count.max(surface_caps.min_image_count);
        if surface_caps.max_image_count > 0 {
            // TODO: same here consider returning error instead of using .min() or allow user to
            // specify allowed clamp
            image_count = image_count.min(surface_caps.max_image_count);
        }

        // TODO: Also consider returning error instead
        let pre_transform = if surface_caps
            .supported_transforms
            .contains(self.surface_transform_flags)
        {
            self.surface_transform_flags
        } else {
            surface_caps.current_transform
        };

        let composite_alpha = self
            .composite_alpha_flags
            .iter()
            .find(|&&mode| surface_caps.supported_composite_alpha.contains(mode))
            .copied()
            .ok_or(vk::Result::ERROR_FORMAT_NOT_SUPPORTED)?;

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(self.array_layers)
            .image_usage(self.image_usage_flags)
            .image_sharing_mode(self.image_sharing_mode)
            .pre_transform(pre_transform)
            .composite_alpha(composite_alpha)
            .present_mode(present_mode)
            .clipped(self.clipped);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None)? };
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
            surface,
            surface_loader,
            images,
            image_views,
            format: surface_format.format,
            extent,
            device: Arc::clone(&device.device),
        })
    }
}

pub struct SwapchainRequirements {
    pub formats: Vec<vk::Format>,
    pub color_spaces: Vec<vk::ColorSpaceKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
    pub image_usage: vk::ImageUsageFlags,
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
        device_constraints.required_swapchain = Some(SwapchainRequirements {
            formats: self.image_formats.clone(),
            color_spaces: self.color_spaces.clone(),
            present_modes: self.present_modes.clone(),
            image_usage: self.image_usage_flags,
        });
        let mut instance_constraints = config.instance_constraints;
        instance_constraints.required_surface = Some(self.surface.display);

        BaseConfig {
            instance: config.instance,
            instance_constraints,
            swapchain: self,
            device: config.device,
            device_constraints,
            pools: config.pools,
        }
    }
}
