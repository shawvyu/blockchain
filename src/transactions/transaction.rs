use serde::{Deserialize, Serialize};

use crate::{
    utils::{hash_to_str, serialize},
    Storage, UTXOSet,
};

use super::{tx_input::Txinput, tx_output::Txoutput};

/**
 * 挖矿奖励数
 */
const SUBSIDY: i32 = 10;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    /**
     * 交易id
     */
    id: String,
    /**
     * 交易输入集合
     */
    vin: Vec<Txinput>,
    /**
     * 交易输出集合
     */
    vout: Vec<Txoutput>,
}

impl Transaction {
    /**
     * 挖矿奖励,没有交易输入
     */
    pub fn new_coinbase(to: &str) -> Self {
        let txin = Txinput::default();
        let txout = Txoutput::new(SUBSIDY, to);

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![txin],
            vout: vec![txout],
        };
        tx.set_hash();
        tx
    }

    pub fn new_utxo<T: Storage>(from: &str, to: &str, amount: i32, utxo_set: &UTXOSet<T>) -> Self {
        let (accomulated, valid_outputs) = utxo_set.find_spendable_outputs(from, amount);
        format!("{}",accomulated);
        if accomulated < amount {
            // panic!("Error not enough funds");
            format!("Error not enough funds,accomulated:{},amount:{}",accomulated,amount);
        }

        let mut inputs = vec![];
        for (txid, outputs) in valid_outputs {
            for idx in outputs {
                let input = Txinput::new(txid.clone(), idx.clone(), from);
                inputs.push(input);
            }
        }

        let mut outputs = vec![Txoutput::new(amount, &to)];
        if accomulated > amount {
            outputs.push(Txoutput::new(accomulated-amount, &to));
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: inputs,
            vout: outputs,
        };
        tx.set_hash();

        tx
    }

    fn set_hash(&mut self) {
        if let Ok(tx_ser) = serialize(self) {
            self.id = hash_to_str(&tx_ser)
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_vout(&self) -> &[Txoutput] {
        self.vout.as_slice()
    }

    pub fn get_vin(&self) -> &[Txinput] {
        self.vin.as_slice()
    }
}
