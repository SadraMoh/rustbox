use bevy::prelude::*;

const PADDING: f32 = 100.;

const BACKGROUND_TEXTURE: &str = "img/bg_blue.png";
const TABLE_TEXTURE: &str = "img/table.png";
const X_TEXTURE: &str = "img/o.png";
const O_TEXTURE: &str = "img/x.png";

pub fn setup_table(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary_mut().unwrap();

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(BACKGROUND_TEXTURE),
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..Default::default()
        },
        ..Default::default()
    });

    // table sprite
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(TABLE_TEXTURE),
        sprite: Sprite {
            custom_size: Some(Vec2::new(
                window.width() - PADDING,
                window.height() - PADDING,
            )),
            ..Default::default()
        },
        ..Default::default()
    });
}
