use std::{
	env, fmt,
	net::{IpAddr, SocketAddr, TcpStream},
	process,
	str::FromStr,
	sync::mpsc::{channel, Sender},
	thread,
	time::Duration
};

const MAX: u16 = 65535;

enum ArgumentError
{
	HelpRequested,
	TooManyArguments,
	NotEnoughArguments,
	InvalidSyntax,
	InvalidIpAddress,
	InvalidThreadsValue
}

impl fmt::Display for ArgumentError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			ArgumentError::HelpRequested => write!(f, "Help requested"),
			ArgumentError::TooManyArguments => write!(f, "Too many arguments"),
			ArgumentError::NotEnoughArguments => write!(f, "Not enough arguments"),
			ArgumentError::InvalidSyntax => write!(f, "Invalid syntax"),
			ArgumentError::InvalidIpAddress => write!(f, "Invalid IP address"),
			ArgumentError::InvalidThreadsValue => write!(f, "Invalid threads value")
		}
	}
}

struct Arguments
{
	ip_address: IpAddr,
	threads:    u16
}

// An 'impl' block in Rust is used to define implementations of methods and associated functions for a particular type.
// This block can be used for implementing methods for struct and enum definitions, for trait implementations, or for
// providing associated functions that don’t take self as a parameter.
impl Arguments
{
	fn new(args: &[String]) -> Result<Arguments, ArgumentError>
	{
		if args.len() < 2
		{
			return Err(ArgumentError::NotEnoughArguments);
		}
		else if args.len() > 4
		{
			return Err(ArgumentError::TooManyArguments);
		}

		let flag = args[1].clone();

		match IpAddr::from_str(&flag)
		{
			Ok(ip_address) =>
			{
				return Ok(Arguments {
					ip_address,
					threads: 50
				})
			}
			Err(_) =>
			{
				if flag.contains("-h") || flag.contains("-help")
				{
					if args.len() == 2
					{
						// The ! in println! indicates that println is a macro in Rust, not a regular function.
						// Macros allow for code generation or metaprogramming at compile time, offering more
						// flexibility than functions, such as accepting a variable number of arguments.
						println!(
							"Usage:
                            \r\n-j to select number of threads. 
                            \r\n-h or -help to show this help message"
						);
						return Err(ArgumentError::HelpRequested);
					}
					else
					{
						return Err(ArgumentError::TooManyArguments);
					}
				}
				else if flag.contains("-j")
				{
					let threads = match args[2].parse::<u16>()
					{
						Ok(parsed_vale) => parsed_vale,
						Err(_) => return Err(ArgumentError::InvalidThreadsValue)
					};

					let ip_address = match IpAddr::from_str(&args[3])
					{
						Ok(parsed_vale) => parsed_vale,
						Err(_) => return Err(ArgumentError::InvalidIpAddress)
					};

					return Ok(Arguments { ip_address, threads });
				}
				else
				{
					return Err(ArgumentError::InvalidSyntax);
				}
			}
		}
	}
}

fn main()
{
	let args: Vec<String> = env::args().collect();

	let program = args[0].clone();

	let arguments = Arguments::new(&args).unwrap_or_else(|error| {
		match error
		{
			ArgumentError::HelpRequested => process::exit(0),
			_ =>
			{
				eprintln!("{} problem parsing arguments: {}", program, error);
				process::exit(0);
			}
		}
	});

	let number_of_threads = arguments.threads;
	let ip_address = arguments.ip_address;

	// Create a channel for communication between threads.
	// 'port_sender' is used to send open port numbers from scanning threads to the main thread.
	// 'port_receiver' is used by the main thread to receive open port numbers from scanning threads.
	let (port_sender, port_receiver) = channel();

	for thread_index in 0..number_of_threads
	{
		let thread_port_sender = port_sender.clone();

		thread::spawn(move || {
			scan(thread_port_sender, thread_index, ip_address, number_of_threads);
		});
	}

	let mut open_ports = vec![];

	// The original sender is no longer needed at this point, so it can be dropped
	// to close the channel and allow the receiving loop to terminate.
	drop(port_sender);

	for port in port_receiver
	{
		open_ports.push(port);
	}

	println!("");

	open_ports.sort();

	for port in open_ports
	{
		println!("{} is open", port);
	}
}

fn scan(tx: Sender<u16>, start_port: u16, ip_address: IpAddr, number_of_threads: u16)
{
	let mut port = start_port + 1;
	loop
	{
		let socket_address = SocketAddr::new(ip_address, port);
		let timeout = Duration::from_nanos(1);

		match TcpStream::connect_timeout(&socket_address, timeout)
		{
			Ok(_) =>
			{
				print!(".");

				if tx.send(port).is_err()
				{
					break;
				}
			}
			Err(_) =>
			{
				// println!("Closed {} is unavailable", port);
			}
		}

		if MAX - port <= number_of_threads
		{
			break;
		}

		port += number_of_threads;
	}
}
