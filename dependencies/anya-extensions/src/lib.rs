#![feature(edition2021)]
pub mod extensions {
    pub fn version() -> &'static str {
        "0.1.0"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        assert_eq!(2 + 2, 4);
    }
} 