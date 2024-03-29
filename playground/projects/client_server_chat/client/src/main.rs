use std::{
	io::{self, ErrorKind, Read, Write},
	net::TcpStream,
	sync::mpsc::{self, TryRecvError},
	thread
};

const LOCAL_PORT: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() { thread::sleep(::std::time::Duration::from_millis(100)); }

fn main()
{
	let mut client = TcpStream::connect(LOCAL_PORT).expect("Stream failed to connect");

	client.set_nonblocking(true).expect("Failed to initiate non-blocking");

	let (tx, rx) = mpsc::channel::<String>();

	thread::spawn(move || {
		loop
		{
			let mut buffer = vec![0; MSG_SIZE];

			match client.read_exact(&mut buffer)
			{
				Ok(_) =>
				{
					let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

					println!("Message received {:?}", msg);
				}

				Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),

				Err(_) =>
				{
					println!("Connection with server was severed");
					break;
				}
			}

			match rx.try_recv()
			{
				Ok(msg) =>
				{
					let mut buffer = msg.clone().into_bytes();

					buffer.resize(MSG_SIZE, 0);

					client.write_all(&buffer).expect("Writing to socket failed");

					println!("Message sent {:?}", msg);
				}

				Err(TryRecvError::Empty) => (),

				Err(TryRecvError::Disconnected) => break
			}
		}

		sleep();
	});

	println!("Write a Message:");

	loop
	{
		let mut buffer = String::new();

		io::stdin().read_line(&mut buffer).expect("Reading from stdin failed");

		let msg = buffer.trim().to_string();

		if msg == ":quit" || tx.send(msg).is_err()
		{
			break;
		}
	}

	println!("Good bye!");
}
