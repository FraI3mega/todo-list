mod tasks;

use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{BufWriter, Write};
use tasks::{add_task, print_tl, remove_task, Task};

#[derive(Parser)]
struct Cli {
    /// What to do
    #[arg(value_enum)]
    mode: Option<Mode>,

    names: Option<Vec<String>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Add,
    Remove,
    Done,
}

fn main() {
    let mut tasks: Vec<Task> = vec![];
    let cli = Cli::parse();
    let file_r = match File::open("TaskList.json") {
        Ok(f) => f,
        Err(_) => {
            File::create("TaskList.json").unwrap();
            File::open("TaskList.json").unwrap()
        }
    };
    let mut tasks = load_tl(file_r);

    match cli.mode {
        Some(Mode::Add) => {
            for name in cli.names.unwrap_or_default() {
                add_task(&mut tasks, Task::create(name));
            }
        }
        Some(Mode::Remove) => {
            for name in cli.names.unwrap_or_default() {
                if tasks.iter().any(|x| x.name == name) {
                    let index = tasks.iter().position(|x| x.name == name).unwrap();
                    remove_task(&mut tasks, index)
                }
            }
        }
        Some(Mode::Done) => {
            for name in cli.names.unwrap_or_default() {
                if let Some(n) = tasks.iter_mut().find(|x| x.name == name) {
                    n.mark_done();
                }
            }
        }
        None => print_tl(&tasks),
    }

    let mut file_w = match File::create("TaskList.json") {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    save_tl(tasks, file_w);
}

fn load_tl(file: File) -> Vec<Task> {
    serde_json::from_reader(file).unwrap()
}

fn save_tl(task_list: Vec<Task>, file: File) {
    let mut writer = BufWriter::new(file);
    match serde_json::to_writer(&mut writer, &task_list) {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    };
    match writer.flush() {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    };
}
