use crate::packet::frame::Packet;

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

    if let Some(ip_header) = &packet.ip_header{
        let source_ip = ip_header.src_ip.start;

        println!("Source IP: {}.{}.{}.{}", 
            packet.bytes[source_ip], 
            packet.bytes[source_ip + 1],
            packet.bytes[source_ip + 2 ],
            packet.bytes[source_ip+ 3],
        );
    } else {
        println!("No IPv4 header found.");
    }
}