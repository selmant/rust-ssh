use std::io::{BufReader, BufWriter};
use std::{
    io::{stdin, stdout, BufRead, Write},
    net::TcpStream,
};
const SERVER_IP: &str = "192.168.0.17:3000";
const FLAG_BYTE: u8 = 3;

fn main() {
    let socket = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");

    let mut client = Client::new(socket);
    input_loop(&mut client);
}

fn input_loop(client: &mut Client) {
    println!("Console is active. Please enter your command (type quit/q for exit :");
    let mut input = String::new();
    loop {
        println!();
        print!("\x1b[0;31m{}> \x1b[0m", client.wd);
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        if input.eq("q\n") || input.eq("quit\n") {
            break;
        }

        let output: String = client.execute_command(input.as_str());
        input.clear();
        println!("  {}", output);
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
        let mut client = Client { wd, reader, writer };
        client.wd = client.execute_command("pwd\n");
        client
    }

    fn execute_command(&mut self, input: &str) -> String {
        let mut byte_vec = input.as_bytes().to_vec();
        byte_vec.pop();
        byte_vec.push(FLAG_BYTE);
        if let Err(e) = self.writer.write_all(&byte_vec) {
            return self.handle_socket_error(e, input);
        }
        self.writer.flush().unwrap();

        let mut output_buf: Vec<u8> = Vec::new();
        match self.reader.read_until(FLAG_BYTE, &mut output_buf) {
            Ok(0) => return self.restore_session_and_try_again(input),
            Err(e) => return self.handle_socket_error(e, input),
            _ => {}
        };
        output_buf.pop();

        let output = String::from_utf8(output_buf).unwrap();
        if (input.starts_with("cd ") && !output.starts_with("cd: no such"))
            || input == "popd" && !output.starts_with("popd: directory")
        {
            self.wd = output.clone();
        }

        output
    }
    fn restore_session_and_try_again(&mut self, input: &str) -> String {
        let new_connection = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");
        *self = Client::new(new_connection);
        self.wd = self.execute_command(format!("cd {}\n", self.wd).as_str());
        self.execute_command(input)
    }

    fn handle_socket_error(&mut self, err: std::io::Error, input: &str) -> String {
        if let std::io::ErrorKind::BrokenPipe = err.kind() {
            self.restore_session_and_try_again(input)
        } else {
            panic!("{}", err);
        }
    }
}
