use std::f32::consts::PI;

use bevy::prelude::*;

use bevy_mat_mtoon::{MtoonMainCamera, MtoonMaterial, MtoonPlugin, MtoonSun};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MtoonPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mtoon_materials: ResMut<Assets<MtoonMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 8.0, 12.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        },
        MtoonMainCamera,
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(2.0, 2.0, 2.0),
                ..default()
            },
            ..default()
        },
        MtoonSun,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::GRAY * 0.2,
        brightness: 0.10,
    });

    let mtoon = mtoon_materials.add(MtoonMaterial::default());

    let shapes = [
        meshes.add(shape::Cube::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Capsule::default().into()),
        meshes.add(shape::Torus::default().into()),
        meshes.add(shape::Cylinder::default().into()),
        meshes.add(shape::Icosphere::default().try_into().unwrap()),
        meshes.add(shape::UVSphere::default().into()),
    ];

    let num_shapes = shapes.len();
    const X_EXTENT: f32 = 14.5;

    for (i, mesh) in shapes.into_iter().enumerate() {
        commands.spawn(MaterialMeshBundle {
            mesh,
            material: mtoon.clone(),
            transform: Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(-PI / 4.0)),
            ..default()
        });
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Handle<MtoonMaterial>>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() / 2.0));
    }
}
