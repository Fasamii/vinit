use ash::vk;

fn main() {
    // TODO: consider moving require_graphics_queue and synonyms into with_queue or
    // with_command_pool in the way that required queues are abstracted into usage of that queues
    let _base = vinit::BaseConfig::default()
        .with_app_info(c"TEST", (0, 0, 0))
        .with_device(|physical_device_selector| {
            physical_device_selector
                .require_graphics_queue()
                .require_compute_queue()
                .require_transfer_queue()
                .require_discrete(false)
                .prefer_bset(true)
        })
        .with_swapchain(|swapchain_config| {
            swapchain_config
                .min_img_count(12)
                .img_format(vk::Format::R8G8B8A8_SRGB)
        })
        .build();
}
