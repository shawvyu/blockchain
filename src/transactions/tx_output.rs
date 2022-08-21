use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Txoutput {
    /**
     * 交易值
     */
    value: i32,
    /**
     * 交易接收方，后续替换为交易接收方的公钥
     */
    to_addr: String,
}

impl Txoutput {
    pub fn new(value: i32, to_addr: &str) -> Self {
        Self { value, to_addr: to_addr.into() }
    }

    pub fn is_locked(&self,address:&str)->bool{
        self.to_addr.eq(address)
    }

    pub fn get_value(&self)->i32{
        self.value
    }
}
