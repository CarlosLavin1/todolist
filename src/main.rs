mod task;
use std::io::{self, Write};
use task::Task;

fn menu() {
    println!("\nTodo List Menu:\n1. Show all tasks\n2. Add a task\n3. Edit a task\n4. Remove a task\n5. Exit");
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();
    let mut next_id:usize = 1;

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
                println!("Enter task title:");
                let mut title: String = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let title = title.trim().to_string();

                println!("Enter task description:");
                let mut description: String = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description: String = description.trim().to_string();

                let id: usize = {
                    next_id += 1;
                    next_id
                };
                todo_list.push(Task::new(id, title, description));
                println!("Task added successfully.");
            }
            3 => {
                // edit a task
                println!("Enter task ID to edit:");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                let id: usize = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                println!("Enter new title:");
                let mut new_title = String::new();
                io::stdin().read_line(&mut new_title).expect("Failed to read line");
                let new_title = new_title.trim().to_string();

                println!("Enter new description:");
                let mut new_description = String::new();
                io::stdin().read_line(&mut new_description).expect("Failed to read line");
                let new_description = new_description.trim().to_string();

                println!("Is the task completed? (y/n)");
                let mut new_completed = String::new();
                io::stdin().read_line(&mut new_completed).expect("Failed to read line");
                // check valid input
                while !new_completed.trim().eq_ignore_ascii_case("y") && !new_completed.trim().eq_ignore_ascii_case("n"){
                    println!("Is the task completed? (y/n)");
                    // read_line() appends to string, so we have to clear the value
                    new_completed = String::new();
                    io::stdin().read_line(&mut new_completed).expect("Failed to read line");
                }
                let new_completed_bool: bool = new_completed.trim().eq_ignore_ascii_case("y");

                // check if id exists in list
                if let Some(task) = todo_list.iter_mut().find(|t| t.id == id) {
                    task.title = new_title;
                    task.description = new_description;
                    task.completed = new_completed_bool;
                } else {
                    println!("Task with ID {} not found.", id);
                }
            }
            4 => {
                // remove a task
                println!("Enter task ID to remove:");
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                let id: usize = match id_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };
                // check if id exists in list
                if let Some(index) = todo_list.iter().position(|t| t.id == id) {
                    todo_list.remove(index);
                } else {
                    println!("Task with ID {} not found.", id);
                }            
            }
            5 => {
                // quit
                println!("Goodbye!");
                break;
            }
            // default case
            _ => {
                println!("Please select a number between 1 and 5");
            }
        }
    }
}
