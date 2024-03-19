mod quartz;
use quartz::BlockChain;
fn main() {
    let mut qrtz = BlockChain::new();

    qrtz.try_add_block(qrtz.derive_from_last("foo"));

    for q in qrtz.blocks {
        println!("{:#?}", q)
    }
}
