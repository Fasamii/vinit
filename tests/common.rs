use ash::vk;
use std::ffi::CString;
use vinit::*;

fn check<I, D>(base: Result<Base<I, D>, vk::Result>)
where
    I: Store<instance::Instance, instance::InstanceInfo>,
    D: Store<device::Device, device::DeviceInfo>,
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
    let base = vinit::BaseConfig::default().build();
    check(base);
}

#[test]
fn create_instance() {
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
