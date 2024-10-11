struct Packet{
    raw_data: Vec<u8>,
    ipheader: IpHeader, 
}

struct IpHeader {
    ip_type: IpType,
    src: usize,
    dst: usize,


}
enum IpType{
    Ipv4,
    Ipv6
}