// include the prelude module to get all the basic bevy modules
use bevy::{prelude::*, math::*};

// bevy is about ECS (Entity Component System)

const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

fn main() {
    // println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins) // include basic plugins like logger, window, input, etc.
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9))) // set background color
        .add_systems(Update, bevy::window::close_on_esc) // exit on ESC
        .add_systems(Startup, setup) // call only once when the game starts
        .add_systems(FixedUpdate, move_paddle) //
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

fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<Time<Fixed>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut(); // get the only mutable transform by the
                                                   // "query", if multiple transform matche, it will panic

    let mut direction = 0.0;
    if input.pressed(KeyCode::A) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        direction += 1.0;
    }

    let new_x = paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds(); // time_step.period.as_sec_f32() is 1 frame's time

    paddle_transform.translation.x = new_x;
}
