use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};

use crate::combat::Team;

#[derive(AsBindGroup, TypePath, Clone, Asset)]
pub struct ShipMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub last_damaged_time: f32,
    #[texture(1)]
    #[sampler(2)]
    pub base_texture: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub color_mask: Handle<Image>,
}

impl Material2d for ShipMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ship.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn set_ship_shader_team_color(
    query: Query<(&MeshMaterial2d<ShipMaterial>, &Team), Changed<Team>>,
    mut materials: ResMut<Assets<ShipMaterial>>
) {
    for (handle, team) in query.iter() {
        let color = match team.0 {
            1 => LinearRgba::new(0.8, 0.2, 0.2, 1.0),
            2 => LinearRgba::new(0.2, 0.2, 0.8, 1.0),
            _ => LinearRgba::new(0.2, 0.2, 0.2, 1.0),
        };
        match materials.get_mut(&handle.0) {
            None => {},
            Some(material) => { material.color = color; }
        };
    }
}
