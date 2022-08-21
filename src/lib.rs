mod blocks;
mod utils;
mod error;
mod storage;
mod transactions;

pub use blocks::*;
pub use storage::*;
pub use transactions::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){
        let result=1+1;
        assert_eq!(result,2)
    }
}