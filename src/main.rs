use etherparse;
use pcap;

const PORT: u16 = 50573;

struct Command {
    raw: String,
    arg: String,
}

impl Command {
    pub fn new(raw: &str, arg: &str) -> Self {
        return Command {
            raw: raw.to_string(),
            arg: arg.to_string(),
        };
    }
}

// fn get_os_command() {
//     let os = std::env::consts::OS;

//     let lookup = match os {
//        "macos" => Command::new(, arg)
//     }
// }

fn main() {
    let device = pcap::Device::lookup().unwrap().unwrap();

    let get_pid = std::process::Command::new("ps")
        .args(["-A", "|", "grep Spotify", "|", "awk 'print ${1}'"])
        .output()
        .expect("failed command");

    println!("pid: {:?}", get_pid);

    let mut cap = pcap::Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    cap.filter(&format!("port {}", PORT), false).unwrap();

    loop {
        println!("checking packet");
        let packet = cap.next_packet();
        if packet.is_err() {
            println!("error in packet");
            break;
        };
        let packet = packet.unwrap();
        decode_data(packet);
    }
}

fn decode_data(packet: pcap::Packet) {
    let headers = etherparse::PacketHeaders::from_ethernet_slice(packet.data);
    let headers = headers.unwrap();
    println!("headers: {:?}", headers);

    let payload = String::from_utf8_lossy(&packet.data[headers.payload[1] as usize..]);

    println!("headers: {:?}", payload);
}
