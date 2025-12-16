// pub trait BuildSwapchain<S: Store<SwapchainInfo>> {
//     fn build_swapchain(
//         config: Option<SwapchainConfig>,
//         instance: &ash::Instance,
//         device: &device::DeviceInfo,
//     ) -> Result<S::Stored, vk::Result>;
// }
//
// impl BuildSwapchain<Absent> for Absent {
//     fn build_swapchain(
//         _config: Option<SwapchainConfig>,
//         _instance: &ash::Instance,
//         _device: &device::DeviceInfo,
//     ) -> Result<(), vk::Result> {
//         Ok(())
//     }
// }
//
// impl BuildSwapchain<Present> for Present {
//     fn build_swapchain(
//         config: Option<SwapchainConfig>,
//         instance: &ash::Instance,
//         device: &device::DeviceInfo,
//     ) -> Result<SwapchainInfo, vk::Result> {
//         SwapchainInfo::new(
//             config.unwrap_or_else(|| panic!("Attempt to create swapchain withot providing config")),
//             instance,
//             device,
//         )
//     }
// }
//
// pub struct SwapchainConfig {
//     min_image_count: u32,
//     image_format: vk::Format,
//     image_sharing_mode: vk::SharingMode,
//     color_space: vk::ColorSpaceKHR,
//     present_mode: vk::PresentModeKHR,
//     image_usage: vk::ImageUsageFlags,
//     transforms: vk::SurfaceTransformFlagsKHR,
//     composite_alpha: vk::CompositeAlphaFlagsKHR,
//     array_layers: u32,
//     extent: vk::Extent2D,
//     clipped: bool,
// }
//
// impl Default for SwapchainConfig {
//     fn default() -> Self {
//         Self {
//             min_image_count: 2,
//             image_format: vk::Format::R8G8B8A8_SRGB,
//             image_sharing_mode: vk::SharingMode::EXCLUSIVE,
//             color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
//             present_mode: vk::PresentModeKHR::FIFO,
//             image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
//             transforms: vk::SurfaceTransformFlagsKHR::IDENTITY,
//             composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
//             array_layers: 1,
//             extent: vk::Extent2D {
//                 width: 1920,
//                 height: 1080,
//             },
//             clipped: true,
//         }
//     }
// }
//
// impl SwapchainConfig {
//     pub fn min_img_count(mut self, count: u32) -> Self {
//         self.min_image_count = count;
//         self
//     }
//
//     pub fn img_format(mut self, format: vk::Format) -> Self {
//         self.image_format = format;
//         self
//     }
// }
//
// pub struct SwapchainInfo {
//     pub swapchain: vk::SwapchainKHR,
//     swapchain_loader: khr::swapchain::Device,
//     pub images: Vec<vk::Image>,
//     pub image_views: Vec<vk::ImageView>,
//     pub format: vk::Format,
//     pub extent: vk::Extent2D,
//     pub image_count: u32,
// }
//
// impl SwapchainInfo {
//     pub fn new(
//         config: SwapchainConfig,
//         instance: &ash::Instance,
//         device: &device::DeviceInfo,
//     ) -> Result<Self, vk::Result> {
//         let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
//             .surface(todo!())
//             .min_image_count(config.min_image_count)
//             .image_format(config.image_format)
//             .image_color_space(config.color_space)
//             .image_extent(config.extent)
//             .image_array_layers(config.array_layers)
//             .image_usage(config.image_usage)
//             .image_sharing_mode(config.image_sharing_mode)
//             .pre_transform(config.transforms)
//             .composite_alpha(config.composite_alpha)
//             .present_mode(config.present_mode)
//             .clipped(config.clipped);
//         let swapchain_loader = khr::swapchain::Device::new(instance, &device.device);
//         let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None)? };
//
//         let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };
//     }
// }
//
// impl Drop for SwapchainInfo {
//     fn drop(&mut self) {
//         unsafe {
//             self.swapchain_loader
//                 .destroy_swapchain(self.swapchain, None);
//         }
//     }
// }
