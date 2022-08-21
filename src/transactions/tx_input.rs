use serde::{Serialize, Deserialize};


#[derive(Debug,Serialize,Deserialize,Clone,Default)]
pub struct Txinput {

    /**
     * 前一笔交易的id
     */
    txid:String,
    /**
     * 前一笔交易的序号
     */
    vout:usize,
    /**
     * 交易发起方，后续替换交易发起方公钥
     */
    from_addr:String
}

impl Txinput {
    pub fn new(txid:String,vout:usize,from_addr:&str)->Self{
        Self { txid, vout, from_addr: from_addr.into() }
    }

    pub fn can_unlock_output(&self,address:&str)->bool{
        self.from_addr.eq(address)
    }

    pub fn get_txid(&self)->String {
        self.txid.clone()
    }

    pub fn get_vout(&self)->usize{
        self.vout
    }
    
}