use std::{io::{Read, Write, stdin}, net::TcpStream};
use std::io::{BufWriter, BufReader};
const SERVER_IP: &str = "192.168.0.17:3000";

fn main(){
    let stream = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);

    input_loop(writer,reader);
}

fn input_loop(mut writer: BufWriter<TcpStream>, mut reader: BufReader<TcpStream>) {
    println!("Console is active. Please enter your command (type quit/q for exit");
    let mut input = String::new();
    loop {
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        if input.eq("q") || input.eq("quit") {
            break;
        }

        
        let output: String = execute_command(&input, &mut writer, &mut reader);
        input.clear();
        println!("{}", output);
    }
}

fn execute_command(mut input: &str, mut writer: &mut std::io::BufWriter<std::net::TcpStream>, mut reader: &mut BufReader<TcpStream>) -> String {
    let mut output: String = String::new();

    writer.write_all(input.as_bytes()).unwrap();
    //user_session.write_all(input.as_bytes());
    writer.flush().unwrap();
    println!("writed");
    //user_session.read_to_string(&mut output);
    output
}
