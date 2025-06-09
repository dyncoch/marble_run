use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Marble Run".into(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add marble (sphere)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(0.5)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2), // Green marble
            metallic: 0.1,
            perceptual_roughness: 0.2,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 5.0, 0.0), // Start marble 5 units high
        ..default()
    });

    // Add camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 8.0, 12.0)
            .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ..default()
    });

    // Add directional light (like sun)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -0.5, // Angle down
            0.3,  // Slight side angle
            0.0,
        )),
        ..default()
    });

    // Add ambient light for general illumination
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });
}
