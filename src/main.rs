use bevy::prelude::*;

#[derive(Component)]
struct Particle {
    x: f64,
    y: f64,
}


fn hello(mut commands: Commands) {
    commands.spawn(Particle {x: 100.0, y: 100.0});
}

fn test(query: Query<&Particle>) {
    for particle in query {
        println!("{},{}", particle.x, particle.y)
    }
}

fn change_pos(mut query: Query<&mut Particle>) {
    for particle in query {
        particle.x += 5.0;
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, hello)
    .add_systems(Update, test)
    .run();
}