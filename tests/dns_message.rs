use std::fs::File;
use std::io::Read;

use cdn_dns::{dns_message::DnsMessage, packet_buffer::PacketBuffer};

#[test]
fn dns_message_test() {
    let base_path = std::env::current_dir().expect("Failed to get current directory.");
    let path = base_path.join("tests/response_packet.txt");

    let mut file = File::open(path).expect("Failed to open file.");
    let mut buffer = PacketBuffer::new();
    file.read(&mut buffer.buf)
        .expect("Failed to read into buffer.");

    let message =
        DnsMessage::from_buf(&mut buffer).expect("Failed to parse PacketBuffer to DnsMessage.");
    println!("{:#?}", message.header);

    for qst in message.questons {
        println!("{:#?}", qst);
    }

    for rec in message.answers {
        println!("{:#?}", rec);
    }

    for rec in message.authorities {
        println!("{:#?}", rec);
    }

    for rec in message.resources {
        println!("{:#?}", rec);
    }
}
