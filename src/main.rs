// include the prelude module to get all the basic bevy modules
use bevy::{prelude::*, math::*};

// bevy is about ECS (Entity Component System)

const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

fn main() {
    // println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins) // include basic plugins like logger, window, input, etc.
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9))) // set background color
        .add_systems(Update, bevy::window::close_on_esc) // exit on ESC
        .add_systems(Startup, setup) // call only once when the game starts
        .run()
}

#[derive(Component)] // kinda like inheritence for struct
struct Paddle;


fn setup(mut commands: Commands) {
    //camera
    commands.spawn(Camera2dBundle::default()); // bundle is just a collection of components

    // paddle
    commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: vec3(0., PADDLE_START_Y, 0.),
                    ..default()
                },
                sprite: Sprite {
                    color: PADDLE_COLOR,
                    custom_size: Some(PADDLE_SIZE),
                    ..default()
                },
                ..default()
            },
            Paddle,
    ));

}
