#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate todo_macro;

#[cfg(test)]
mod tests {
    #[test]
    fn todo_future() {
        todo!("This is a todo which must be completed by December 31, 2099", "2099-12-31");
    }
}
