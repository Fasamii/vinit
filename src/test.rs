fn main() {
    let config = vinit::RendererConfig::default()
        .with_app_info(c"TEST", 0, 0, 0)
        .with_device(|selector| {
            selector
                .require_graphics_queue()
                .require_compute_queue()
                .require_transfer_queue()
        });
}

