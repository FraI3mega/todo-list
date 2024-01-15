mod tasks;

use clap::{Parser, ValueEnum};
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
    print_tl(&tasks);
}
