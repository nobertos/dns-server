use std::io::Read;
use std::{fs::File, net::UdpSocket};

use cdn_dns::dns_message::dns_question::DnsQuestion;
use cdn_dns::{
    dns_message::{DnsMessage, QueryType},
    packet_buffer::PacketBuffer,
};

#[test]
fn dns_message_text_test() {
    let base_path = std::env::current_dir().expect("Failed to get current directory.");
    let path = base_path.join("tests/response_packet.txt");

    let mut file = File::open(path).expect("Failed to open file.");
    let mut buffer = PacketBuffer::new();
    file.read(&mut buffer.buf)
        .expect("Failed to read into buffer.");

    let message =
        DnsMessage::from_buf(&mut buffer).expect("Failed to parse PacketBuffer to DnsMessage.");
    println!("{:#?}", message.header);

    for qst in message.questions {
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

#[test]
fn dns_message_socket_test() {
    let qtype = QueryType::MX;
    let qname = "www.yahoo.com";
    let server = ("8.8.8.8", 53);
    let socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();

    let mut message = DnsMessage::new();

    message.header.id = 6969;
    message.header.questions = 1;
    message.header.recursion_desired = true;
    message
        .questions
        .push(DnsQuestion::new(qname.into(), qtype));

    let buffer = message.into_buf().unwrap();
    socket
        .send_to(&buffer.buf[0..buffer.pos()], server)
        .unwrap();

    let mut recv_buffer = PacketBuffer::new();
    socket.recv_from(&mut recv_buffer.buf).unwrap();
    let recv_message = DnsMessage::from_buf(&mut recv_buffer).unwrap();

    println!("{:#?}", recv_message.header);
    for q in recv_message.questions {
        println!("{:#?}", q);
    }
    for rec in recv_message.answers {
        println!("{:#?}", rec);
    }
    for rec in recv_message.authorities {
        println!("{:#?}", rec);
    }
    for rec in recv_message.resources {
        println!("{:#?}", rec);
    }
}
