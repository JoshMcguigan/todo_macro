#![feature(proc_macro_hygiene)]

#[cfg(test)]
mod tests {
    use todo_macro::todo;

    #[test]
    fn todo_future() {
        todo!("This is a todo which must be completed by December 31, 2099", "2099-12-31");
    }
}
