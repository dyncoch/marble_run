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
    // Materials
    let track_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.6, 0.4), // Wood-like brown
        perceptual_roughness: 0.8,
        ..default()
    });

    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.1, 0.1), // Red walls
        perceptual_roughness: 0.6,
        ..default()
    });

    // Track dimensions
    let track_depth = 20.0; // How far into the screen the track goes
    let track_width = 8.0; // Total width of the U-shape
    let track_thickness = 0.3;
    let wall_height = 0.8;
    let wall_thickness = 0.2;

    // Main track floor (the bottom of the U)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(track_width, track_thickness, track_depth)),
        material: track_material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Left wall (left side of the U)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(wall_thickness, wall_height, track_depth)),
        material: wall_material.clone(),
        transform: Transform::from_xyz(
            -track_width / 2.0 - wall_thickness / 2.0,
            wall_height / 2.0,
            0.0,
        ),
        ..default()
    });

    // Right wall (right side of the U)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(wall_thickness, wall_height, track_depth)),
        material: wall_material.clone(),
        transform: Transform::from_xyz(
            track_width / 2.0 + wall_thickness / 2.0,
            wall_height / 2.0,
            0.0,
        ),
        ..default()
    });

    // Add marble (sphere) - positioned at the back of the track
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(0.3)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2), // Green marble
            metallic: 0.3,
            perceptual_roughness: 0.1,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, -8.0), // Start at back of track, slightly above
        ..default()
    });

    // Add camera - positioned behind and above like racing games
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, -18.0)
            .looking_at(Vec3::new(0.0, 0.0, 5.0), Vec3::Y),
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
