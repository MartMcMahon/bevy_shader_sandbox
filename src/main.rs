use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("071f3c").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(change_color)
        .add_system(move_around)
        .run();
}

#[derive(Component)]
struct MovesAround;

#[derive(Component)]
struct Cube;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cube_mat_handle = materials.add(CustomMaterial {
        time: 0.,
        alpha_mode: AlphaMode::Blend,
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
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.75 })),
            transform: Transform::from_xyz(i as f32, i as f32 - 0.2, i as f32),
            material: cube_mat_handle.clone(),
            ..default()
        });
    }
}

fn change_color(mut materials: ResMut<Assets<CustomMaterial>>, time: Res<Time>) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds() as f32;
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

// fn update_camera
