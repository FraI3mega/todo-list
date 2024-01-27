mod tasks;

use clap::{Parser, ValueEnum};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use tasks::{add_task, print_tl, remove_task, Task};

#[derive(Parser)]
struct Cli {
    /// What to do
    #[arg(value_enum)]
    mode: Option<Mode>,

    /// Task name/s
    names: Option<Vec<String>>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Add task/s
    Add,
    /// Remove task/s
    Remove,
    /// Mark task/s done
    Done,
    /// Mark task/s undone
    Undone,
    /// Remove tasks marked done
    RemoveDone,
    /// Clear the task list
    Clear,
    /// Export as markdown
    Markdown,
}

fn main() {
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
                if name.parse::<usize>().is_ok()
                    && name.parse::<usize>().unwrap_or_default() <= tasks.len()
                {
                    remove_task(&mut tasks, (name.parse::<usize>().unwrap()) - 1)
                } else {
                    if tasks.iter().any(|x| x.name == name) {
                        let index = tasks.iter().position(|x| x.name == name).unwrap();
                        remove_task(&mut tasks, index)
                    }
                }
            }
        }
        Some(Mode::Done) => {
            for name in cli.names.unwrap_or_default() {
                if name.parse::<usize>().is_ok()
                    && name.parse::<usize>().unwrap_or_default() <= tasks.len()
                {
                    tasks[(name.parse::<usize>().unwrap()) - 1].mark_done();
                } else {
                    if let Some(n) = tasks.iter_mut().find(|x| x.name == name) {
                        n.mark_done();
                    }
                }
            }
        }
        Some(Mode::Undone) => {
            for name in cli.names.unwrap_or_default() {
                if name.parse::<usize>().is_ok()
                    && name.parse::<usize>().unwrap_or_default() <= tasks.len()
                {
                    tasks[(name.parse::<usize>().unwrap()) - 1].mark_undone();
                } else {
                    if let Some(n) = tasks.iter_mut().find(|x| x.name == name) {
                        n.mark_undone();
                    }
                }
            }
        }
        Some(Mode::RemoveDone) => tasks.retain(|x| !x.done),
        Some(Mode::Clear) => tasks = vec![],
        Some(Mode::Markdown) => {
            tasks.iter().for_each(|x| {
                if x.done {
                    println!("- [x] {}", x.name)
                } else {
                    println!("- [ ] {}", x.name)
                }
            });
            return;
        }

        None => {} //print_tl(&tasks)
    }
    print_tl(&tasks);

    let file_w = match File::create("TaskList.json") {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    save_tl(tasks, file_w);
}

fn load_tl(file: File) -> Vec<Task> {
    if fs::read_to_string("TaskList.json").unwrap_or_default() != *"" {
        serde_json::from_reader(file).unwrap()
    } else {
        vec![]
    }
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
