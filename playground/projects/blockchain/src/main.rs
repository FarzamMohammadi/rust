extern crate serde_derive;

use std::{io, io::Write, process};

mod blockchain;
fn main()
{
	let mut miner_address = String::new();
	let mut difficulty = String::new();
	let mut choice = String::new();

	print!("input a miner address: ");
	receive_input_from_user(&mut miner_address);

	print!("Difficulty: ");
	receive_input_from_user(&mut difficulty);

	let difficulty = difficulty.trim().parse::<u32>().expect("Difficulty must be an integer");

	println!("Generating genesis block! ");

	let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), difficulty);

	loop
	{
		choice.clear();

		println!("Menu");
		println!("1) New Transaction");
		println!("2) Mine block");
		println!("3) Change Difficulty");
		println!("4) Change Reward");
		println!("0) Exit");
		print!("Enter your choice: ");

		receive_input_from_user(&mut choice);

		println!("");

		match choice.trim().parse().unwrap()
		{
			0 =>
			{
				println!("Existing!");
				process::exit(0);
			}
			1 =>
			{
				let mut sender = String::new();
				let mut receiver = String::new();
				let mut amount = String::new();

				print!("Enter sender address:");
				receive_input_from_user(&mut sender);

				print!("Enter receiver address: ");
				receive_input_from_user(&mut receiver);

				print!("Enter amount: ");
				receive_input_from_user(&mut amount);

				let new_transaction = chain.new_transaction(
					sender.trim().to_string(),
					receiver.trim().to_string(),
					amount.trim().parse().unwrap()
				);

				match new_transaction
				{
					true => println!("Transaction added successfully"),
					false => println!("Failed to add new transaction")
				}
			}
			2 =>
			{
				println!("Generating new block");

				let res = chain.generate_new_block();

				match res
				{
					true => println!("Block generated successfully"),
					false => println!("Failed to generate new block")
				}
			}
			3 =>
			{
				println!("Current difficulty: {}", chain.get_difficulty());

				let mut new_difficulty = String::new();

				println!("Enter new difficulty: ");
				receive_input_from_user(&mut new_difficulty);

				let res = chain.update_difficulty(new_difficulty.trim().parse().unwrap());

				match res
				{
					true => println!("Updated Difficulty"),
					false => println!("Failed Update Difficulty")
				}
			}
			4 =>
			{
				println!("Current reward: {}", chain.get_reward());

				let mut new_reward = String::new();

				print!("Enter new reward: ");
				receive_input_from_user(&mut new_reward);

				let res = chain.update_reward(new_reward.trim().parse().unwrap());

				match res
				{
					true => println!("Updated reward"),
					false => println!("Failed Update reward")
				}
			}
			_ => println!("Invalid option please retry")
		}
	}
}

fn receive_input_from_user(value: &mut String)
{
	io::stdout().flush();
	io::stdin().read_line(value);
}
