use std::{
	io::{ErrorKind, Read, Write},
	net::TcpListener,
	sync::mpsc,
	thread
};

const LOCAL_PORT: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() { thread::sleep(std::time::Duration::from_millis(100)); }

fn main()
{
	let server = TcpListener::bind(LOCAL_PORT).expect("Listener failed to bind");

	server.set_nonblocking(true).expect("Failed to initialize non-blocking");

	let mut clients = vec![];

	let (tx, rx) = mpsc::channel::<String>();

	loop
	{
		if let Ok((mut tcp_stream, socket_address)) = server.accept()
		{
			println!("Client {} connected", socket_address);

			let tx = tx.clone();

			clients.push(tcp_stream.try_clone().expect("Failed to clone client"));

			thread::spawn(move || {
				loop
				{
					let mut buffer = vec![0; MSG_SIZE];

					match tcp_stream.read_exact(&mut buffer)
					{
						Ok(_) =>
						{
							let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

							let msg = String::from_utf8(msg).expect("Invalid utf8 message");

							println!("{}: {:?}", socket_address, msg);

							tx.send(msg).expect("Failed to send message to rx");
						}

						Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),

						Err(_) =>
						{
							println!("Closing connection to: {}", socket_address);
							break;
						}
					}

					sleep();
				}
			});
		}

		if let Ok(msg) = rx.try_recv()
		{
			clients = clients
				.into_iter()
				.filter_map(|mut client| {
					let mut buff = msg.clone().into_bytes();

					buff.resize(MSG_SIZE, 0);

					client.write_all(&buff).map(|_| client).ok()
				})
				.collect::<Vec<_>>();
		}

		sleep();
	}
}
