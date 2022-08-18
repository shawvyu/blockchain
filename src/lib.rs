mod blocks;
mod utils;
mod error;

pub use blocks::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){
        let result=1+1;
        assert_eq!(result,2)
    }
}