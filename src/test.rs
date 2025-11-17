use ash::vk;
use std::ffi::CString;

fn main() {
    // TODO: consider moving require_graphics_queue and synonyms into with_queue or
    // with_command_pool in the way that required queues are abstracted into usage of that queues
    let _base = vinit::BaseConfig::default()
        .with_app_name(CString::from(c"Super Cool App"))
        .with_app_version((0, 0, 1))
        .with_instance_extensions([].into())
        // TODO: Consider creating device_config instead which will take all the possible
        // parameters not only ones suitable for selecting physical_device if that isn't to much of
        // an overhead
        .with_device(|physical_device_selector| {
            physical_device_selector
                .require_discrete(false)
                .prefer_best(true)
        })
        .with_device_extensions([CString::from(vk::KHR_SWAPCHAIN_NAME)].into())
        .with_swapchain(|swapchain_config| {
            swapchain_config
                .min_img_count(12)
                .img_format(vk::Format::R8G8B8A8_SRGB)
        })
        .build();
}
