#[macro_use]
extern crate clap;
extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use hyper::Client;
use hyper::status::StatusCode::Ok;
use rustc_serialize::json;
use clap::{App, Arg};

#[derive(RustcDecodable, Debug)]
pub struct BucketInfo {
	name: String,
	nodes: Vec<NodeInfo>
}

#[derive(RustcDecodable, Debug)]
pub struct NodeInfo {
	hostname: String
}

fn main() {
	let args = App::new("sdkchecker")
		.author("Couchbase, Inc.")
		.version(&crate_version!()[..])
		.about("This tool performs checks to see if this host is SDK ready.")
		.arg(Arg::with_name("host")
			.help("the hostname of a node in the cluster")
			.takes_value(true)
			.short("H")
			.long("host")
			.required(false))
		.arg(Arg::with_name("bucket")
			.help("the name of the bucket")
			.takes_value(true)
			.short("b")
			.long("bucket")
			.required(false))
		.get_matches();

	let hostname = match args.value_of("host") {
		Some(h) => h,
		None => "localhost"
	};

	let bucket = match args.value_of("bucket") {
		Some(b) => b,
		None => "default"
	};

	let bucket_info = grab_bucket_info(hostname, bucket);
    println!("{:?}", bucket_info);
}

fn grab_bucket_info(hostname: &str, bucket: &str) -> BucketInfo {
	let mut client = Client::new();

	let mut response = client.get(&format!("http://{}:8091/pools/default/buckets/{}", hostname, bucket)).send().unwrap();
	match response.status {
		Ok => println!("[debug] HTTP 200 Response Returned from 8091, moving forward."),
		_ => println!("error")
	}

	let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let parsed: BucketInfo = json::decode(&body).unwrap();
    parsed
}
