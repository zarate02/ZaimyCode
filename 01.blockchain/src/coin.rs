use sha2::{Sha256, Digest};
use std::time::{Instant};
use std::collections::LinkedList;

#[derive(Clone, Debug)]
pub struct User {
    name: String,
    cnt:i32,
    block_list : LinkedList<Block>
}
impl User {

    //계정생성
    pub const fn from(pname:String) -> User {
        User {
            name : pname,
            cnt : 0,
            block_list : LinkedList::new()
        }
    }

    pub fn get_cnt(&self) -> i32 {
        return self.cnt;
    }

    //전송
    pub fn send(&self, target: &User, p:i32) -> Transaction{
        return Transaction{
            sender: self.name.clone(),
            receiver: target.name.clone(),
            value: p
        };
    }

    //블럭정보받음
    pub fn get_block(&mut self, block:Block){
        if self.name == block.tr.sender {
            self.cnt -= block.tr.value;
        }
        else if self.name == block.tr.receiver{
            self.cnt += block.tr.value;
        }

        self.block_list.push_back(block.clone());
    }

    //채굴
    pub fn mine(&mut self, tr: Transaction) -> Block{

        let hpb:String;
        let mut self_bk_list = self.block_list.clone();

        match self_bk_list.pop_back()
        {
            None => hpb = "".to_string(),
            Some(hash_prev_block) => hpb = hash_prev_block.n_block_header.hash_merkle_root
        }

        let blkh = BlockHeader{
            version: 1,
            hash_prev_block:hpb,
            hash_merkle_root: tr.gethash(),
            time: Instant::now().elapsed().as_millis(),
            bits: 2,
            nonce: 0
        };

        let mut blk = Block{
            n_block_header: blkh,
            tr : tr.clone()
        };

        let mut blkhash = blk.n_block_header.gethash();
        let mut az = String::new();
        for _ in 0..blk.n_block_header.bits
        {
            az = format!("{}0", az);
        }

        while !blkhash.starts_with(&az)
        {
             blk.n_block_header.nonce += 1;
             blkhash = blk.n_block_header.gethash();
        }

        //채굴자몫
        self.cnt += 1;

        println!("ok : {}, {}", blk.n_block_header.nonce, blk.n_block_header.gethash()  );

        return blk;
    }
}

trait Hash {
    fn gethash(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Block{
    n_block_header : BlockHeader,
    tr: Transaction
}

#[derive(Debug, Clone)]
struct BlockHeader{
    version: i32,
    hash_prev_block: String,
    hash_merkle_root: String,
    time: u128,
    bits: usize,
    nonce: u32,
}
impl Hash for BlockHeader {
    fn gethash(&self) -> String{
        let mut hasher = Sha256::new();
        hasher.update(self.version.to_be_bytes());
        hasher.update(self.hash_prev_block.as_bytes());
        hasher.update(self.hash_merkle_root.as_bytes());
        hasher.update(self.time.to_be_bytes());
        hasher.update(self.bits.to_be_bytes());
        hasher.update(self.nonce.to_be_bytes());
        let result = hasher.finalize();
        return format!("{:x}", result);
    }
}

#[derive(Debug, Clone)]
pub struct Transaction{
    sender:String,
    receiver:String,
    value:i32,
}
impl Hash for Transaction{
    fn gethash(&self) -> String{
        let mut hasher = Sha256::new();
        hasher.update(self.value.to_be_bytes());
        hasher.update(self.receiver.as_bytes());
        hasher.update(self.sender.as_bytes());
        let result = hasher.finalize();
        return format!("{:x}",result);
    }
}