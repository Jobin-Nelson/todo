use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct TodoList {
    tasks: Vec<TodoItem>,
    todo_path: PathBuf,
}

impl TodoList {
    pub fn new() -> Self {
        let home_path = std::env::var("HOME").expect("Could not find $HOME path");

        let todo_path: PathBuf = [&home_path, ".cache", "todo_cli", "todo.txt"]
            .iter()
            .collect();

        fs::create_dir_all(todo_path.parent().unwrap()).expect("Could not cache directory");

        if !todo_path.is_file() {
            return TodoList {
                tasks: Vec::new(),
                todo_path,
            };
        }

        let file = fs::File::open(todo_path.as_path()).expect("Could not open todo file");
        let reader = BufReader::new(file);

        let mut todo_list = TodoList {
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
