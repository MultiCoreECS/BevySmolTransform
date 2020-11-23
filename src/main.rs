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
        .arg(clap::Arg::with_name("transform_type")
            .short("t")
            .long("transform_type")
            .help("Sets the type of transform. 0=all, 1=rotation, 2=scale, 3=translation")
            .takes_value(true))
        .get_matches();

    let object_count = matches.value_of("object_count").unwrap_or("10").parse::<i32>().unwrap_or(10);
    let update_iterations = matches.value_of("update_iterations").unwrap_or("100000").parse::<i32>().unwrap_or(100000);
    let transform_type = matches.value_of("transform_type").unwrap_or("0").parse::<i32>().unwrap_or(0);
    
    match transform_type {
        0 => App::build()
                .add_resource(ObjectCounter(object_count))
                .add_resource(Counter{current: 0, max: update_iterations})
                .add_plugins(MinimalPlugins)
                .add_startup_system(start.system())
                .add_system(apply_rotational_velocity.system())
                .add_system(apply_scale_adjustment.system())
                .add_system(apply_translation_adjustment.system())
                .add_system(count_then_exit.system())
                .run(),
        1 => App::build()
                .add_resource(ObjectCounter(object_count))
                .add_resource(Counter{current: 0, max: update_iterations})
                .add_plugins(MinimalPlugins)
                .add_startup_system(start.system())
                .add_system(apply_rotational_velocity.system())
                .add_system(count_then_exit.system())
                .run(),
        2 => App::build()
                .add_resource(ObjectCounter(object_count))
                .add_resource(Counter{current: 0, max: update_iterations})
                .add_plugins(MinimalPlugins)
                .add_startup_system(start.system())
                .add_system(apply_scale_adjustment.system())
                .add_system(count_then_exit.system())
                .run(),
        3 => App::build()
                .add_resource(ObjectCounter(object_count))
                .add_resource(Counter{current: 0, max: update_iterations})
                .add_plugins(MinimalPlugins)
                .add_startup_system(start.system())
                .add_system(apply_translation_adjustment.system())
                .add_system(count_then_exit.system())
                .run(),
        _ => {
            println!("Invalid transform type. Action cancelled.");
            return;
        }
    }
}

fn start(mut commands: Commands, object_counter: Res<ObjectCounter>) {
    commands.spawn((
        Id(0),
    ));

    let mut rng = rand::thread_rng();
    for id in 0..object_counter.0 {
        commands.spawn((
            Id(id as usize+1),
            Parent(id as usize),
            Position{
                x: rng.gen_range(0.0, 100.0),
                y: rng.gen_range(0.0, 100.0)
            },
            Scale(rng.gen_range(0.0, 2.0)),
            Rotation(rng.gen_range(0.0, 360.0)),
            RotationalVelocity(rng.gen_range(0.0, 1.0))
        ));
    }
}

struct ObjectCounter(i32);

pub struct Id(usize);

pub struct Parent(usize);

struct Position {
    x: f32,
    y: f32
}

pub struct Scale(f32);

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
}

fn apply_scale_adjustment(mut exit: ResMut<Events<AppExit>>, mut counter: ResMut<Counter>, mut objects: Query<(&mut Scale)>) {
    let mut rng = rand::thread_rng();

    for (mut s) in objects.iter_mut() {
        s.0 += rng.gen_range(-0.5, 0.5);
    }
}

fn apply_translation_adjustment(mut exit: ResMut<Events<AppExit>>, mut counter: ResMut<Counter>, mut objects: Query<(&mut Position)>) {
    let mut rng = rand::thread_rng();

    for (mut p) in objects.iter_mut() {
        p.x += rng.gen_range(-5.0, 5.0);
        p.y += rng.gen_range(-5.0, 5.0);
    }
}

fn count_then_exit(mut exit: ResMut<Events<AppExit>>, mut counter: ResMut<Counter>, info: Query<(&Id, &Parent, &Position, &Scale, &Rotation, &RotationalVelocity)>){
    if counter.current < counter.max {
        counter.current += 1;
    } else {
        for (id, parent, position, scale, angle, angle_vel) in info.iter() {
            println!("ID {} : Parent {} : ({}, {}) : Scaled {} units : {} degs : {} per second", id.0, parent.0, position.x, position.y, scale.0, angle.0, angle_vel.0);
        }

        exit.send(AppExit{});
    }
}