use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    // collect user-supplied args into a Vec from std::env::args()
    let args: Vec<String> = std::env::args().collect();

    // args[0] is the name of the program
    // args[1] would be a supplied command
    // if the number of args is < 2, we haven't received a command
    // return early
    if args.len() < 2 {
        println!("Usage: {} <command> [args]", args[0]);
        println!("Commands:");
        println!("  add <task>    Add a new task");
        println!("  delete <number>    Delete a task by number");
        return Ok(());
    }

    // borrow the value of args[1] as the supplied command
    let command = &args[1];

    // match on command as a string slice (such that each match arm can be "<some_string_slice>")
    match command.as_str() {
        "add" => {
            // add requires 3 args <program_name> add (command) <task>
            if args.len() < 3 {
                println!("Please supply a task when using add");
                println!("  Usage: {} add <task_description>", args[0]);
                return Ok(());
            }
            let task = &args[2..].join(" ");
            add_task(task);
            print_tasks();
        }
        "delete" => {
            if args.len() < 3 {
                println!("Please supply a task number when using delete");
                println!("  Usage: {} delete <number>", args[0]);
                return Ok(());
            }
            let task_number: usize = args[2].parse().expect("Please enter a valid number");
            delete_task(task_number);
            print_tasks();
        }
        "print" => print_tasks(),
        _ => {
            println!("Unknown command: {}", command);
            println!("Usage: {} <command> [args]", args[0]);
            println!("Commands:");
            println!("  add <task>       Add a new task");
            println!("  delete <number>  Delete a task by number");
            return Ok(());
        }
    }

    Ok(())
}

fn print_tasks() {
    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt")
        .expect("Cannot open file");

    let f = BufReader::new(file);

    let mut count = 0;
    for line in f.lines() {
        count += 1;
        match line {
            Ok(content) => println!("{count}. {}", content),
            Err(e) => println!("Error reading line: {}", e),
        }
    }
}

fn add_task(task: &str) {
    println!("Adding task...");
    let task = String::from(task);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Cannot open file");

    writeln!(file, "{}", task.trim()).expect("Cannot write to file");
}

fn delete_task(task_number: usize) {
    println!("Deleting task...");

    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt")
        .expect("Cannot open file");

    let reader = BufReader::new(file);
    let mut tasks: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("Error reading lines");

    if task_number == 0 || task_number > tasks.len() {
        println!("Invalid task number");
        return;
    }

    tasks.remove(task_number - 1);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.txt")
        .expect("Cannot open file");

    let mut writer = BufWriter::new(file);

    for task in tasks {
        writeln!(writer, "{}", task).expect("Error writing to file");
    }

    println!("Task deleted successfully");
}
