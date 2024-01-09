use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PeoplePlugin)
    .run();
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
