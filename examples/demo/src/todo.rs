use std::cell::RefCell;

#[derive(Clone, Debug)]

pub struct Todo {
    pub id: usize,
    pub text: String,
}

thread_local! {
    static ID: RefCell<usize> = RefCell::new(0);
}

pub fn add_todo(todos: &mut Vec<Todo>, text: Option<String>) {
    let id = ID.with_borrow_mut(|mut_id| {
        let id = *mut_id;
        *mut_id += 1;
        id
    });
    let text = text.unwrap_or_else(|| format!("Todo item #{id}"));
    todos.push(Todo { id, text });
}

pub fn delete_todo(todos: &mut Vec<Todo>, id: usize) {
    kano::log(&format!("Deleting todo {id}"));
    todos.retain(|todo| todo.id != id);
    kano::log(&format!("list now: {todos:?}"));
}
