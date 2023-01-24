use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "6d535a38-2b0f-4d43-9bc2-2f000a2c9b33"]
pub struct SpaceMaterial {
    #[uniform(0)]
    pub(crate) main_background: Color,
    #[uniform(1)]
    pub(crate) highlight_color: Color,
    #[uniform(2)]
    pub(crate) dark_color: Color,
    #[uniform(3)]
    pub(crate) star_color: Color,
    #[uniform(4)]
    pub(crate) map_boundary: Vec4,
}

impl Default for SpaceMaterial {
    fn default() -> Self {
        Self {
            main_background: Color::rgb_u8(67, 13, 75),
            highlight_color: Color::rgb_u8(204, 111, 218),
            dark_color: Color::rgb_u8(23, 13, 25),
            star_color: Color::rgb_u8(246, 225, 249),
            map_boundary: Vec4::new(-900., -500., 900., 500.),
        }
    }
}

impl Material2d for SpaceMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "space.wgsl".into()
    }
}
