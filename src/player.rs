use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));

    commands.spawn((
        Player {},
        Mesh3d(mesh),
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
    ));
}

fn player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();
    let speed = 2.0;

    for mut transform in query.iter_mut() {
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
    }
}
