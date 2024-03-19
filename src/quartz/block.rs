use chrono::Utc;
use serde_json::json;
use sha2::{Digest, Sha256};

type Hash = [u8; 32];

trait Convert {
    fn to_hex(&self) -> String;
    fn to_bin(&self) -> String;
}

trait Serialize {
    fn serialize(&self) -> String;
}

impl Convert for Hash {
    fn to_hex(&self) -> String {
        let mut res: String = String::default();
        for c in self {
            res.push_str(&format!("{:x}", c));
        }

        res
    }

    fn to_bin(&self) -> String {
        let mut res: String = String::default();
        for c in self {
            res.push_str(&format!("{:b}", c));
        }

        res
    }
}

pub(crate) struct NakedBlock {
    pub id: u64,
    pub data: String,
    pub timestamp: i64,
    pub p_hash: Hash,
}

impl NakedBlock {
    pub(crate) fn new(id: u64, data: &str, p_hash: Hash) -> Self {
        NakedBlock {
            id,
            data: data.to_string(),
            timestamp: Utc::now().timestamp(),
            p_hash,
        }
    }
}

impl Serialize for NakedBlock {
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Debug for NakedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NakedBlock")
            .field("id", &self.id)
            .field("data", &self.data)
            .field("timestamp", &self.timestamp)
            .field("p_hash", &self.p_hash.to_hex())
            .finish()
    }
}

pub(crate) struct Block {
    pub hash: Hash,
    pub nonce: u64,
    pub naked: NakedBlock,
}

impl Block {
    const DIFFICULTY_PREFIX: &str = "00";

    pub(crate) fn new(id: u64, data: &str, p_hash: Hash) -> Self {
        let nb = NakedBlock::new(id, data, p_hash);
        Self::mine_block(nb)
    }

    pub(crate) fn validate(&self, prev_block: &Block) -> bool {
        self.validate_prev_hash(&prev_block.hash)
            && self.validate_id(prev_block.naked.id)
            && self.validate_hash()
            && self.validate_dif()
    }

    pub fn genesis() -> Self {
        Self::mine_block(NakedBlock::new(0, "genesis", Self::rand_hash()))
    }

    fn mine_block(nb: NakedBlock) -> Self {
        let mut nonce: u64 = 0;

        loop {
            let hash = Self::gen_hash(&nb, &nonce);
            if hash.to_bin().starts_with(Self::DIFFICULTY_PREFIX) {
                return Block {
                    hash: hash,
                    nonce,
                    naked: nb,
                };
            }

            nonce += 1;
        }
    }

    fn gen_hash(naked: &NakedBlock, nonce: &u64) -> Hash {
        let json_data = json!({
            "id": naked.id,
            "nonce": nonce,
            "naked": naked.data,
            "timestamp": naked.timestamp,
            "p_hash": naked.p_hash,
        });

        let mut hasher = Sha256::new();
        hasher.update(json_data.to_string().as_bytes());
        hasher
            .finalize()
            .as_slice()
            .to_owned()
            .try_into()
            .expect("Hash should always be 32 bytes")
    }

    fn rand_hash() -> Hash {
        let mut nonce: u64 = 0;
        let rnum: u64 = rand::random::<u64>();

        loop {
            let json_data = json!({
                "nonce": nonce,
                "rand": rnum,
            });

            let mut hasher = Sha256::new();
            hasher.update(json_data.to_string().as_bytes());
            let hash: Hash = hasher
                .finalize()
                .as_slice()
                .to_owned()
                .try_into()
                .expect("Hash should always be 32 bytes");

            if hash.to_bin().starts_with(Self::DIFFICULTY_PREFIX) {
                return hash;
            }

            nonce += 1;
        }
    }

    fn validate_prev_hash(&self, prev_hash: &Hash) -> bool {
        if &self.naked.p_hash != prev_hash {
            eprintln!(
                "quartz :: block :: validation error >> block with id `{}` has wrong previous hash",
                self.naked.id
            );

            return false;
        }

        true
    }

    fn validate_id(&self, prev_id: u64) -> bool {
        if self.naked.id != prev_id + 1 {
            eprintln!(
                "quartz :: block :: validation error >> block with id `{}` is not the next block after the latest: `{}`",
                self.naked.id,
                prev_id,
            );

            return false;
        }

        true
    }

    fn validate_dif(&self) -> bool {
        if self.hash.to_hex().starts_with(Self::DIFFICULTY_PREFIX) == false {
            eprintln!(
                "quartz :: block :: validation error >> block with id `{}` has invalid difficulty",
                self.naked.id
            );

            return false;
        }

        true
    }

    fn validate_hash(&self) -> bool {
        if Self::gen_hash(&self.naked, &self.nonce) != self.hash {
            eprintln!(
                "quartz :: block :: validation error >> block with id `{}` has invalid hash",
                self.naked.id
            );

            return false;
        }

        true
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("hash", &self.hash.to_hex())
            .field("nonce", &self.nonce)
            .field("naked", &self.naked)
            .finish()
    }
}
