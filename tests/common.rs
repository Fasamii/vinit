use ash::vk;
use std::ffi::CString;
use vinit::*;

fn check<D, S, CG, CC, CT, CS, CP>(base: Result<Base<D, S, CG, CC, CT, CS, CP>, vk::Result>)
where
    D: Store<DeviceInfo>,
    S: Store<SwapchainInfo>,
    CG: Store<Vec<command::CommandPoolInfo<families::Graphics>>>,
    CC: Store<Vec<command::CommandPoolInfo<families::Compute>>>,
    CT: Store<Vec<command::CommandPoolInfo<families::Transfer>>>,
    CS: Store<Vec<command::CommandPoolInfo<families::Sparse>>>,
    CP: Store<Vec<command::CommandPoolInfo<vinit::families::Protected>>>,
{
    if base.is_err() {
        eprintln!("\x1b[38;5;1m [ ERR ]::[Failed to initialize vulkan]\x1b[0m");
        let _ = base.inspect_err(|err| {
            eprintln!("err = {err:?}");
        });
    } else {
        println!("\x1b[38;5;2m [ OK ]::[Initialization was succesfull]\x1b[0m");
    }
}

#[test]
fn create_empty() {
    // let base = BaseConfig::default().with_device(|config| config).build();

    // if base.is_err() {
    //     eprintln!("\x1b[38;5;1m [ ERR ]::[Failed to initialize vulkan]\x1b[0m");
    //     let _ = base.inspect_err(|err| {
    //         eprintln!("err = {err:?}");
    //     });
    // } else {
    //     println!("\x1b[38;5;2m [ OK ]::[Initialization was succesfull]\x1b[0m");
    // }
    panic!("You can't even use .build() method because of type system");
}

#[test]
fn create_device() {
    let base = vinit::BaseConfig::default()
        .with_app_name(CString::from(c"kms"))
        .with_app_version((0, 0, 1))
        .with_validation_layers(vec![CString::from(c"VK_LAYER_KHRONOS_validation")])
        .with_device(|config| config.prefer_best(true).require_discrete(false))
        .build();
    check(base);
}

#[test]
fn create_all() {
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
        .with_swapchain(|swapchain_config| {
            swapchain_config
                .min_img_count(12)
                .img_format(vk::Format::R8G8B8A8_SRGB)
        })
        .build();
    check(base);
}
