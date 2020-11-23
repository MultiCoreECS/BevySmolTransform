use bevy::prelude::*;
use bevy::app::AppExit;
use rand::Rng;

fn main() {
    let matches = clap::App::new("BevySmolTransform")
        .version("1.0")
        .author("Blake Wyatt")
        .about("A transform hierarchy experiment in Bevy")
        .arg(clap::Arg::with_name("object_count")
            .short("c")
            .long("object_count")
            .help("Sets the number of objects to generate")
            .takes_value(true))
        .arg(clap::Arg::with_name("update_iterations")
            .short("i")
            .long("update_iterations")
            .help("Sets the number of transform update iterations to perform")
            .takes_value(true))
        .get_matches();

    let object_count = matches.value_of("object_count").unwrap_or("10").parse::<i32>().unwrap_or(10);
    let update_iterations = matches.value_of("update_iterations").unwrap_or("100000").parse::<i32>().unwrap_or(100000);
    
    App::build()
        .add_resource(ObjectCounter(object_count))
        .add_resource(Counter{current: 0, max: update_iterations})
        .add_plugins(MinimalPlugins)
        .add_startup_system(start.system())
        .add_system(apply_rotational_velocity.system())
        .run();
}

fn start(mut commands: Commands, object_counter: Res<ObjectCounter>) {
    let mut rng = rand::thread_rng();
    for id in 0..object_counter.0 {
        commands.spawn((
            Position{
                x: rng.gen_range(0.0, 100.0),
                y: rng.gen_range(0.0, 100.0)
            },
            Rotation(rng.gen_range(0.0, 360.0)),
            RotationalVelocity(rng.gen_range(0.0, 1.0))
        ));
    }
}

struct ObjectCounter(i32);

struct Position {
    x: f32,
    y: f32
}

pub struct Rotation(f32);

pub struct RotationalVelocity(f32);

struct Counter {
    current: i32,
    max: i32,
}

fn apply_rotational_velocity(mut exit: ResMut<Events<AppExit>>, mut counter: ResMut<Counter>, mut objects: Query<(&mut Rotation, &RotationalVelocity)>) {
    for (mut rot, vel) in objects.iter_mut() {
        rot.0 += vel.0;
        rot.0 = rot.0.signum() * rot.0.abs() % 360.0;
    }

    if counter.current < counter.max {
        counter.current += 1;
    } else {
        for (angle, angle_vel) in objects.iter_mut() {
            println!("{} degs : {} per second", angle.0, angle_vel.0);
        }

        exit.send(AppExit{});
    }
}
