#[cfg(test)]
use super::*;

#[test]
fn add_task_test() {
    let mut todo_list = Todo::new();
    let len_before = todo_list.tasks.len();
    todo_list.add_task("Buy milk".to_owned());
    todo_list.add_task("Make todo cli app".to_owned());
    let len_after = todo_list.tasks.len();
    assert_eq!((len_after - len_before), 2 as usize);
}

#[test]
fn remove_task_test() {
    let mut todo_list = Todo::new();
    todo_list.add_task("Buy milk".to_owned());
    todo_list.add_task("Make todo cli app".to_owned());
    todo_list.add_task("I love linux".to_owned());
    let len_before = todo_list.tasks.len();
    todo_list.remove_task(1);
    let len_after = todo_list.tasks.len();
    assert_eq!((len_before - len_after), 1 as usize);
}

#[test]
fn update_task_test() {
    let mut todo_list = Todo::new();
    todo_list.add_task("Buy milk".to_owned());
    todo_list.add_task("Make todo cli app".to_owned());
    todo_list.add_task("I love linux".to_owned());
    todo_list.update_task(1, "Make the greatest todo app ever".to_owned());
    assert_eq!(
        todo_list.tasks[1].content,
        "Make the greatest todo app ever"
    );
}

#[test]
fn change_status_test() {
    let mut todo_list = Todo::new();
    todo_list.add_task("Buy milk".to_owned());
    todo_list.add_task("Make todo cli app".to_owned());
    todo_list.add_task("I love linux".to_owned());
    todo_list.complete_task(1);
    assert!(todo_list.tasks[1].is_completed);
    todo_list.uncomplete_task(1);
    assert!(!todo_list.tasks[1].is_completed);
}

#[test]
fn purge_test() {
    let mut todo_list = Todo::new();
    todo_list.add_task("Buy milk".to_owned());
    todo_list.add_task("Make todo cli app".to_owned());
    todo_list.add_task("I love linux".to_owned());
    todo_list.purge();
    assert_eq!(todo_list.tasks.len(), 0 as usize);
}
