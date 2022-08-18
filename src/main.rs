use bevy::prelude::*;

mod game;

pub const WINDOW_WIDTH: f32 = 400.;
pub const WINDOW_HEIGHT: f32 = 400.;

fn setup_window(mut commands: Commands, mut windows: ResMut<Windows>) {

    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();

    window.set_position(IVec2 {
        x: 1920 - window.width() as i32,
        y: 0,
    });
 
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("My Rusty Game"),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(setup_window)
        .add_startup_system(game::setup_table)
        .add_plugins(DefaultPlugins)
        .run();
}
