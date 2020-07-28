use anyhow::Result;

pub trait BenchIRI {
    // fn foo() -> u8
    // where
    //     Self: Sized;
    fn create_from_str(data: &'static str) -> Result<()>;
    // where
    //     Self: Sized;
    // fn create_owned_from_str(&'static str) -> Result<()>;
}
