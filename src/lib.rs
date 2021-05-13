#![feature(trait_alias)]

pub mod block;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::block::Block;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
    struct Test(String);

    impl Test {
        pub fn new(s: &str) -> Self {
            Self { 0: s.to_owned() }
        } 
    }

    #[test]
    fn it_works() {
        const RUST_LOG: &str = "debug";
        std::env::set_var(stringify!(RUST_LOG), RUST_LOG);
        env_logger::init();

        let test_str = "test";

        {
            let mut test: Block<Test> = Block::load(stringify!(test));
            test.set(Test::new(test_str));
        }

        {
            let test: Block<Test> = Block::load(stringify!(test));
            assert_eq!(test_str, test.get().0);
        }
    }
}
