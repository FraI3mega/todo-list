# Todo-list



A simple todo-list cli program.

> [!IMPORTANT]
> Currently the program saves the tasklist in the current working directory. This will most likely change in the future.

[![asciicast](https://asciinema.org/a/boDzy6wuQ3EV32g7qxUgWrhh8.svg)](https://asciinema.org/a/boDzy6wuQ3EV32g7qxUgWrhh8)

## Installation
Just use cargo
```bash
cargo install --git https://github.com/FraI3mega/todo-list.git --locked
```

## Usage
> [!TIP]
> Almost all of the below operations can be used with names of the task or the index numbers.

### add
Adds a task/s to the tasklist. You can specify the time left in minutes, hours or days.
```bash
# Adds a task named "Write a better README" without a deadline
$ task-list add "Write a better README" 

# Adds tasks named "Refactor the code" and "Fix EOF bug"
$ task-list add "Refactor the code" "Fix EOF bug"

# Add a task named "Clean up the code" with a 3 day deadline
$ task-list add "Clean up the code" -t 3d
```

### remove
Removes a task from the tasklist. 
```bash
# Removes the task named "Clean up the code"
$ task-list remove "Clean up the code"

# Removes the second entry from the tasklist
$ task-list remove 2
```
### remove-done
Removes only the done tasks.
```bash
# Removes th edone tasks from the list
$ todo-list remove-done
```

### clear
removes **all** the items from the tasklist
```bash
$ todo-list clear
```

### done
Marks a task/s done
```bash
# Marks task "Refactor the code" done
$ todo-list done "Refactor the code"
```

### undone
Does the opposite of ```done```. It marks the task as not done yet.
```bash
# Marks task "Write a better README" undone
$ todo-list undone "Write a better README"
```

### markdown
Exports the tasklist as markdown
```bash
$ todo-list markdown
```
