struct MtoonMaterial {
    base_color: vec4<f32>,
    shade_color: vec4<f32>,
    ambient_color: vec4<f32>,
    light_color: vec4<f32>,
    light_dir: vec3<f32>,
    shading_shift_factor: f32,
    shading_toony_factor: f32,
    gl_equalization_factor: f32,
};

@group(1) @binding(0)
var<uniform> material: MtoonMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var shade_color_texture: texture_2d<f32>;
@group(1) @binding(4)
var shade_color_sampler: sampler;
@group(1) @binding(5)
var normal_texture: texture_2d<f32>;
@group(1) @binding(6)
var normal_sampler: sampler;

#import bevy_pbr::mesh_vertex_output MeshVertexOutput

@fragment
fn fragment (in: MeshVertexOutput) -> @location(0) vec4<f32> {
    // Base lighting
    let base_color = material.base_color * textureSample(base_color_texture, base_color_sampler, in.uv);
    let shade_color = material.shade_color * textureSample(shade_color_texture, shade_color_sampler, in.uv);

    let normal = normalize(in.world_normal);
    let n_dot_l = dot(normal, material.light_dir);

    let base_shading = n_dot_l + material.shading_shift_factor;
    let shading = 1.0 - linear_step(material.shading_toony_factor - 1.0, 1.0 - material.shading_toony_factor, base_shading);

    var color = mix(base_color, shade_color, shading) * material.light_color;

    // Global illumination
    // !! This is not at all correct, idk how to get global illumination for a point in the scene,
    //    so I just use the ambient light and apply it to the shading
    let uniform_gi = material.ambient_color.a / 2.0;
    let passthrough_gi = n_dot_l * material.ambient_color.a;
    let gi = shading * mix(passthrough_gi, uniform_gi, material.gl_equalization_factor);
    color = color + gi * material.ambient_color;
    
    return color;
}

fn linear_step(a: f32, b: f32, t: f32) -> f32 {
    return saturate((t - a) / (b - a));
}
