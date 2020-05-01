//! A Routerify middleware which enables CORS.
//!
//! # Examples
//!
//! ```
//! use routerify_cors;
//!
//! # fn run() {
//! println!("{}", routerify_cors::add(2, 3));
//! # }
//! # run();
//! ```

/// This function adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
