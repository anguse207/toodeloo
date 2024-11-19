use task::Task;

mod task;
mod task_tank;
mod user;

fn main() {
    let tasks: Vec<Task> = vec![];

    let alice = user::User::new("Alice".to_string());
    let bob = user::User::new("Bob".to_string());

    println!("Hello, world!");
}
