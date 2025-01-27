use pnet::datalink;
use pnet::datalink::NetworkInterface;
use std::env;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;



fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            println!("Found a Ipv4 packet!");

            let header = Ipv4Packet::new(ethernet.payload());
            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        println!("TCP packet found!");
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            println!("Found a TCP packet from {}:{} => {}:{}", header.get_source(), tcp.get_source(), header.get_destination(), tcp.get_destination());
                        } else {
                            println!("The TCP packet couldn't be parsed!");
                        }
                    }
                    _ => {
                        println!("Some packet of type other than TCP!");
                    }
                }
            } else {
                println!("The Ipv4 packet couldnt be parsed!");
            }
        }
        _ => {
            println!("Some packet of type other than Ipv4!");
        }
    }
}



fn main() {
    
    println!("--------------------------------------------------");
    println!("-------Made with ❤️  by rahulk1264 aka rykan-------");
    println!("--------------------------------------------------");

    let interface_name = env::args().nth(1).expect("No interface name provided...\n");
    println!("Selected interface name is => {}", interface_name);


    let interfaces: Vec<NetworkInterface> = datalink::interfaces();
    println!("Printing the list of available network interfaces...\n");
    println!("{:?}", interfaces);

    for iface in interfaces.into_iter() {
        println!("name => {}", iface.name);
        println!("description => {}", iface.description);
        println!("index =>{}", iface.index);
        println!("mac => {:?}", iface.mac);
        println!("ips => {:?}", iface.ips);
        println!("flags => {}", iface.flags);
    }

    let interfaces: Vec<NetworkInterface> = datalink::interfaces();

    let interface = interfaces.into_iter().filter(|iface: &NetworkInterface| iface.name == interface_name).next().expect("\nError finding the provided interface...\n");

    println!("\nThe following interface was accepted => \n {:?}", interface);

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("This channel type is not handled...\n"),
        Err(e) => panic!("Error creating a channel => {}\n", e),
    };

    loop {
        match rx.next() {
        Ok(packet) => {
            let packet = EthernetPacket::new(packet).unwrap();
            handle_packet(&packet);
        }
        Err(e) => {
            println!("Error receiving the packets => {}\n", e);
        }
    }

  }
}


