#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::utils
#import noisy_bevy::prelude
#import bevy_sprite::mesh2d_bindings

struct V4 {
    v: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> main_background: V4;

@group(1) @binding(1)
var<uniform> highlight_color: V4;

@group(1) @binding(2)
var<uniform> dark_color: V4;

@group(1) @binding(3)
var<uniform> star_color: V4;

@group(1) @binding(4)
var<uniform> map_boundary: V4;


@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let p = world_position.xy;

    let layer_0 = simplex_noise_2d(p * 0.001) * 0.5;

    let layer_1_p = p * 0.001 + p * 0.07;

    var layer_1 : f32 = clamp(max(0., floor(simplex_noise_2d(layer_1_p) * 5. % 5.) - 3.) * 0.2, 0., 1.);

    let layer_2_p = p * 0.006 + p * 0.04;

    var layer_2: f32 = simplex_noise_2d(layer_2_p);

    layer_2 = layer_2 + simplex_noise_2d(layer_2_p * 3. + 5.) * 0.3;
    layer_2 = layer_2 + simplex_noise_2d(layer_2_p * 8. + 2.) * 0.2;
    layer_2 = layer_2 + simplex_noise_2d(layer_2_p * 12. + 7.) * 0.4;
    layer_2 = clamp(layer_2, 0., 1.);

    let backdrop_noise = mix(main_background.v, dark_color.v, layer_0);

    let next_noise = mix(backdrop_noise, star_color.v, layer_1);

    let next_noise_2 = mix(next_noise, highlight_color.v, next_noise);

    var border: f32 = 0.;
    let p = p + simplex_noise_2d(p *0.5 + 16.) * 10.;
    if any(p < map_boundary.v.xy) || any(p > map_boundary.v.zw) {
        border = 1.;
    }

    let result = mix(next_noise_2, dark_color.v, border * 0.5);

    return result;
}