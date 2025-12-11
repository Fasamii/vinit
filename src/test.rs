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
                .require_properties(vk::PhysicalDeviceProperties::default())
                .require_discrete(false)
                .prefer_best(true)
        })
        .with_device_extensions([CString::from(vk::KHR_SWAPCHAIN_NAME)].into())
        .with_graphics_pool(|config| config)
        .with_transfer_pool(|config| config)
        .with_compute_pool(|config| config)
        // .with_protected_pool(|config| config)
        // .add_command_pool(|config| config.protected_queue())
        // .with_swapchain(|swapchain_config| {
        //     swapchain_config
        //         .min_img_count(12)
        //         .img_format(vk::Format::R8G8B8A8_SRGB)
        // })
        .build();

    if base.is_err() {
        eprintln!("\x1b[38;5;1m [ ERR ]::[Failed to initialize vulkan]\x1b[0m");
        let _ = base.inspect_err(|err| {
            eprintln!("err = {err:?}");
        });
    } else {
        println!("\x1b[38;5;2m [ OK ]::[Initialization was succesfull]\x1b[0m");
    }
}
