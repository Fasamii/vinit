use ash::vk;
use std::ffi::CString;

fn main() {
    let mut base = vinit::BaseConfig::default()
        .with_app_name(CString::from(c"Super Cool App"))
        .with_app_version((0, 0, 1))
        .with_instance_extensions([].into())
        .with_device(|device_selector| {
            device_selector
                .require_features(
                    vk::PhysicalDeviceFeatures::default().draw_indirect_first_instance(true),
                )
                .require_properties(
                    vk::PhysicalDeviceProperties::default()
                        .device_name(c"")
                        .unwrap(),
                )
                .require_discrete(false)
                .prefer_best(true)
        })
        .with_device_extensions([CString::from(vk::KHR_SWAPCHAIN_NAME)].into());

    let _cmd_pool_graphics = base.add_command_pool(|config| config.graphics_queue());
    let _cmd_pool_transfer = base.add_command_pool(|config| config.transfer_queue());

    // .with_swapchain(|swapchain_config| {
    //     swapchain_config
    //         .min_img_count(12)
    //         .img_format(vk::Format::R8G8B8A8_SRGB)
    // })

    let _ = base.build();
}
