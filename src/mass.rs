use ash::vk;

pub fn satisifes_features(
    have: &vk::PhysicalDeviceFeatures,
    required: &vk::PhysicalDeviceFeatures,
) -> bool {
    (required.robust_buffer_access == 0 || have.robust_buffer_access != 0)
        && (required.full_draw_index_uint32 == 0 || have.full_draw_index_uint32 != 0)
        && (required.image_cube_array == 0 || have.image_cube_array != 0)
        && (required.independent_blend == 0 || have.independent_blend != 0)
        && (required.geometry_shader == 0 || have.geometry_shader != 0)
        && (required.tessellation_shader == 0 || have.tessellation_shader != 0)
        && (required.sample_rate_shading == 0 || have.sample_rate_shading != 0)
        && (required.dual_src_blend == 0 || have.dual_src_blend != 0)
        && (required.logic_op == 0 || have.logic_op != 0)
        && (required.multi_draw_indirect == 0 || have.multi_draw_indirect != 0)
        && (required.draw_indirect_first_instance == 0 || have.draw_indirect_first_instance != 0)
        && (required.depth_clamp == 0 || have.depth_clamp != 0)
        && (required.depth_bias_clamp == 0 || have.depth_bias_clamp != 0)
        && (required.fill_mode_non_solid == 0 || have.fill_mode_non_solid != 0)
        && (required.depth_bounds == 0 || have.depth_bounds != 0)
        && (required.wide_lines == 0 || have.wide_lines != 0)
        && (required.large_points == 0 || have.large_points != 0)
        && (required.alpha_to_one == 0 || have.alpha_to_one != 0)
        && (required.multi_viewport == 0 || have.multi_viewport != 0)
        && (required.sampler_anisotropy == 0 || have.sampler_anisotropy != 0)
        && (required.texture_compression_etc2 == 0 || have.texture_compression_etc2 != 0)
        && (required.texture_compression_astc_ldr == 0 || have.texture_compression_astc_ldr != 0)
        && (required.texture_compression_bc == 0 || have.texture_compression_bc != 0)
        && (required.occlusion_query_precise == 0 || have.occlusion_query_precise != 0)
        && (required.pipeline_statistics_query == 0 || have.pipeline_statistics_query != 0)
        && (required.vertex_pipeline_stores_and_atomics == 0
            || have.vertex_pipeline_stores_and_atomics != 0)
        && (required.fragment_stores_and_atomics == 0 || have.fragment_stores_and_atomics != 0)
        && (required.shader_tessellation_and_geometry_point_size == 0
            || have.shader_tessellation_and_geometry_point_size != 0)
        && (required.shader_image_gather_extended == 0 || have.shader_image_gather_extended != 0)
        && (required.shader_storage_image_extended_formats == 0
            || have.shader_storage_image_extended_formats != 0)
        && (required.shader_storage_image_multisample == 0
            || have.shader_storage_image_multisample != 0)
        && (required.shader_storage_image_read_without_format == 0
            || have.shader_storage_image_read_without_format != 0)
        && (required.shader_storage_image_write_without_format == 0
            || have.shader_storage_image_write_without_format != 0)
        && (required.shader_uniform_buffer_array_dynamic_indexing == 0
            || have.shader_uniform_buffer_array_dynamic_indexing != 0)
        && (required.shader_sampled_image_array_dynamic_indexing == 0
            || have.shader_sampled_image_array_dynamic_indexing != 0)
        && (required.shader_storage_buffer_array_dynamic_indexing == 0
            || have.shader_storage_buffer_array_dynamic_indexing != 0)
        && (required.shader_storage_image_array_dynamic_indexing == 0
            || have.shader_storage_image_array_dynamic_indexing != 0)
        && (required.shader_clip_distance == 0 || have.shader_clip_distance != 0)
        && (required.shader_cull_distance == 0 || have.shader_cull_distance != 0)
        && (required.shader_float64 == 0 || have.shader_float64 != 0)
        && (required.shader_int64 == 0 || have.shader_int64 != 0)
        && (required.shader_int16 == 0 || have.shader_int16 != 0)
        && (required.shader_resource_residency == 0 || have.shader_resource_residency != 0)
        && (required.shader_resource_min_lod == 0 || have.shader_resource_min_lod != 0)
        && (required.sparse_binding == 0 || have.sparse_binding != 0)
        && (required.sparse_residency_buffer == 0 || have.sparse_residency_buffer != 0)
        && (required.sparse_residency_image2_d == 0 || have.sparse_residency_image2_d != 0)
        && (required.sparse_residency_image3_d == 0 || have.sparse_residency_image3_d != 0)
        && (required.sparse_residency2_samples == 0 || have.sparse_residency2_samples != 0)
        && (required.sparse_residency4_samples == 0 || have.sparse_residency4_samples != 0)
        && (required.sparse_residency8_samples == 0 || have.sparse_residency8_samples != 0)
        && (required.sparse_residency16_samples == 0 || have.sparse_residency16_samples != 0)
        && (required.sparse_residency_aliased == 0 || have.sparse_residency_aliased != 0)
        && (required.variable_multisample_rate == 0 || have.variable_multisample_rate != 0)
        && (required.inherited_queries == 0 || have.inherited_queries != 0)
}

pub fn satisfies_properties(
    have: &vk::PhysicalDeviceProperties,
    required: &vk::PhysicalDeviceProperties,
) -> bool {
    let limits_ok = have.limits.max_image_dimension1_d >= required.limits.max_image_dimension1_d
        && have.limits.max_image_dimension2_d >= required.limits.max_image_dimension2_d
        && have.limits.max_image_dimension3_d >= required.limits.max_image_dimension3_d
        && have.limits.max_image_dimension_cube >= required.limits.max_image_dimension_cube
        && have.limits.max_image_array_layers >= required.limits.max_image_array_layers
        && have.limits.max_texel_buffer_elements >= required.limits.max_texel_buffer_elements
        && have.limits.max_uniform_buffer_range >= required.limits.max_uniform_buffer_range
        && have.limits.max_storage_buffer_range >= required.limits.max_storage_buffer_range
        && have.limits.max_push_constants_size >= required.limits.max_push_constants_size
        && have.limits.max_memory_allocation_count >= required.limits.max_memory_allocation_count
        && have.limits.max_bound_descriptor_sets >= required.limits.max_bound_descriptor_sets
        && have.limits.max_per_stage_descriptor_samplers
            >= required.limits.max_per_stage_descriptor_samplers
        && have.limits.max_per_stage_descriptor_uniform_buffers
            >= required.limits.max_per_stage_descriptor_uniform_buffers
        && have.limits.max_per_stage_descriptor_storage_buffers
            >= required.limits.max_per_stage_descriptor_storage_buffers
        && have.limits.max_per_stage_descriptor_sampled_images
            >= required.limits.max_per_stage_descriptor_sampled_images
        && have.limits.max_per_stage_descriptor_storage_images
            >= required.limits.max_per_stage_descriptor_storage_images
        && have.limits.max_descriptor_set_samplers >= required.limits.max_descriptor_set_samplers
        && have.limits.max_descriptor_set_uniform_buffers
            >= required.limits.max_descriptor_set_uniform_buffers
        && have.limits.max_descriptor_set_storage_buffers
            >= required.limits.max_descriptor_set_storage_buffers
        && have.limits.max_descriptor_set_sampled_images
            >= required.limits.max_descriptor_set_sampled_images
        && have.limits.max_descriptor_set_storage_images
            >= required.limits.max_descriptor_set_storage_images
        && have.limits.max_vertex_input_attributes >= required.limits.max_vertex_input_attributes
        && have.limits.max_vertex_input_bindings >= required.limits.max_vertex_input_bindings
        && have.limits.max_viewports >= required.limits.max_viewports
        && have.limits.max_compute_work_group_count[0]
            >= required.limits.max_compute_work_group_count[0]
        && have.limits.max_compute_work_group_count[1]
            >= required.limits.max_compute_work_group_count[1]
        && have.limits.max_compute_work_group_count[2]
            >= required.limits.max_compute_work_group_count[2];

    // Check device type if specified (0 = OTHER means not specified)
    let type_ok = required.device_type == vk::PhysicalDeviceType::OTHER
        || have.device_type == required.device_type;

    limits_ok && type_ok
}
