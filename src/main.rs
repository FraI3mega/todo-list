use clap::{Parser, ValueEnum};
use colored::Colorize;

#[derive(Debug, PartialEq)]
struct Task {
    name: String,
    done: bool,
}

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
                if let Some(_) = tasks.iter().find(|x| x.name == name) {
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

/// add a to to the tasklist
fn add_task(mut list: &mut Vec<Task>, task: Task) {
    list.push(task);
}

/// Remove item from tasklist. Takes in an index starting at 0
fn remove_task(mut list: &mut Vec<Task>, index: usize) {
    list.remove(index);
}

/// pretty prints the task list
fn print_tl(list: &Vec<Task>) {
    for task in list {
        if &task.done == &false {
            println!(
                "{}. {}",
                list.iter().position(|x| &x == &task).unwrap() + 1,
                &task.name.blue()
            )
        } else {
            println!(
                "{}. {}",
                list.iter().position(|x| &x == &task).unwrap() + 1,
                &task.name.as_str().bright_black().strikethrough()
            )
        }
    }
}

impl Task {
    ///creates a undone task
    pub fn create(name: String) -> Self {
        Task { name, done: false }
    }

    ///marks task done
    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::{add_task, remove_task, Task};

    #[test]
    fn atask() {
        assert_eq!(
            Task::create("Test".to_string()),
            Task {
                name: "Test".to_string(),
                done: false
            }
        );
    }

    #[test]
    fn done() {
        let mut t = Task::create("test".to_string());
        t.mark_done();
        assert_eq!(
            t,
            Task {
                name: "test".to_string(),
                done: true
            }
        );
    }

    #[test]
    fn tl() {
        let mut list: Vec<Task> = vec![];
        assert!(list.is_empty());
        add_task(&mut list, Task::create("test".to_string()));
        assert_eq!(list, vec![Task::create("test".to_string())]);
        remove_task(&mut list, 0);
        assert!(list.is_empty());
    }
}
