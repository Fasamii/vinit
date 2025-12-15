use ash::vk;
use std::ffi::CString;
use std::sync::Once;
use vinit::*;

static INIT: Once = Once::new();

fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder().is_test(true).try_init().ok();
    });
}

#[test]
fn create_empty() {
    init_logger();
    let _base = vinit::BaseConfig::default()
        .build()
        .expect("Failed to create Base");
}

#[test]
fn create_instance() {
    init_logger();
    let _base = vinit::BaseConfig::default()
        .with(
            instance::Instance::default()
                .api_version(0, 3, 1)
                .validation(vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()])
                .app_name(CString::new("TEST").unwrap())
                .app_version(0, 0, 0),
        )
        .build()
        .expect("Failed to create Base");
}

#[test]
fn create_device() {
    init_logger();
    let _base = vinit::BaseConfig::default()
        .with(
            instance::Instance::default()
                .validation(vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]),
        )
        .with(
            device::Device::default()
                .require_features(
                    vk::PhysicalDeviceFeatures::default()
                        .alpha_to_one(true)
                        .occlusion_query_precise(true),
                )
                .require_properties(
                    vk::PhysicalDeviceProperties::default().limits(
                        vk::PhysicalDeviceLimits::default()
                            .max_fragment_combined_output_resources(1235),
                    ),
                ),
        )
        .build()
        .expect("Failed to create Base");
}

#[test]
fn create_pool() {
    init_logger();
    let _base = BaseConfig::default()
        .with(
            instance::Instance::default()
                .app_name(CString::new("My App").unwrap())
                .validation(vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]),
        )
        .with(
            device::Device::default()
                .require_features(
                    vk::PhysicalDeviceFeatures::default()
                        .alpha_to_one(true)
                        .occlusion_query_precise(true),
                )
                .require_properties(
                    vk::PhysicalDeviceProperties::default().limits(
                        vk::PhysicalDeviceLimits::default()
                            .max_fragment_combined_output_resources(1235),
                    ),
                ),
        )
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::compute().flags(vk::CommandPoolCreateFlags::empty()))
        .build()
        .expect("Failed to create Base");
}

#[test]
fn base_lifetimes() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .build()
        .expect("Failed to create Base");

    {
        let _instance = base.instance();
    }

    {
        let _command = base.graphics_pools();
    }
}
