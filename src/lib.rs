use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
};

pub const MTOON_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 11079857277321825555);

#[derive(Default)]
pub struct MtoonPlugin;

impl Plugin for MtoonPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, MTOON_SHADER_HANDLE, "mtoon.wgsl", Shader::from_wgsl);
        app.add_plugins(MaterialPlugin::<MtoonMaterial>::default())
            .add_systems(Update, update_mtoon_shader);
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone, Default)]
#[uuid = "c114d3b1-2274-4733-99df-f1fb5fd80bd1"]
#[uniform(0, MtoonMaterialUniform)]
pub struct MtoonMaterial {
    pub color: Color,
    pub sun_dir: Vec3,
    pub sun_color: Color,
    pub camera_pos: Vec3,
    pub ambient_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
}

impl Material for MtoonMaterial {
    fn fragment_shader() -> ShaderRef {
        MTOON_SHADER_HANDLE.typed().into()
    }
}

impl AsBindGroupShaderType<MtoonMaterialUniform> for MtoonMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<Image>,
    ) -> MtoonMaterialUniform {
        MtoonMaterialUniform {
            color: self.color.into(),
            sun_dir: self.sun_dir,
            sun_color: self.sun_color.into(),
            camera_pos: self.camera_pos,
            ambient_color: self.ambient_color.into(),
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct MtoonMaterialUniform {
    pub color: Vec4,
    pub sun_dir: Vec3,
    pub sun_color: Vec4,
    pub camera_pos: Vec3,
    pub ambient_color: Vec4,
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
            mtoon.camera_pos = cam_t.translation;
        }

        if let Ok((sun_t, dir_light)) = sun.get_single() {
            mtoon.sun_dir = sun_t.back();
            mtoon.sun_color = dir_light.color;
        }

        if let Some(light) = &ambient_light {
            mtoon.ambient_color = light.color;
        }
    }
}
