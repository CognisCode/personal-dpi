use crate::packet::{self, frame::Packet};

pub fn process_packet(bytes: Vec<u8>) {
    let result = Packet::new(bytes);

    let mut packet = match result {
        Ok(value) => { value }
        Err(e) => {
            eprintln!("Error encountered: {:?}", e);
            return;
        }
    };
    
    packet.extract_ip_header().unwrap();

    if let Some(ipv4_header) = &packet.ipv4 {
        println!("Source IP: {}.{}.{}.{}", 
            ipv4_header.source_ip[0], 
            ipv4_header.source_ip[1], 
            ipv4_header.source_ip[2], 
            ipv4_header.source_ip[3]
        );
    } else {
        println!("No IPv4 header found.");
    }
}