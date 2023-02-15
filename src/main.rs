use pcap;

const PORT: u16 = 6214;

fn main() {
    let device = pcap::Device::lookup().unwrap().unwrap();
    let devices = pcap::Device::list().unwrap();

    println!("devices: {:?}", devices);

    // let mut cap = pcap::Capture::from_device(device)
    //     .unwrap()
    //     .immediate_mode(true)
    //     .open()
    //     .unwrap();

    // cap.filter(&format!("port {}", PORT), false).unwrap();

    // get a packet and print its bytes
    // println!("{:?}", cap.next_packet());

    // while let Ok(packet) = cap.next_packet() {
    //     println!("received packet! {:?}", packet);
    // }
}

fn establish_connection() {}
