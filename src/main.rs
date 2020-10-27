extern crate clap;

use clap::{App, Arg};
use ipr::ip::*;

fn main() -> Result<(), String> {
    let matches = App::new("ipr")
        .version("0.1.0")
        .author("oamam <chapa0106@gmail.com>")
        .about("This is the tool to output IPv4 informations.")
        .arg(
            Arg::with_name("ip")
                .help("IPv4 address")
                .short("i")
                .long("ip")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("net_mask")
                .help("The bit numbers of the network part")
                .short("n")
                .long("net_mask")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let ip = match matches.value_of("ip") {
        Some(ip) => ip,
        None => return Err(String::from("ip is required")),
    };
    let net_mask = match matches.value_of("net_mask") {
        Some(net_mask) => match net_mask.parse::<u8>() {
            Ok(net_mask) => net_mask,
            Err(_) => return Err(String::from("net_mask has to be u8 number")),
        },
        None => return Err(String::from("net_mask is required")),
    };
    let ip_bit = match get_bit(ip) {
        Ok(ip_bit) => ip_bit,
        Err(e) => return Err(e),
    };
    let network_address = match get_network_address(ip, net_mask) {
        Ok(network_address) => network_address,
        Err(e) => return Err(e),
    };
    let subnet_mask = match get_subnet_mask(net_mask) {
        Ok(subnet_mask) => subnet_mask,
        Err(e) => return Err(e),
    };
    let subnet_mask_bit = match get_bit(&subnet_mask) {
        Ok(subnet_mask_bit) => subnet_mask_bit,
        Err(e) => return Err(e),
    };
    let broadcast_address = match get_broadcast_address(ip, net_mask) {
        Ok(broadcast_address) => broadcast_address,
        Err(e) => return Err(e),
    };

    println!("{:<19}{}", "ip", ip);
    println!("{:<19}{}/{}", "cidr", ip, net_mask);
    println!("{:<19}{}", "subnet mask", subnet_mask);
    println!("{:<19}{}", "network address", network_address);
    println!("{:<19}{}", "broadcast address", broadcast_address);
    println!(
        "{:<19}{} ~ {}",
        "ip address range", network_address, broadcast_address
    );
    println!(
        "{:<19}{}",
        "ip address number",
        2u32.pow((32 - net_mask) as u32)
    );
    println!("{:<19}{}", "ip(bit)", ip_bit);
    println!("{:<19}{}", "subnet mask(bit)", subnet_mask_bit);
    Ok(())
}
