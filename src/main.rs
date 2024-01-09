use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_player)
    .run();
}

#[derive(Component)]
pub struct Player {

}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
)
{
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle{
                transform: Transform::from_xyz(window.width() *0.5,window.height()*0.5,0.0),
                texture: asset_server.load("sprites/mask.png"),
                ..default()
            },
            Player {},
        )
    );

}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() *0.5,window.height()*0.5,0.0),
            ..default()
    }
);

}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(Startup, hello_world)
        .add_systems(Update, print_people_names)
        .add_systems(Update, print_people_with_jobs)
        .add_systems(Update, print_people_without_jobs)
        .add_systems(Update, print_people_jobs);
    }
}

pub fn hello_world() {
    println!("Game started successfully!");
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Carl".to_string(),
        },
        Employed { job: Job::Doctor },
    ));

    commands.spawn(
        Person {
            name: "Paul".to_string(),
        }
    );

    commands.spawn((
        Person {
            name: "George".to_string(),
        },
        Employed { job: Job::FireFighter },
    ));

    commands.spawn((
        Person {
            name: "John".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));

    commands.spawn((
        Person {
            name: "John Jr.".to_string(),
        },
        Employed { job: Job::FireFighter },
    ));

    println!("Hello from setup!");
}

pub fn print_people_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn print_people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("Name: {} as a job",person.name);
    }
}

pub fn print_people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("Name: {} as not a job",person.name);
    }
}

pub fn print_people_jobs(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name: &str = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
        };

        println!("{0} is a {1}.",person.name ,job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
