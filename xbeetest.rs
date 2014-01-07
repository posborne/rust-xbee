extern mod extra;
use extra::getopts::{optopt,optflag,getopts,Opt};
use std::os;

struct XBeeArguments {
	device : ~str
}

fn print_usage(program: &str, _opts: &[Opt]) {
	println!("Usage: {} [options]:", program);
	println("-d\t\tDevice (e.g. /dev/ttyS0)");
}

fn parse_args() -> Option<~XBeeArguments> {
	let osargs = os::args();
	let program = osargs[0].clone();
	let opts = ~[
		optopt("d"),
		optflag("h"),
		optflag("help"),
	];
	let matches = match getopts(osargs.tail(), opts) {
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};
	if matches.opt_present("h") || matches.opt_present("help") {
		print_usage(program, opts);
		return None;
	}
	let device = matches.opt_str("d");
	return match device {
		Some(device) => { Some(~XBeeArguments { device: device }) }
		None => None
	}
}

fn main() {
	let xbee_args = parse_args();
	println!("device: {:?}", xbee_args);
}
