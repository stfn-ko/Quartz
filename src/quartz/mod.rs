pub(crate) mod block;

use block::Block;

pub(crate) struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub(crate) fn new() -> Self {
        let mut bc = BlockChain {
            blocks: Vec::with_capacity(2),
        };

        bc.blocks.push(Block::genesis());

        bc
    }

    pub(crate) fn try_add_block(&mut self, block: Block) {
        let last_block = self.blocks.last().expect("there is at least one block");
        if block.validate(last_block) {
            self.blocks.push(block)
        } else {
            eprintln!("quartz :: BlockChain :: error >> `try_add_block` failed at `validate_block` operation")
        }
    }

    pub(crate) fn validate_chain(&self) -> bool {
        for i in 0..self.blocks.len() {
            if i == 0 {
                continue;
            }

            let prv = self.blocks.get(i - 1).expect("has to exist");
            let cur = self.blocks.get(i).expect("has to exist");

            if cur.validate(prv) == false {
                return false;
            }
        }

        true
    }

    pub(crate) fn upgrade_chain(self, remote: BlockChain) -> BlockChain {
        let valid_local: bool = self.validate_chain();
        let valid_remote: bool = remote.validate_chain();

        if valid_local && valid_remote {
            if self.blocks.len() >= remote.blocks.len() {
                self
            } else {
                remote
            }
        } else if !valid_local && valid_remote {
            remote
        } else if valid_local && !valid_remote {
            self
        } else {
            panic!("local and remote chains are both invalid")
        }
    }

    pub(crate) fn derive_from_last(&self, data: &str) -> Block {
        let lb = self.blocks.last().expect("blockchain is not empty");
        Block::new(lb.naked.id.clone() + 1, data, lb.hash)
    }
}
