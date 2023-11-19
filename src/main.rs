// include the prelude module to get all the basic bevy modules
use bevy::prelude::*;

fn main() {
    // println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins) // include basic plugins like logger, window, input, etc.
        .add_systems(Update, bevy::window::close_on_esc) // exit on ESC
        .run()
}
