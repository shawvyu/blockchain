use blockchain::Blockchain;

fn main() {
    tracing_subscriber::fmt().init();

    let mut bc = Blockchain::new();
    bc.mine_block("shawvyu -> Bob 2 btc");
    bc.mine_block("shawvyu -> Bruce 2 btc");

    bc.blocks_info();
}


