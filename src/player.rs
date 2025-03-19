use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Target;

// #[derive(Resource)]
// struct CursorWorldPosition {
//     pub position: Vec3,
// }

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
        // .insert_resource(CursorWorldPosition {
        //     position: Vec3::ZERO,
        // });
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    let model: Handle<Scene> = asset_server.load("Unicorn.glb#Scene0");

    commands.spawn((
        Target,
        Mesh3d(mesh),
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
    ));

    commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Player,
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneRoot(model),
                // Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI))
                //     .with_scale(Vec3::new(0.1, 0.1, 0.1)),
                Transform::from_scale(Vec3::splat(0.1)),
            ));
        });
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    mut target_query: Query<&mut Transform, (With<Target>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // cursor_res: Res<CursorWorldPosition>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let delta_time = time.delta_secs();
    let speed = 8.0;
    let mut target_transform = target_query.single_mut();

    let (camera, camera_transform) = cameras.single();
    let window = windows.single();
    let mut transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation.z -= speed * delta_time;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation.z += speed * delta_time;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed * delta_time;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation.x += speed * delta_time;
    }

    if let Some(cursor_position) = window.cursor_position() {
        if let Some(ray) = camera
            .viewport_to_world(camera_transform, cursor_position)
            .ok()
        {
            let t = -ray.origin.y / ray.direction.y;
            let target = ray.origin + t * ray.direction;
            // transform.look_at(look_at, Vec3::Y);

            let player_position = transform.translation;
            let mut direction = target - player_position;
            direction.y = 0.0;

            let new_rotation = Quat::from_rotation_arc(Vec3::Z, direction.normalize());

            transform.rotation = new_rotation;
            target_transform.translation = target;
        }
    }
}
