extern crate pcap;

use crossbeam::channel::{unbounded, Receiver};
use pcap::{Device, Capture};
use std::thread;

use personal_dpi::dpi::process_packet;

fn main() {
    // I use a single consumer and multiple receiver threading setup
    
    // set up working threads
    let (tx, rx) = unbounded();
    let num_threads = 4;
    for _ in 0..num_threads {
        let rx:  Receiver<Vec<u8>> = rx.clone();
        thread::spawn(move || {
            while let Ok(packet) = rx.recv() {
                process_packet(packet);
            }
        });
    }
    
    // auto lookup for the interface to listen to
    let device = Device::lookup().expect("No device found").unwrap();
    println!("Using device: {}", device.name);

    // set up listener to interface
    let mut capture = Capture::from_device(device)
        .unwrap()
        .promisc(true) // Set promiscuous mode to capture all packets
        .open()
        .unwrap();

    println!("Listening for packets...");

    // this loop listends continously on the interface and send packets to the worker threads.
    while let Ok(packet) = capture.next_packet() {
        // create a raw byte vector so we can work on the bytes directly 
        let packet_vec: Vec<u8> = packet.data.to_vec();
        let _ = tx.send(packet_vec);        
    }
}
