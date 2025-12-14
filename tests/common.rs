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

fn check<I, D, CG, CC, CT, CS, CP>(base: Result<Base<I, D, CG, CC, CT, CS, CP>, vk::Result>)
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
    CG: Store<
            Vec<command::CommandPool<families::Graphics>>,
            Vec<command::CommandPoolInfo<families::Graphics>>,
        >,
    CC: Store<
            Vec<command::CommandPool<families::Compute>>,
            Vec<command::CommandPoolInfo<families::Compute>>,
        >,
    CT: Store<
            Vec<command::CommandPool<families::Transfer>>,
            Vec<command::CommandPoolInfo<families::Transfer>>,
        >,
    CS: Store<
            Vec<command::CommandPool<families::Sparse>>,
            Vec<command::CommandPoolInfo<families::Sparse>>,
        >,
    CP: Store<
            Vec<command::CommandPool<families::Protected>>,
            Vec<command::CommandPoolInfo<families::Protected>>,
        >,
{
    // println!("{base:#?}"); // FIXME: Fix the printing problem with Base Debug.
    if base.is_err() {
        eprintln!("\x1b[38;5;1m [ ERR ]::[Failed to initialize vulkan]\x1b[0m");
        let _ = base.inspect_err(|err| {
            eprintln!("err = {err:?}");
        });
        panic!("Some error (quick note: -- --nocapture (for showing output of test))");
    } else {
        println!("\x1b[38;5;2m [ OK ]::[Initialization was succesfull]\x1b[0m");
    }
}

// TODO: make create_base_instance, create_base_device, ... Then make test foos which asserts
// values in created base

#[test]
fn create_empty() {
    init_logger();
    let base = vinit::BaseConfig::default().build();
    check(base);
}

#[test]
fn create_instance() {
    init_logger();
    let base = vinit::BaseConfig::default()
        .with(
            instance::Instance::default()
                .api_version(0, 3, 1)
                .validation(vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()])
                .app_name(CString::new("TEST").unwrap())
                .app_version(0, 0, 0),
        )
        .build();

    check(base);
}

#[test]
fn create_device() {
    init_logger();
    let base = vinit::BaseConfig::default()
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
        .build();

    check(base);
}

#[test]
fn create_pool() {
    init_logger();
    let base = vinit::BaseConfig::default()
        .with(
            instance::Instance::default()
                .validation(vec![CString::from(c"VK_LAYER_KHRONOS_validation")]),
        )
        .with(device::Device::default())
        .with(command::CommandPool::<families::Graphics>::graphics())
        .build();

    check(base);
}
