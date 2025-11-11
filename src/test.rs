use ash::vk;

fn main() {
    let base = vinit::BaseConfig::default()
        .with_app_info(c"TEST", 0, 0, 0)
        .with_device(|physical_device_selector| {
            physical_device_selector
                .require_graphics_queue()
                .require_compute_queue()
                .require_transfer_queue()
        })
        .with_swapchain(|swapchain_config| {
            swapchain_config
                .min_img_count(12)
                .img_format(vk::Format::R8G8B8A8_SRGB)
        })
        .create();
}
