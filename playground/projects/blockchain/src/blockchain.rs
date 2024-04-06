use chrono::prelude::*;
use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Serialize, Clone, Debug)]
struct Transaction
{
	sender:   String,
	receiver: String,
	amount:   f32
}

#[derive(Serialize, Debug)]
pub struct BlockHeader
{
	timestamp:  i64,
	nonce:      u32,
	prev_hash:  String,
	merkle:     String,
	difficulty: u32
}

#[derive(Serialize, Debug)]
pub struct Block
{
	header:       BlockHeader,
	count:        u32,
	transactions: Vec<Transaction>
}

pub struct Chain
{
	blocks:               Vec<Block>,
	current_transactions: Vec<Transaction>,
	difficulty:           u32,
	miner_address:        String,
	reward:               f32
}

impl Chain
{
	pub fn new(miner_address: String, difficulty: u32) -> Chain
	{
		let mut chain = Chain {
			blocks: Vec::new(),
			current_transactions: Vec::new(),
			difficulty,
			miner_address: miner_address,
			reward: 100.0
		};

		chain.generate_new_block();
		chain
	}

	pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool
	{
		self.current_transactions.push(Transaction {
			sender,
			receiver,
			amount
		});

		true
	}

	pub fn last_hash(&self) -> String
	{
		let block = match self.blocks.last()
		{
			Some(block) => block,
			None => return String::from_utf8(vec![48; 64]).unwrap()
		};
		Chain::hash(&block.header)
	}

	pub fn update_difficulty(&mut self, difficulty: u32) -> bool
	{
		self.difficulty = difficulty;
		true
	}

	pub fn get_difficulty(&self) -> u32 { self.difficulty }

	pub fn update_reward(&mut self, reward: f32) -> bool
	{
		self.reward = reward;
		true
	}

	pub fn get_reward(&self) -> f32 { self.reward }

	pub fn generate_new_block(&mut self) -> bool
	{
		let block_header = BlockHeader {
			timestamp:  Utc::now().timestamp_millis(),
			nonce:      0,
			prev_hash:  self.last_hash(),
			merkle:     String::new(),
			difficulty: self.difficulty
		};

		let reward_transaction = Transaction {
			sender:   String::from("Root"),
			receiver: self.miner_address.clone(),
			amount:   self.reward
		};

		let mut block = Block {
			header:       block_header,
			count:        0,
			transactions: vec![]
		};

		block.transactions.push(reward_transaction);
		block.transactions.append(&mut self.current_transactions);

		block.count = block.transactions.len() as u32;
		block.header.merkle = Chain::get_merkle(block.transactions.clone());

		Chain::proof_of_work(&mut block.header);

		println!("{:#?}", &block);
		self.blocks.push(block);

		true
	}

	fn get_merkle(current_transactions: Vec<Transaction>) -> String
	{
		let mut merkle = Vec::new();

		for transaction in &current_transactions
		{
			let hash = Chain::hash(transaction);
			merkle.push(hash);
		}

		if merkle.len() % 2 == 1
		{
			let last_hash = merkle.last().cloned().unwrap();
			merkle.push(last_hash);
		}

		while merkle.len() > 1
		{
			let mut hash_one = merkle.remove(0);
			let mut hash_two = merkle.remove(0);

			hash_one.push_str(&mut hash_two);

			let new_hash = Chain::hash(&hash_one);
			merkle.push(new_hash)
		}

		merkle.pop().unwrap()
	}

	pub fn proof_of_work(header: &mut BlockHeader)
	{
		loop
		{
			let hash = Chain::hash(header);
			let slice = &hash[..header.difficulty as usize];

			match slice.parse::<u32>()
			{
				Ok(value) =>
				{
					if value == 0
					{
						println!("Block hash: {}", hash);
						break;
					}

					header.nonce += 1;
				}
				Err(_) =>
				{
					header.nonce += 1;
					continue;
				}
			}
		}
	}

	pub fn hash<T: serde::Serialize>(item: &T) -> String
	{
		let input = serde_json::to_string(&item).unwrap();
		let mut hasher = Sha256::new();
		hasher.update(input.as_bytes());
		let res = hasher.finalize();
		let vec_result = res.to_vec();

		Chain::bytes_to_hex_string(vec_result.as_slice())
	}

	pub fn bytes_to_hex_string(hex_vec: &[u8]) -> String
	{
		let mut string = String::new();

		for bytes in hex_vec
		{
			write!(&mut string, "{:x}", bytes).expect("unable to write");
		}

		string
	}
}
