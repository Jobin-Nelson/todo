use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use std::io::{BufRead, BufReader, Write, Error, ErrorKind};
use std::path::PathBuf;
use std::{fs, io};

pub struct Todo {
    tasks: Vec<TodoItem>,
    todo_path: PathBuf,
}

impl Todo {
    pub fn new() -> Self {
        let home_path = std::env::var("HOME").expect("Could not find $HOME path");

        let todo_path: PathBuf = [&home_path, ".cache", "todo_cli", "todo.txt"]
            .iter()
            .collect();

        fs::create_dir_all(todo_path.parent().unwrap()).expect("Could not cache directory");

        if !todo_path.is_file() {
            return Todo {
                tasks: Vec::new(),
                todo_path,
            };
        }

        let file = fs::File::open(todo_path.as_path()).expect("Could not open todo file");
        let reader = BufReader::new(file);

        let mut todo_list = Todo {
            tasks: Vec::new(),
            todo_path,
        };

        for line in reader.lines() {
            if let Ok(line) = line {
                let (status, content) = line.split_at(1);
                let todo_item = TodoItem {
                    content: content.to_owned(),
                    is_completed: if status == "T" { true } else { false },
                };
                todo_list.tasks.push(todo_item);
            }
        }

        todo_list
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(TodoItem {
            content: task,
            is_completed: false,
        });
    }

    pub fn get_task(&self) {
        let completed = "🗹";
        let not_completed = "☐";
        println!();
        self.tasks.iter().enumerate().for_each(|(index, todo)| {
            let status = if todo.is_completed {
                completed
            } else {
                not_completed
            };
            println!("{} {}  {}", index, status, todo.content);
        })
    }

    pub fn remove_task(&mut self, index: u8) {
        let index = index as usize;
        if index < self.tasks.len() {
            self.tasks.remove(index);
        } else {
            eprintln!("Index out of task range");
        }
    }

    pub fn update_task(&mut self, index: u8, new_task: String) {
        match self.tasks.get_mut(index as usize) {
            Some(task) => task.content = new_task,
            None => eprintln!("Index out of task range, try again"),
        };
    }

    pub fn complete_task(&mut self, index: u8) {
        match self.tasks.get_mut(index as usize) {
            Some(task) => task.is_completed = true,
            None => eprintln!("Index out of task range, try again"),
        }
    }

    pub fn uncomplete_task(&mut self, index: u8) {
        match self.tasks.get_mut(index as usize) {
            Some(task) => task.is_completed = false,
            None => eprintln!("Index out of task range, try again"),
        }
    }

    pub fn clean(&mut self) {
        self.tasks.retain(|task| !task.is_completed);
    }

    pub fn purge(&mut self) {
        self.tasks = Vec::new();
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.todo_path)?;

        let completed = "T";
        let not_completed = "F";

        let contents = self
            .tasks
            .iter()
            .map(|task| {
                let status = if task.is_completed {
                    completed
                } else {
                    not_completed
                };
                status.to_owned() + task.content.trim()
            })
            .collect::<Vec<String>>()
            .join("\n");

        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}

struct TodoItem {
    content: String,
    is_completed: bool,
}

pub fn run() {
    println!(
        "\
Welcome to Todo App

a: Add
u: Update
d: Delete
c: Complete
C: Not Complete
x: Clean
p: purge
q: quit

Getting All tasks..."
    );

    let mut todo = Todo::new();
    loop {
        todo.get_task();
        let operation = if let Ok(o) = read_operation() {
            o
        } else {
            continue;
        };

        match operation {
            'a' => {
                print!("\nAdd a new task: ");
                let new_task = if let Some(t) = read_task() {
                    t
                } else {
                    continue;
                };
                todo.add_task(new_task);
            }
            'u' => {
                print!("\nEnter the task number you want to update: ");
                let index = if let Some(i) = read_index() {
                    i
                } else {
                    continue;
                };

                print!("\nUpdate the task: ");
                let updated_task = if let Some(t) = read_task() {
                    t
                } else {
                    continue;
                };
                todo.update_task(index, updated_task);
            }
            'd' => {
                print!("\nEnter the task number you want to delete: ");
                let index = if let Some(i) = read_index() {
                    i
                } else {
                    continue;
                };
                todo.remove_task(index);
            }
            'c' => {
                print!("\nEnter the task number you want to mark as complete: ");
                let index = if let Some(i) = read_index() {
                    i
                } else {
                    continue;
                };
                todo.complete_task(index);
            }
            'C' => {
                print!("\nEnter the task number you want to mark as not complete: ");
                let index = if let Some(i) = read_index() {
                    i
                } else {
                    continue;
                };
                todo.uncomplete_task(index);
            }
            'x' => {
                println!("\nCleaning up all completed tasks...");
                todo.clean();
            }
            'p' => {
                println!("\nPurging all tasks...");
                todo.purge();
            }
            'q' => {
                if let Err(_) = todo.flush() {
                    eprintln!("Could not write tasks to a file");
                };
                break;
            }
            _ => {
                eprintln!("Invalid input try again");
            }
        };
    }

    println!("\nGoodbye !");
}

pub fn read_index() -> Option<u8> {
    io::stdout().flush().unwrap();
    let mut index = String::new();
    if let Err(_) = io::stdin().read_line(&mut index) {
        eprintln!("Could not read input, try again");
        return None;
    };
    let index = if let Ok(i) = index.trim().parse::<u8>() {
        i
    } else {
        eprintln!("Could not parse task number {}, try again", index.trim());
        return None;
    };
    Some(index)
}

pub fn read_task() -> Option<String> {
    io::stdout().flush().unwrap();
    let mut new_task = String::new();
    if let Err(_) = io::stdin().read_line(&mut new_task) {
        eprintln!("Could not read input, try again");
        return None;
    };
    Some(new_task.trim().to_owned())
}

pub fn read_operation() -> crossterm::Result<char> {
    print!("\n[a,u,d,c,C,x,p,q]: ");
    io::stdout().flush().unwrap();
    terminal::enable_raw_mode()?;
    if let Ok(Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        ..
    })) = read()
    {
        terminal::disable_raw_mode()?;
        Ok(c)
    } else {
        terminal::disable_raw_mode()?;
        Err(Error::new(
            ErrorKind::Other,
            "Could not read operation, try again",
        ))
    }
}

mod test;
