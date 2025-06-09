use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Component to mark the marble
#[derive(Component)]
struct Marble;

// Component to mark the camera that should follow
#[derive(Component)]
struct FollowCamera;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default()) // Optional: shows physics wireframes
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (camera_follow_system, marble_control_system))
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
    let track_depth = 2000.0;
    let track_width = 8.0;
    let track_thickness = 0.3;
    let wall_height = 0.8;
    let wall_thickness = 0.2;
    let track_slope = 5.0_f32.to_radians(); // 5 degree slope down toward camera

    // Main track floor (the bottom of the U) - STATIC PHYSICS BODY with slope
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(track_width, track_thickness, track_depth)),
            material: track_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(track_slope)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(track_width / 2.0, track_thickness / 2.0, track_depth / 2.0),
        Friction::coefficient(0.7), // Some friction for realistic rolling
    ));

    // Left wall (left side of the U) - STATIC PHYSICS BODY with slope
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(wall_thickness, wall_height, track_depth)),
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                -track_width / 2.0 - wall_thickness / 2.0,
                wall_height / 2.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(track_slope)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0, track_depth / 2.0),
    ));

    // Right wall (right side of the U) - STATIC PHYSICS BODY with slope
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(wall_thickness, wall_height, track_depth)),
            material: wall_material.clone(),
            transform: Transform::from_xyz(
                track_width / 2.0 + wall_thickness / 2.0,
                wall_height / 2.0,
                0.0,
            )
            .with_rotation(Quat::from_rotation_x(track_slope)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0, track_depth / 2.0),
    ));

    // Add marble (sphere) - DYNAMIC PHYSICS BODY - positioned higher at back of sloped track
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.3)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.8, 0.2), // Green marble
                metallic: 0.3,
                perceptual_roughness: 0.1,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 2.0, -8.0), // Start higher due to slope
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(0.3),
        Restitution::coefficient(0.3),        // Some bounciness
        Friction::coefficient(0.4),           // Rolling friction
        ColliderMassProperties::Density(1.0), // Mass density
        Velocity {
            linvel: Vec3::new(0.0, 0.0, 2.0), // Small forward push (positive Z = toward camera)
            angvel: Vec3::ZERO,
        },
        Marble, // Mark this as the marble to follow
    ));

    // Add camera - positioned behind and above like racing games
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6.0, -18.0)
                .looking_at(Vec3::new(0.0, 0.0, 5.0), Vec3::Y),
            ..default()
        },
        FollowCamera, // Mark this as the following camera
    ));

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

    // Configure gravity - pointing down with standard Earth gravity
    commands.insert_resource(RapierConfiguration {
        gravity: Vec3::new(0.0, -9.81, 0.0),
        physics_pipeline_active: true,
        query_pipeline_active: true,
        timestep_mode: TimestepMode::Variable {
            max_dt: 1.0 / 60.0,
            time_scale: 1.0,
            substeps: 1,
        },
        scaled_shape_subdivision: 10,
        force_update_from_transform_changes: false,
    });
}

fn marble_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut marble_query: Query<&mut Velocity, With<Marble>>,
) {
    if let Ok(mut velocity) = marble_query.get_single_mut() {
        let control_force = 5.0; // Strength of left/right movement
        let mut lateral_force = 0.0;

        // Check for left/right input
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            lateral_force += control_force;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            lateral_force -= control_force;
        }

        // Apply lateral movement while preserving forward momentum
        velocity.linvel.x = lateral_force;
        // Keep the marble rolling forward - don't interfere with Z velocity
        // Keep Y velocity for physics (gravity, bouncing)
    }
}

fn camera_follow_system(
    mut camera_query: Query<&mut Transform, (With<FollowCamera>, Without<Marble>)>,
    marble_query: Query<&Transform, (With<Marble>, Without<FollowCamera>)>,
    time: Res<Time>,
) {
    if let (Ok(mut camera_transform), Ok(marble_transform)) =
        (camera_query.get_single_mut(), marble_query.get_single())
    {
        // Camera offset relative to marble (behind and above)
        let camera_offset = Vec3::new(0.0, 5.0, -9.0);

        // Target camera position - follows marble's X and Z, but keeps fixed Y offset
        let target_camera_pos = Vec3::new(
            marble_transform.translation.x, // Follow left/right movement
            marble_transform.translation.y + camera_offset.y, // Fixed height above marble
            marble_transform.translation.z + camera_offset.z, // Follow forward/back movement
        );

        // Smooth camera movement using interpolation
        let lerp_speed = 10.0;
        let current_pos = camera_transform.translation;
        let new_pos = current_pos.lerp(target_camera_pos, lerp_speed * time.delta_seconds());

        // Update camera position
        camera_transform.translation = new_pos;

        // Fixed forward-looking direction - no rotation, always look straight ahead
        let look_target = Vec3::new(
            camera_transform.translation.x, // Same X as camera (no left/right rotation)
            camera_transform.translation.y - 2.0, // Look slightly down
            camera_transform.translation.z + 10.0, // Look straight ahead
        );
        camera_transform.look_at(look_target, Vec3::Y);
    }
}
