use crate::device;
use ash::{khr, vk};

pub struct SwapchainInfo {
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_loader: khr::swapchain::Device,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
    pub image_count: u32,
}

impl SwapchainInfo {}

impl Drop for SwapchainInfo {
    fn drop(&mut self) {
        unsafe {
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
    image_usage: vk::ImageUsageFlags,
    transforms: vk::SurfaceTransformFlagsKHR,
    composite_alpha: vk::CompositeAlphaFlagsKHR,
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
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
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
}

impl Swapchain {
    pub fn create(
        self,
        instance: &ash::Instance,
        device: &device::DeviceInfo,
    ) -> Result<SwapchainInfo, vk::Result> {
        let surface = todo!("Read vulkan docs");

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface) // TODO: Check if surface field for create info is required
            .min_image_count(self.min_image_count)
            .image_format(self.image_format)
            .image_color_space(self.color_space)
            .image_extent(self.extent)
            .image_array_layers(self.array_layers)
            .image_usage(self.image_usage)
            .image_sharing_mode(self.image_sharing_mode)
            .pre_transform(self.transforms)
            .composite_alpha(self.composite_alpha)
            .present_mode(self.present_mode)
            .clipped(self.clipped);
        let swapchain_loader = khr::swapchain::Device::new(instance, &device.device);
        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None)? };
        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };

        Ok(SwapchainInfo {
            swapchain,
            swapchain_loader,
            images: todo!(),
            image_views: todo!(),
            format: todo!(),
            extent: todo!(),
            image_count: todo!(),
        })
    }
}
