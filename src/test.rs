use ash::vk;
use std::ffi::CString;

fn main() {
    let mut base = vinit::BaseConfig::default()
        .with_app_name(CString::from(c"Super Cool App"))
        .with_app_version((0, 0, 1))
        .with_instance_extensions([].into())
        // TODO: Consider creating device_config instead which will take all the possible
        // parameters not only ones suitable for selecting physical_device if that isn't to much of
        // an overhead
        .with_device(|physical_device_selector| {
            physical_device_selector
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
        let cb = base.with_command_pool::<vinit::families::Graphics>(todo!("create cmd pool config"));
        let cb = base.with_command_pool::<vinit::families::Transfer>(todo!("create cmd pool config"));

        // .with_swapchain(|swapchain_config| {
        //     swapchain_config
        //         .min_img_count(12)
        //         .img_format(vk::Format::R8G8B8A8_SRGB)
        // })

        let _ = base.build();
}
