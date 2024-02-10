use color_eyre::owo_colors::OwoColorize;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) done: bool,
    pub(crate) due_date: DueDate,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) enum DueDate {
    #[serde(with = "time::serde::timestamp")]
    DueDate(OffsetDateTime),
    NoDueDate,
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
pub(crate) fn print_tl(list: &[Task]) {
    list.iter().for_each(|task| {
        if task.due_date == DueDate::NoDueDate {
            if !task.done {
                println!(
                    "{}. {}",
                    list.iter().position(|x| x == task).unwrap() + 1,
                    &task.name.blue()
                )
            } else {
                println!(
                    "{}. {}",
                    list.iter().position(|x| x == task).unwrap() + 1,
                    &task.name.as_str().bright_black().strikethrough()
                )
            }
        } else {
            let current_time = OffsetDateTime::now_utc();
            let task_due_time = if let DueDate::DueDate(n) = task.due_date {
                n
            } else {
                unreachable!()
            };
            let dur: Duration = current_time - task_due_time;

            let is_overdue: bool = task_due_time < current_time;

            let dur_text: String = format!(
                "{} {}",
                if is_overdue {
                    "overdue by".bright_red()
                } else {
                    "due in".bright_green()
                },
                if dur.whole_weeks() != 0 {
                    format!("{}w", dur.whole_weeks().abs())
                } else if dur.whole_days() != 0 {
                    format!("{}d", dur.whole_days().abs())
                } else if dur.whole_hours() != 0 {
                    format!("{}h", dur.whole_hours().abs())
                } else if dur.whole_minutes() != 0 {
                    format!("{}m", dur.whole_minutes().abs())
                } else if dur.whole_seconds() != 0 {
                    format!("{}s", dur.whole_seconds().abs())
                } else {
                    "now".to_string()
                }
                .yellow()
            );

            if !task.done {
                println!(
                    "{}. {} - {}",
                    list.iter().position(|x| x == task).unwrap() + 1,
                    &task.name.blue(),
                    dur_text
                )
            } else {
                println!(
                    "{}. {}",
                    list.iter().position(|x| x == task).unwrap() + 1,
                    &task.name.as_str().bright_black().strikethrough()
                )
            }
        }
    });
}

impl Task {
    ///creates a undone task
    pub fn create(name: String, due_date: Option<OffsetDateTime>) -> Self {
        Task {
            name,
            done: false,
            due_date: if let Some(n) = due_date {
                DueDate::DueDate(n)
            } else {
                DueDate::NoDueDate
            },
        }
    }

    ///marks task done
    pub fn mark_done(&mut self) {
        self.done = true;
    }

    ///marks task undone
    pub fn mark_undone(&mut self) {
        self.done = false;
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use crate::tasks::{add_task, remove_task, DueDate, Task};

    #[test]
    fn add_task_test() {
        assert_eq!(
            Task::create("Test".to_string(), None),
            Task {
                name: "Test".to_string(),
                done: false,
                due_date: DueDate::NoDueDate,
            }
        );
    }

    #[test]
    fn add_task_with_due_date() {
        assert_eq!(
            Task::create("Test".to_string(), Some(datetime!(2024-02-10 12:00:00 UTC))),
            Task {
                name: "Test".to_string(),
                done: false,
                due_date: DueDate::DueDate(datetime!(2024-02-10 12:00:00 UTC))
            }
        )
    }

    #[test]
    fn mark_task_done() {
        let mut t = Task::create("test".to_string(), None);
        t.mark_done();
        assert_eq!(
            t,
            Task {
                name: "test".to_string(),
                done: true,
                due_date: DueDate::NoDueDate,
            }
        );
    }

    #[test]
    fn create_task_list() {
        let mut list: Vec<Task> = vec![];
        assert!(list.is_empty());
        add_task(&mut list, Task::create("test".to_string(), None));
        assert_eq!(list, vec![Task::create("test".to_string(), None)]);
        remove_task(&mut list, 0);
        assert!(list.is_empty());
    }
}
