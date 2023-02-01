use std::f32::consts::PI;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    transform,
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin, PlayerPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("071f3c").unwrap()))
        .add_plugins(DefaultPlugins)
        // .add_plugin(PlayerPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            ..Default::default()
        })
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MaterialPlugin::<CylinderMaterial>::default())
        .add_startup_system(setup)
        .add_startup_system(cyl)
        .add_system(change_color)
        .add_system(move_around)
        .run();
}

#[derive(Component)]
struct MovesAround;

#[derive(Component)]
struct Cube;

fn cyl(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CylinderMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cyl_material = materials.add(CylinderMaterial {
        time: 0.,
        alpha_mode: AlphaMode::Blend,
    });
    let cyl = MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
        transform: Transform::from_xyz(-0.01, 0.0, 0.0),
        material: cyl_material.clone(),
        ..default()
    };
    commands.spawn(cyl);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    // mut cyl_materials: ResMut<Assets<CylinderMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cube_mat_handle = materials.add(CustomMaterial {
        time: 0.,
        alpha_mode: AlphaMode::Blend,
    });

    // camera
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    commands.spawn(camera).insert(FlyCam);

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Torus {
            radius: 1.0,
            ring_radius: 1.0,
            ..default()
        })),
        transform: Transform::from_xyz(-2.0, 5.0, 5.0),
        material: cube_mat_handle.clone(),
        ..default()
    });

    // plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 1.0, y: 1.0 }))),
        transform: Transform {
            translation: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            // rotation: Quat::from_rotation_x(-PI / 2.0),
            ..default()
        },
        material: cube_mat_handle.clone(),
        ..default()
    });

    // cube
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: cube_mat_handle.clone(),
            ..default()
        })
        .insert(MovesAround);

    for i in -2..2 {
        if i == 0 {
            continue;
        }
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.75 })),
            transform: Transform::from_xyz(i as f32, i as f32 - 0.2, i as f32),
            material: cube_mat_handle.clone(),
            ..default()
        });
    }
}

fn change_color(
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut cyl_mats: ResMut<Assets<CylinderMaterial>>,
    time: Res<Time>,
) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds() as f32;
    }
    for mat in cyl_mats.iter_mut() {
        mat.1.time = time.elapsed_seconds() as f32;
    }
}

fn move_around(mut query: Query<(&MovesAround, &mut Transform)>, time: Res<Time>) {
    for (thing, mut transform) in query.iter_mut() {
        let s = time.elapsed_seconds();
        transform.translation.x = s.sin();
        transform.translation.y = s.cos();
        transform.translation.z = s.cos();
    }
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    // fn vertex_shader() -> ShaderRef {
    //     "shaders/custom_material.wgsl".into()
    // }

    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

impl Material for CylinderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cyl_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    alpha_mode: AlphaMode,
}

#[derive(AsBindGroup, Debug, Clone)]
pub struct Camera {
    #[uniform(1)]
    pos: Vec3,
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "fcb427ef-8a5e-415b-bb9f-fe1a49bc712e"]
pub struct CylinderMaterial {
    #[uniform(0)]
    time: f32,
    alpha_mode: AlphaMode,
}
