#[derive(Debug, PartialEq)]
struct Task {
    name: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = vec![];

    let t = Task::create("test".to_string());
    println!("{:?}", &t);
    add_task(&mut tasks, Task::create("TEst2".to_string()));
    println!("{:?}", t);
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
        println!(
            "{}. {}",
            list.iter().position(|x| &x == &task).unwrap() + 1,
            task.name
        )
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
