mod task;
use std::io::{self, Write};
use task::Task;

fn menu() {
    println!("\nTodo List Menu:\n1. Show all tasks\n2. Add a task\n3. Edit a task\n4. Remove a task\n5. Exit");
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();

    let _ = &todo_list.push(Task::new(
        1,
        "Garbage".to_string(),
        "Take out the garbage".to_string(),
    ));

    loop {
        menu();
        println!("Please select an option:  ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                // print tasks in list
                println!("Tasks:");
                for task in &todo_list {
                    println!(
                        "[{}] {} - {} (Completed: {})",
                        task.id, task.title, task.description, task.completed
                    );
                }
            }
            2 => {
                // add a task
            }
            3 => {
                // edit a task
            }
            4 => {
                // remove a task
            }
            5 => {
                // quit
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Please select a number between 1 and 5");
            }
        }
    }
}
