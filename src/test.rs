use ash::vk;
use std::ffi::CString;

fn main() {
    #[allow(unused_variables)]
    let base = vinit::BaseConfig::default()
        .with_app_name(CString::from(c"Super Cool App"))
        .with_app_version((0, 0, 1))
        .with_instance_extensions(vec![
            CString::from(c"VK_KHR_surface"),
            CString::from(c"VK_KHR_wayland_surface"),
        ])
        .with_validation_layers(vec![CString::from(c"VK_LAYER_KHRONOS_validation")])
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
        .with_device_extensions([CString::from(vk::KHR_SWAPCHAIN_NAME)].into())
        .add_command_pool(|config| config.graphics_queue())
        // .add_command_pool(|config| config.transfer_queue())
        // .add_command_pool(|config| config.compute_queue())
        // .with_swapchain(|swapchain_config| {
        //     swapchain_config
        //         .min_img_count(12)
        //         .img_format(vk::Format::R8G8B8A8_SRGB)
        // })
        .build();
}
