pub mod frame {
    use crate::ETHERNETSIZE;


    #[derive(Debug)]
    pub struct Packet {
        pub bytes: Vec<u8>,   
        pub vlan: Option<bool>, 
        pub ip_header: Option<IpHeader>,
    }


    #[derive(Debug)]
    pub struct StartEnd {
        pub start: usize,
        pub end: usize
    }

    #[derive(Debug)]
    pub struct IpHeader {
        pub complete: StartEnd,
        pub src_ip: StartEnd,
        pub dst_ip: StartEnd,
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

            if ethertype != 0x0800 && ethertype != 0x86DD {
                // no vlan parsing only ipv4 and ipv6 for now
                // vlan present tag 0x8100 not found in my home network so 
                return Err(PacketError::UnsupportedEthertype);  
            }
            
            Ok(Packet { 
                bytes: bytes[14..bytes.len()].to_vec(),
                vlan: Some(false),
                ip_header: None,
            })
        }

   
        pub fn extract_ip_header(&mut self) -> Result<(), PacketError> {
            
            let ip_version = (self.bytes[0] & 0b11110000) >> 4;

            match ip_version {
                4 => {self.zero_copy_ip_header(4)?},
                6 => {},
                _ => {return Err(PacketError::IpVersionNotRead)}

            }
            Ok(())
        }

        fn zero_copy_ip_header(&mut self, version: usize) -> Result<(), PacketError> {
            // ipv4 for now
            
            if version == 4 {
                let ihl = self.bytes[0] & 0b00001111;

                self.ip_header = Some(IpHeader{
                    complete: StartEnd { start: 0, end: (ihl * 4 - 1) as usize },
                    src_ip: StartEnd { start: 12, end: 15 },
                    dst_ip: StartEnd { start: 16, end: 19 },
                });
            } else {
                // parse ipv6
            };

            Ok(())
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
