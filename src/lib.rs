#![feature(trait_alias)]

pub mod block;
pub mod error;

#[cfg(test)]
mod test;

#[cfg(test)]
mod tests {
    use crate::block::Block;
    use crate::test::Test;

    #[test]
    fn it_works() {
        const RUST_LOG: &str = "debug";
        std::env::set_var(stringify!(RUST_LOG), RUST_LOG);
        env_logger::init();

        let test = Test::new("test");

        {
            let mut block: Block<Test> = Block::load(stringify!(block));
            block.set(test.clone());
        }

        {
            let block: Block<Test> = Block::load(stringify!(block));
            assert_eq!(test, block.get().to_owned());
        }
    }
}
