use bevy::prelude::*;

fn main() {
  App::new().add_plugins((DefaultPlugins, PeoplePlugin)).run();
}

pub fn hello_world() {
  println!("Hello World!");
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Employed {
  pub job: Job,
}

#[derive(Component)]
pub enum Job {
  Doctor,
  FireFighter,
  Lawyer,
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, add_people).add_systems(
      Update,
      (
        hello_world,
        greet_people,
        people_with_jobs,
        people_ready_for_hire,
        person_does_job,
      ),
    );
  }
}

pub fn add_people(mut commands: Commands) {
  commands.spawn((Person, Name("Renzo Hume".to_string())));

  commands.spawn((
    Person,
    Name("Elaina Proctor".to_string()),
    Employed { job: Job::Doctor },
  ));

  commands.spawn((
    Person,
    Name("Zayna Nieves".to_string()),
    Employed {
      job: Job::FireFighter,
    },
  ));
}

pub fn greet_people(query: Query<&Name, With<Person>>) {
  for name in &query {
    println!("hello {}!", name.0);
  }
}

pub fn people_with_jobs(query: Query<&Name, With<Employed>>) {
  for name in &query {
    println!("{} has a job!", name.0);
  }
}

pub fn people_ready_for_hire(query: Query<&Name, Without<Employed>>) {
  for name in &query {
    println!("{} is ready for hire!", name.0);
  }
}

pub fn person_does_job(query: Query<(&Name, &Employed)>) {
  for (name, employed) in &query {
    let job_name: &str = match employed.job {
      Job::Doctor => "Doctor",
      Job::FireFighter => "Fire Fighter",
      Job::Lawyer => "Lawyer",
    };
    println!("{0} is a {1}", name.0, job_name);
  }
}
