use crossterm::event::{read, Event, KeyCode, KeyEvent};
use std::io::{Error, ErrorKind, Write};

mod todo_list;
pub use todo_list::TodoList;

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
p: Purge
q: Quit

Getting All tasks..."
    );

    let mut todo = TodoList::new();
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
                if todo.flush().is_err() {
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
    std::io::stdout().flush().unwrap();
    let mut index = String::new();
    if std::io::stdin().read_line(&mut index).is_err() {
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
    std::io::stdout().flush().unwrap();
    let mut new_task = String::new();
    if std::io::stdin().read_line(&mut new_task).is_err() {
        eprintln!("Could not read input, try again");
        return None;
    };
    Some(new_task.trim().to_owned())
}

pub fn read_operation() -> crossterm::Result<char> {
    print!("\n[a,u,d,c,C,x,p,q]: ");
    std::io::stdout().flush().unwrap();
    crossterm::terminal::enable_raw_mode()?;

    let character = if let Ok(Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        ..
    })) = read()
    {
        Ok(c)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Could not read operation, try again",
        ))
    };
    crossterm::terminal::disable_raw_mode()?;
    character
}

mod test;
