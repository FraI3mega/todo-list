#![forbid(unsafe_code)]

mod tasks;

use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;
use regex::{Captures, Regex};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use tasks::{add_task, print_tl, remove_task, DueDate, Task};
use time::macros::format_description;
use time::{Duration, OffsetDateTime, UtcOffset};

#[derive(Parser)]
struct Cli {
    /// What to do
    #[arg(value_enum)]
    mode: Option<Mode>,

    /// Task name/s
    names: Option<Vec<String>>,

    /// Due in time
    #[arg(short, long, value_name = "TIME")]
    time: Option<String>,
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

fn main() -> Result<()> {
    color_eyre::install()?;
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
                if cli.time.is_some() {
                    add_task(
                        &mut tasks,
                        Task::create(name, Some(parse_due_date(cli.time.clone().unwrap())?)),
                    )
                } else {
                    add_task(&mut tasks, Task::create(name, None))
                }
            }
        }
        Some(Mode::Remove) => {
            for name in cli.names.unwrap_or_default() {
                if name.parse::<usize>().is_ok()
                    && name.parse::<usize>().unwrap_or_default() <= tasks.len()
                {
                    remove_task(&mut tasks, (name.parse::<usize>().unwrap()) - 1)
                } else if tasks.iter().any(|x| x.name == name) {
                    let index = tasks.iter().position(|x| x.name == name).unwrap();
                    remove_task(&mut tasks, index)
                }
            }
        }
        Some(Mode::Done) => {
            for name in cli.names.unwrap_or_default() {
                if name.parse::<usize>().is_ok()
                    && name.parse::<usize>().unwrap_or_default() <= tasks.len()
                {
                    tasks[(name.parse::<usize>().unwrap()) - 1].mark_done();
                } else if let Some(n) = tasks.iter_mut().find(|x| x.name == name) {
                    n.mark_done();
                }
            }
        }
        Some(Mode::Undone) => {
            for name in cli.names.unwrap_or_default() {
                if name.parse::<usize>().is_ok()
                    && name.parse::<usize>().unwrap_or_default() <= tasks.len()
                {
                    tasks[(name.parse::<usize>().unwrap()) - 1].mark_undone();
                } else if let Some(n) = tasks.iter_mut().find(|x| x.name == name) {
                    n.mark_undone();
                }
            }
        }
        Some(Mode::RemoveDone) => tasks.retain(|x| !x.done),
        Some(Mode::Clear) => tasks = vec![],
        Some(Mode::Markdown) => {
            print_as_md(tasks);
            return Ok(());
        }

        None => {} //print_tl(&tasks)
    }
    print_tl(&tasks);

    let file_w = File::create("TaskList.json")?;

    save_tl(tasks, file_w);

    Ok(())
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

fn parse_due_date(input: String) -> Result<OffsetDateTime> {
    let re = Regex::new(r"^(?<amount>\d+)(?<unit>[m,h,d])$").unwrap();

    let caps: Captures<'_> = match re.captures(&input) {
        Some(cap) => cap,
        None => panic!("Can't extract time from {input}"),
    };

    let due_date = match &caps["unit"] {
        "m" => OffsetDateTime::now_utc() + Duration::minutes(caps["amount"].parse()?),
        "h" => OffsetDateTime::now_utc() + Duration::hours(caps["amount"].parse()?),
        "d" => OffsetDateTime::now_utc() + Duration::days(caps["amount"].parse()?),
        &_ => panic!("Unit not found"),
    };

    Ok(due_date)
}

fn print_as_md(task_list: Vec<Task>) {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    task_list.iter().for_each(|task| {
        if task.due_date == DueDate::NoDueDate {
            if task.done {
                println!("- [x] {}", task.name)
            } else {
                println!("- [ ] {}", task.name)
            }
        } else {
            let due_time = (if let DueDate::DueDate(n) = task.due_date {
                n
            } else {
                unreachable!()
            })
            .to_offset(UtcOffset::current_local_offset().unwrap())
            .format(format)
            .unwrap();
            if task.done {
                println!("- [x] {}", task.name)
            } else {
                println!("- [ ] {} - Due on {}", task.name, due_time)
            }
        }
    });
}
