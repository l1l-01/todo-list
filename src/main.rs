use std::fs::OpenOptions;
use std::io::{self, Read, Seek, Write};

fn main() {
    // Open the file for reading and writing, creating it if it doesn't exist
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todoapp.txt")
        .expect("Failed to open file!");

    let mut answer = String::new();
    let mut content = String::new();
    let mut task_to_remove = String::new();

    // Read the initial content of the file
    file.read_to_string(&mut content).expect("Failed to read file content!");

    loop {
        answer.clear();
        println!("Enter 1 to add a new task, 2 to remove a task, 3 to show tasks, 4 to quit");
        io::stdin().read_line(&mut answer).expect("Failed to read line!");
        answer = answer.trim().to_string();

        let choice: u8 = answer.parse::<u8>().expect("Failed to parse!");

        match choice {
            1 => {
                // Add a new task
                println!("Enter task:");
                io::stdout().flush().expect("Failed to flush!");
                answer.clear();
                io::stdin().read_line(&mut answer).expect("Failed to read line!");
                file.write_all(answer.as_bytes()).expect("Failed to write to the file!");
                content.push_str(&answer); // Update the content string
            }
            2 => {
                // Remove a task
                println!("Enter the task you want to remove:");
                io::stdout().flush().expect("Failed to flush!");
                task_to_remove.clear();
                io::stdin().read_line(&mut task_to_remove).expect("Failed to read line");
                task_to_remove = task_to_remove.trim().to_string();
                let new_content: String = content.replace(&task_to_remove, "");
                file.set_len(0).expect("Failed to truncate the file!");
                file.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek to the start!");
                file.write_all(new_content.as_bytes()).expect("Failed to rewrite the file!");
                content = new_content; // Update the content string
                println!("{}", content);
            }
            3 => {
                // Show tasks
                file.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek to the start!");
                content.clear();
                file.read_to_string(&mut content).expect("Failed to read content!");
                println!("Your current tasks are:\n{}", content);
            }
            4 => {
                // Quit the application
                println!("Have a great day!");
                break;
            }
            _ => println!("Invalid input"),
        }
    }
}