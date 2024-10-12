pub mod frame {
    use crate::ETHERNETSIZE;


    #[derive(Debug)]
    pub struct Packet {
        pub bytes: Vec<u8>,   
        pub vlan: Option<bool>, 
        pub ipv4: Option<Ipv4Header>,
        pub ipv6: Option<Ipv6Header>,
    }

    #[derive(Debug)]
    pub struct Ipv4Header {
        ihl: u8,
        total_length: u16,
        protocol: u8,
        pub source_ip: [u8; 4],
        pub destination_ip: [u8; 4],
    }

    #[derive(Debug)]
    pub struct Ipv6Header {
        version: u8,
        traffic_class: u8,
        flow_label: u32,
        payload_length: u16,
        next_header: u8,
        hop_limit: u8,
        source_ip: [u8; 16],
        destination_ip: [u8; 16],
}

    #[derive(Debug)]
    pub enum IpType {
        Ipv4,
        Ipv6,
    }

    #[derive(Debug)]
    pub enum PacketError {
        InsufficientBytes,
        UnsupportedEthertype,
        IpVersionNotRead,
        InvalidIpv4Header,
    }

    impl Packet {
    
        pub fn new(bytes: Vec<u8>) -> Result<Packet, PacketError> {

            if bytes.len() < ETHERNETSIZE {
                return Err(PacketError::InsufficientBytes);  
            }
            
            let ethertype_bytes = &bytes[12..14];
            let ethertype = ((ethertype_bytes[0] as u16) << 8) | (ethertype_bytes[1] as u16);

            if ethertype != 0x0800{
                // no vlan parsing
                // vlan present tag 0x8100 not found in my home network so I skip it for now
                return Err(PacketError::UnsupportedEthertype);  
            }
            
            Ok(Packet { 
                bytes: bytes[14..bytes.len()].to_vec(),
                vlan: Some(false),
                ipv4: None,
                ipv6: None,
                
            })
        }

        pub fn extract_ip_header(&mut self) -> Result<(), PacketError> {
            
            let ip_version = (self.bytes[0] & 0b11110000) >> 4;

            match ip_version {
                4 => {self.ipv4 = Some(self.extract_ipv4()?)},
                6 => {},
                _ => {return Err(PacketError::IpVersionNotRead)}

            }
            Ok(())
        }

        fn extract_ipv4(&self) -> Result<Ipv4Header, PacketError> {

            if self.bytes.len() < 20 {
                return Err(PacketError::InvalidIpv4Header);  
            }

            Ok(Ipv4Header{
                ihl: self.bytes[0] & 0x0F,
                total_length: u16::from_be_bytes([self.bytes[2], self.bytes[3]]),
                protocol: self.bytes[9],
                source_ip: <[u8; 4]>::try_from(&self.bytes[12..16]).unwrap(),
                destination_ip: <[u8; 4]>::try_from(&self.bytes[16..20]).unwrap(),
            }
            )   
            
        }

        fn verify_overlapping_8_bit_sequence(first_byte: u8, second_byte: u8, verify: u8) -> bool {
            
            let last_4_bits = first_byte & 0b00001111;
        
            let first_4_bits = second_byte >> 4;
        
            if verify == (last_4_bits << 2) | first_4_bits {
                return true;
            } else {
                return false;
            }
        }
        
    }
}
