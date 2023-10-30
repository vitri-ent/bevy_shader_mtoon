use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
};

pub const MTOON_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 11079857777321825555);

#[derive(Default)]
pub struct MtoonPlugin;

impl Plugin for MtoonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, MTOON_SHADER_HANDLE, "mtoon.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<MtoonMaterial>::default())
            .add_systems(Update, update_mtoon_shader);
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "c114d3b1-2274-4733-99df-f1fb5fd80bd1"]
#[uniform(0, MtoonMaterialUniform)]
pub struct MtoonMaterial {
    pub base_color: Color,
    pub shade_color: Color,
    pub ambient_color: Color,
    pub light_color: Color,
    pub light_dir: Vec3,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub gl_equalization_factor: f32,

    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub shade_color_texture: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    pub normal_texture: Option<Handle<Image>>,
}

impl Default for MtoonMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            shade_color: Color::BLACK,
            ambient_color: Color::WHITE,
            light_color: Color::WHITE,
            light_dir: Vec3::Y,
            shading_shift_factor: 0.0,
            shading_toony_factor: 0.9,
            gl_equalization_factor: 0.9,
            base_color_texture: None,
            shade_color_texture: None,
            normal_texture: None,
        }
    }
}

impl Material for MtoonMaterial {
    fn fragment_shader() -> ShaderRef {
        MTOON_SHADER_HANDLE.typed().into()
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct MtoonMaterialUniform {
    pub base_color: Vec4,
    pub shade_color: Vec4,
    pub ambient_color: Vec4,
    pub light_color: Vec4,
    pub light_dir: Vec3,
    pub shading_shift_factor: f32,
    pub shading_toony_factor: f32,
    pub gl_equalization_factor: f32,
}

impl AsBindGroupShaderType<MtoonMaterialUniform> for MtoonMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<Image>,
    ) -> MtoonMaterialUniform {
        MtoonMaterialUniform {
            base_color: self.base_color.into(),
            shade_color: self.shade_color.into(),
            ambient_color: self.ambient_color.into(),
            light_color: self.light_color.into(),
            light_dir: self.light_dir,
            shading_shift_factor: self.shading_shift_factor,
            shading_toony_factor: self.shading_toony_factor,
            gl_equalization_factor: self.gl_equalization_factor,
        }
    }
}

#[derive(Component)]
pub struct MtoonMainCamera;

#[derive(Component)]
pub struct MtoonSun;

pub fn update_mtoon_shader(
    main_cam: Query<&Transform, With<MtoonMainCamera>>,
    sun: Query<(&Transform, &DirectionalLight), With<MtoonSun>>,
    ambient_light: Option<Res<AmbientLight>>,
    mut materials: ResMut<Assets<MtoonMaterial>>,
) {
    for (_, mtoon) in materials.iter_mut() {
        if let Ok(cam_t) = main_cam.get_single() {
            // mtoon.camera_pos = cam_t.translation;
        }

        if let Ok((transform, light)) = sun.get_single() {
            mtoon.light_dir = transform.back();
            mtoon.light_color = light.color;
        }

        if let Some(light) = &ambient_light {
            let mut ambient_color = light.color;
            ambient_color.set_a(ambient_color.a() * light.brightness);
            mtoon.ambient_color = ambient_color;
        }
    }
}
