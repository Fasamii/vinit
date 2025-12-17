use ash::vk;
use std::ffi::CString;
use std::sync::Once;
use vinit::{
    base::{Base, BaseConfig},
    *,
};

static INIT: Once = Once::new();

pub fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init()
            .ok();
    });
}

fn validation_layers() -> Vec<CString> {
    vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]
}

#[test]
fn test_empty_base() {
    crate::init_logger();
    let _base = BaseConfig::default()
        .build()
        .expect("Failed to create empty Base");

    log::info!("Empty base created successfully");
}

#[test]
fn test_instance_only() {
    init_logger();
    let base = BaseConfig::default()
        .with(
            instance::Instance::default()
                .api_version(1, 0, 0)
                .app_name(CString::new("TestApp").unwrap())
                .app_version(1, 0, 0),
        )
        .build()
        .expect("Failed to create instance");

    let inst = base.instance();
    log::info!("Instance created: {:?}", inst.0.handle());
}

#[test]
fn test_instance_with_validation() {
    init_logger();
    let _base = BaseConfig::default()
        .with(
            instance::Instance::default()
                .api_version(1, 3, 0)
                .validation(validation_layers()),
        )
        .build()
        .expect("Failed to create instance with validation");

    log::info!("Instance with validation created successfully");
}

#[test]
fn test_device_creation() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .build()
        .expect("Failed to create device");

    let device = base.device();
    log::info!("Device created: {:?}", device.device.handle());
}

#[test]
fn test_single_graphics_pool() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .build()
        .expect("Failed to create graphics pool");

    let pools = base.graphics_pools();
    assert_eq!(pools.len(), 1, "Should have exactly 1 graphics pool");
    log::info!("Graphics pool: {:?}", pools[0].pool);
}

#[test]
fn test_multiple_graphics_pools() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics().flags(vk::CommandPoolCreateFlags::TRANSIENT))
        .with(
            command::CommandPool::graphics()
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
        )
        .build()
        .expect("Failed to create multiple graphics pools");

    let pools = base.graphics_pools();
    assert_eq!(pools.len(), 3, "Should have exactly 3 graphics pools");
    log::info!("Created {} graphics pools", pools.len());
}

#[test]
fn test_compute_pool() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::compute())
        .build()
        .expect("Failed to create compute pool");

    let pools = base.compute_pools();
    assert_eq!(pools.len(), 1, "Should have exactly 1 compute pool");
    log::info!("Compute pool created successfully");
}

#[test]
fn test_transfer_pool() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::transfer())
        .build()
        .expect("Failed to create transfer pool");

    let pools = base.transfer_pools();
    assert_eq!(pools.len(), 1, "Should have exactly 1 transfer pool");
}

#[test]
fn test_mixed_pools() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::compute())
        .with(command::CommandPool::transfer())
        .build()
        .expect("Failed to create mixed pools");

    assert_eq!(base.graphics_pools().len(), 1);
    assert_eq!(base.compute_pools().len(), 1);
    assert_eq!(base.transfer_pools().len(), 1);
    log::info!("Mixed pools created successfully");
}

#[test]
fn test_device_prefer_best() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default().prefer_best(true))
        .build()
        .expect("Failed to create device");

    let device = base.device();
    log::info!("Selected device: {:?}", device.physical);
}

#[test]
fn test_device_prefer_worst() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default().prefer_best(false))
        .build()
        .expect("Failed to create device");

    let device = base.device();
    log::info!("Selected worst device: {:?}", device.physical);
}

#[test]
fn test_device_discrete_gpu() {
    init_logger();
    let result = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default().require_discrete(true))
        .build();

    match result {
        Ok(base) => {
            let device = base.device();
            assert_eq!(
                device.physical.properties.device_type,
                vk::PhysicalDeviceType::DISCRETE_GPU,
                "Should be discrete GPU"
            );
            log::info!("Discrete GPU found and selected");
        }
        Err(vk::Result::ERROR_FEATURE_NOT_PRESENT) => {
            log::info!("No discrete GPU available - test passed (expected on integrated systems)");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_invalid_api_version() {
    init_logger();
    let result = BaseConfig::default()
        .with(
            instance::Instance::default().api_version(0, 0, 0), // Invalid
        )
        .build();

    match result {
        Ok(_) => log::warn!("Invalid API version was accepted (might be driver-dependent)"),
        Err(e) => {
            log::info!("Expected error for invalid API version: {:?}", e);
            assert!(
                e == vk::Result::ERROR_INCOMPATIBLE_DRIVER
                    || e == vk::Result::ERROR_INITIALIZATION_FAILED
            );
        }
    }
}

#[test]
fn test_no_compatible_device() {
    init_logger();
    let result = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default().require_properties(
            vk::PhysicalDeviceProperties::default().limits(
                vk::PhysicalDeviceLimits::default().max_image_dimension2_d(u32::MAX), // Impossible requirement
            ),
        ))
        .build();

    match result {
        Ok(_) => panic!("Should not find device with impossible requirements"),
        Err(vk::Result::ERROR_FEATURE_NOT_PRESENT) => {
            log::info!("Correctly rejected impossible requirements");
        }
        Err(e) => log::warn!("Got error {:?}, expected ERROR_FEATURE_NOT_PRESENT", e),
    }
}

#[test]
fn test_builder_chaining() {
    init_logger();
    let base = BaseConfig::default()
        .with(
            instance::Instance::default()
                .api_version(1, 3, 0)
                .app_name(CString::new("ChainTest").unwrap())
                .app_version(2, 1, 5)
                .engine_name(CString::new("MyEngine").unwrap())
                .engine_version(3, 0, 0),
        )
        .with(device::Device::default().prefer_best(true))
        .with(command::CommandPool::graphics())
        .with(
            command::CommandPool::compute().flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
        )
        .build()
        .expect("Failed to build with chaining");

    assert_eq!(base.graphics_pools().len(), 1);
    assert_eq!(base.compute_pools().len(), 1);
    log::info!("Builder chaining ok");
}

#[test]
fn test_builder_reuse() {
    init_logger();

    let config = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default());

    let base1 = config
        .with(command::CommandPool::graphics())
        .build()
        .expect("Failed to build base1");

    let config2 = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default());

    let base2 = config2
        .with(command::CommandPool::compute())
        .build()
        .expect("Failed to build base2");

    assert_eq!(base1.graphics_pools().len(), 1);
    assert_eq!(base2.compute_pools().len(), 1);
}

#[test]
fn test_base_drop_order() {
    init_logger();
    {
        let base = BaseConfig::default()
            .with(instance::Instance::default())
            .with(device::Device::default())
            .with(command::CommandPool::graphics())
            .build()
            .expect("Failed to create base");

        let _device = base.device();
        let _instance = base.instance();
        let _pools = base.graphics_pools();

        log::info!("All components accessed successfully");
    }

    log::info!("Base dropped successfully");
}

#[test]
fn test_nested_scopes() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .build()
        .expect("Failed to create base");

    {
        let _instance_ref = base.instance();
        {
            let _device_ref = base.device();
            {
                let _pools_ref = base.graphics_pools();
                log::info!("All nested scopes valid");
            }
        }
    }

    let _final_check = base.instance();
    log::info!("Nested scopes test passed");
}

#[test]
fn test_move_semantics() {
    init_logger();

    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .build()
        .expect("Failed to create base");

    fn take_ownership(_base: Base<Present, Present, Absent, Present, Absent, Absent, Absent, Absent>) {
        log::info!("Took ownership of base");
    }

    take_ownership(base);

    log::info!("Move semantics test passed");
}

#[test]
fn test_queue_families_populated() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .build()
        .expect("Failed to create base");

    let device = base.device();
    assert!(
        device.queue_handles.graphics.is_some(),
        "Graphics queue should exist"
    );

    if let Some(queue) = device.queue_handles.graphics {
        log::info!("Graphics queue handle: {:?}", queue);
    }
}

#[test]
fn test_multiple_queue_families() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::compute())
        .build()
        .expect("Failed to create base");

    let device = base.device();
    assert!(
        device.queue_handles.graphics.is_some(),
        "Graphics queue should exist"
    );
    assert!(
        device.queue_handles.compute.is_some(),
        "Compute queue should exist"
    );

    log::info!("Multiple queue families verified");
}

#[test]
fn test_queue_family_indices() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .build()
        .expect("Failed to create base");

    let device = base.device();
    let indices = &device.physical.queue_families_indices;

    assert!(
        indices.graphics.is_some(),
        "Should have graphics queue family index"
    );
    log::info!("Graphics queue family index: {:?}", indices.graphics);
}

#[test]
fn test_device_properties() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .build()
        .expect("Failed to create base");

    let device = base.device();
    let props = &device.physical.properties;

    log::info!("Device type: {:?}", props.device_type);
    log::info!("Device name: {:?}", unsafe {
        std::ffi::CStr::from_ptr(props.device_name.as_ptr())
    });
    log::info!(
        "API version: {}.{}.{}",
        vk::api_version_major(props.api_version),
        vk::api_version_minor(props.api_version),
        vk::api_version_patch(props.api_version)
    );
}

#[test]
fn test_device_limits() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .build()
        .expect("Failed to create base");

    let device = base.device();
    let limits = &device.physical.limits;

    log::info!("Max image dimension 2D: {}", limits.max_image_dimension2_d);
    log::info!(
        "Max compute work group invocations: {}",
        limits.max_compute_work_group_invocations
    );
    log::info!("Max framebuffer width: {}", limits.max_framebuffer_width);
}

#[test]
fn test_device_memory_properties() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .build()
        .expect("Failed to create base");

    let device = base.device();
    let mem_props = &device.physical.memory_properties;

    let heap_count = mem_props.memory_heap_count;
    log::info!("Memory heap count: {}", heap_count);

    for i in 0..heap_count {
        let heap = mem_props.memory_heaps[i as usize];
        log::info!(
            "Heap {}: size = {} MB, flags = {:?}",
            i,
            heap.size / (1024 * 1024),
            heap.flags
        );
    }
}

#[test]
fn test_command_pool_flags() {
    init_logger();
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics().flags(vk::CommandPoolCreateFlags::TRANSIENT))
        .with(
            command::CommandPool::graphics()
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
        )
        .build()
        .expect("Failed to create pools with flags");

    let pools = base.graphics_pools();
    assert_eq!(
        pools.len(),
        2,
        "Should have 2 graphics pools with different flags"
    );
}

#[test]
fn test_many_command_pools() {
    init_logger();
    let config = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::graphics());

    let base = config.build().expect("Failed to create many pools");
    assert_eq!(
        base.graphics_pools().len(),
        10,
        "Should have 10 graphics pools"
    );
    log::info!("Successfully created 10 command pools");
}

#[test]
fn test_create_drop_multiple() {
    init_logger();

    for i in 0..5 {
        let _base = BaseConfig::default()
            .with(instance::Instance::default())
            .with(device::Device::default())
            .with(command::CommandPool::graphics())
            .build()
            .expect("Failed to create base in loop");

        log::info!("Created and about to drop base {}", i);
    }

    log::info!("All 5 iterations completed successfully");
}

#[test]
fn test_all_pool_types() {
    init_logger();

    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(command::CommandPool::graphics())
        .with(command::CommandPool::compute())
        .with(command::CommandPool::transfer())
        .build()
        .expect("Failed to create all pool types");

    assert_eq!(base.graphics_pools().len(), 1);
    assert_eq!(base.compute_pools().len(), 1);
    assert_eq!(base.transfer_pools().len(), 1);

    log::info!("All pool types created successfully");
}

#[test]
fn test_api_version_1_0() {
    init_logger();
    let _base = BaseConfig::default()
        .with(instance::Instance::default().api_version(1, 0, 0))
        .build()
        .expect("Failed with Vulkan 1.0");

    log::info!("Vulkan 1.0 instance created successfully");
}

#[test]
fn test_api_version_1_1() {
    init_logger();
    let result = BaseConfig::default()
        .with(instance::Instance::default().api_version(1, 1, 0))
        .build();

    match result {
        Ok(_) => log::info!("Vulkan 1.1 supported"),
        Err(e) => log::warn!("Vulkan 1.1 not supported: {:?}", e),
    }
}

#[test]
fn test_api_version_1_2() {
    init_logger();
    let result = BaseConfig::default()
        .with(instance::Instance::default().api_version(1, 2, 0))
        .build();

    match result {
        Ok(_) => log::info!("Vulkan 1.2 supported"),
        Err(e) => log::warn!("Vulkan 1.2 not supported: {:?}", e),
    }
}

#[test]
fn test_api_version_1_3() {
    init_logger();
    let result = BaseConfig::default()
        .with(instance::Instance::default().api_version(1, 3, 0))
        .build();

    match result {
        Ok(_) => log::info!("Vulkan 1.3 supported"),
        Err(e) => log::warn!("Vulkan 1.3 not supported: {:?}", e),
    }
}

#[test]
fn test_create_with_swapchain() {
    let base = BaseConfig::default()
        .with(instance::Instance::default())
        .with(device::Device::default())
        .with(swapchain::Swapchain::default())
        .build()
        .expect("Failed to build base");

    let _swapchain = base.swapchain();
    log::info!("Created swapchain");
}
