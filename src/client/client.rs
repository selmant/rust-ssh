use std::io::{BufReader, BufWriter};
use std::{
    io::{stdin, BufRead, Write},
    net::TcpStream,
};
const SERVER_IP: &str = "192.168.0.17:3000";
const FLAG_BYTE: u8 = 3;

fn main() {
    let stream = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");
    let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);

    input_loop(&mut writer, &mut reader);
}

fn input_loop(mut writer: &mut BufWriter<TcpStream>, mut reader: &mut BufReader<TcpStream>) {
    println!("Console is active. Please enter your command (type quit/q for exit");
    let mut input = String::new();
    loop {
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        if input.eq("q") || input.eq("quit") {
            break;
        }

        let output: String =
            execute_command::<BufReader<TcpStream>>(&input, &mut writer, &mut reader);
        input.clear();
        println!("{}", output);
    }
}

fn execute_command<R: BufRead>(
    input: &str,
    writer: &mut BufWriter<TcpStream>,
    reader: &mut R,
) -> String {
    let mut byte_vec = input.as_bytes().to_vec();
    byte_vec.pop();
    byte_vec.push(FLAG_BYTE);
    writer.write_all(&byte_vec).unwrap();
    writer.flush().unwrap();
    println!("writed");

    let mut output_buf: Vec<u8> = Vec::new();
    reader.read_until(FLAG_BYTE, &mut output_buf).unwrap();
    output_buf.pop();
    println!("readed");

    //user_session.read_to_string(&mut output);
    String::from_utf8(output_buf).unwrap()
}


struct Client {
    wd: String,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Client {
    fn new(wd: String, reader: BufReader<TcpStream>, writer: BufWriter<TcpStream>) -> Client {
        Client { wd, reader, writer }
    }
}