use bevy::{prelude::*, sprite::Anchor};

const PLAYER_SPRITE: &str = "player.png";

const WINDOW_WIDTH: f32 = 400.;
const WINDOW_HEIGHT: f32 = 300.;

fn setup_scene(mut commands: Commands, mut windows: ResMut<Windows>) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (wind_w, win_h) = (window.width(), window.height());

    window.set_position(IVec2 {
        x: 1920 - wind_w as i32,
        y: 0,
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(20., 20.)),
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: String::from("My Rusty Game"),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .add_startup_system(setup_scene)
        .add_plugins(DefaultPlugins)
        .run();
}
