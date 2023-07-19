use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_svg::prelude::*;

mod game_rules;

const SQUARE_SIZE: f32 = 64.0 + 64.0;
const SCREEN_WIDTH: f32 = SQUARE_SIZE * 8.0;
const SCREEN_HEIGHT: f32 = SCREEN_WIDTH;

fn board(mut commands: Commands) {
    // SLSO8, names according to color-name.com
    let _yankees_blue: Color = Color::rgb_u8(13, 43, 69);
    let _japanese_indigo: Color = Color::rgb_u8(32, 60, 86);
    let _independence: Color = Color::rgb_u8(84, 78, 104);
    let antique_fuchsia: Color = Color::rgb_u8(141, 105, 122);
    let _raw_sienna: Color = Color::rgb_u8(208, 129, 89);
    let _rajah: Color = Color::rgb_u8(255, 170, 94);
    let _deep_champagne: Color = Color::rgb_u8(255, 212, 163);
    let papaya_whip: Color = Color::rgb_u8(255, 236, 214);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0, 999.9),
        ..default()
    });
    for i in 0..8 {
        for j in 0..8 {
            let color = if (i + j) % 2 == 0 {
                antique_fuchsia // Black
            } else {
                papaya_whip // White
            };
            let shape = shapes::Rectangle {
                extents: Vec2::new(SQUARE_SIZE, SQUARE_SIZE),
                origin: RectangleOrigin::BottomLeft,
            };

            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    transform: Transform::from_xyz(
                        SQUARE_SIZE * j as f32,
                        SQUARE_SIZE * i as f32,
                        0.0,
                    ),
                    ..default()
                },
                Fill::color(color),
            ));
        }
    }
}

fn figure(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("pieces/w_queen.svg");
    commands.spawn(Svg2dBundle {
        svg,
        origin: Origin::BottomLeft, // Origin::TopLeft is the default
        transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(3.0)),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_plugin(SvgPlugin)
        .add_startup_system(board)
        .add_startup_system(figure)
        .run();
}
