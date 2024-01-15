use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) done: bool,
}

/// add a to to the tasklist
pub(crate) fn add_task(list: &mut Vec<Task>, task: Task) {
    list.push(task);
}

/// Remove item from tasklist. Takes in an index starting at 0
pub(crate) fn remove_task(list: &mut Vec<Task>, index: usize) {
    list.remove(index);
}

/// pretty prints the task list
pub(crate) fn print_tl(list: &Vec<Task>) {
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
    use crate::tasks::{add_task, remove_task, Task};

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
