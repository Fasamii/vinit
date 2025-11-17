use ash::vk;
use std::ffi::CString;

fn main() {
    let _base = vinit::BaseConfig::default()
        .with_app_name(CString::from(c"Super Cool App"))
        .with_app_version((0, 0, 1))
        .with_instance_extensions([].into())
        // TODO: Consider creating device_config instead which will take all the possible
        // parameters not only ones suitable for selecting physical_device if that isn't to much of
        // an overhead
        .with_device(|physical_device_selector| {
            physical_device_selector
                .require_discrete(false)
                .prefer_best(true)
        })
        .with_device_extensions([CString::from(vk::KHR_SWAPCHAIN_NAME)].into())
        // You should have some structure that holds CommandBuffers and CommandBufferHandle that
        // holds only handle to that Pool then actual pools stored in Base should be guaranteed to
        // don't use any features that would be unsafe in multithreaded applications where pools
        // are hold by different threads, that call should define which queues are required and
        // also fill up that Handle structure (srr for bad English if you're reading this for some
        // reason)
        .with_command_pool(vinit::QueueFamilyType::Graphics(()))
        .with_command_pool(vinit::QueueFamilyType::Graphics(()))
        .with_command_pool(vinit::QueueFamilyType::Compute(()))
        // .with_swapchain(|swapchain_config| {
        //     swapchain_config
        //         .min_img_count(12)
        //         .img_format(vk::Format::R8G8B8A8_SRGB)
        // })
        .build();
}
