use std::{
    io::{stdin, BufRead, Write},
    net::TcpStream,
};
use std::{
    io::{BufReader, BufWriter}
};
const SERVER_IP: &str = "192.168.0.17:3000";
const FLAG_BYTE: u8 = 3;

fn main() {
    let socket = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");

    let mut client = Client::new(socket);
    input_loop(&mut client);
}

fn input_loop(client: &mut Client) {
    println!("Console is active. Please enter your command (type quit/q for exit");
    let mut input = String::new();
    loop {
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        if input.eq("q") || input.eq("quit") {
            break;
        }

        let output: String = client.execute_command(input.as_str());
        input.clear();
        println!("{}", output);
    }
}

struct Client {
    wd: String,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Client {
    fn new(socket: TcpStream) -> Client {
        let wd = "/".to_string();
        let socket_clone = socket.try_clone().unwrap();
        let reader = BufReader::new(socket);
        let writer = BufWriter::new(socket_clone);
        Client { wd, reader, writer }
    }

    fn execute_command(&mut self, input: &str) -> String {
        let mut byte_vec = input.as_bytes().to_vec();
        byte_vec.pop();
        byte_vec.push(FLAG_BYTE);
        if let Err(e) = self.writer.write_all(&byte_vec) {
            match e.kind() {
                std::io::ErrorKind::BrokenPipe => return self.restore_session_and_try_again(input),
                _ => panic!("{}", e),
            }
        }
        self.writer.flush().unwrap();
        println!("writed");

        let mut output_buf: Vec<u8> = Vec::new();
        let n = match self.reader.read_until(FLAG_BYTE, &mut output_buf) {
            Ok(n) => n,
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => return self.restore_session_and_try_again(input),
                _ => panic!("{}", e),
            },
        };

        if n == 0 {
            return self.restore_session_and_try_again(input);
        };
        output_buf.pop();

        let output =String::from_utf8(output_buf).unwrap();
        if (input.starts_with("cd ") && !output.starts_with("cd: no such")) || input == "popd" && !output.starts_with("popd: directory"){
            self.wd = output.clone();
        }

        output
        
    }
    fn restore_session_and_try_again(&mut self, input: &str) -> String {
        let new_connection = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");
        *self = Client::new(new_connection);
        self.execute_command(format!("cd {}", self.wd).as_str());
        self.execute_command(input)
    }
}