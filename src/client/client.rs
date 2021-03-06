use std::io::{BufReader, BufWriter};
use std::{
    io::{stdin, stdout, BufRead, Write},
    net::TcpStream,
};
const SERVER_IP: &str = "192.168.0.17:3000";
const FLAG_BYTE: u8 = 3;

fn main() {
    let socket = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");

    let client = Client::new(socket);
    input_loop(client);
}

fn input_loop(mut client: Client) {
    println!("Console is active. Please enter your command (type quit/q for exit) :");
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

        if (input.starts_with("cd ") && !output.starts_with("cd: "))
            || input.starts_with("popd") && !output.starts_with("popd: ")
        {
            client.wd = output;
        }
        else {
            println!("  {}", output);
        }
        input.clear();
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

        let output =client.execute_command("pwd\n");
        if !output.starts_with("cd: ") {
            client.wd = output;
        }
        
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

        String::from_utf8(output_buf).unwrap()
    }
    fn restore_session_and_try_again(&mut self, input: &str) -> String {
        let new_connection = TcpStream::connect(SERVER_IP).expect("couldn't connect to server");
        let temp_wd = self.wd.clone();
        *self = Client::new(new_connection);

        let output = self.execute_command(format!("cd {}\n", temp_wd).as_str());
        if !output.starts_with("cd: ") {
            self.wd = output;
            self.execute_command(input)
        }
        else {
            "Something went wrong. Directory Changed to \"/\" and command didn't executed. ".to_string()
        }
    }

    fn handle_socket_error(&mut self, err: std::io::Error, input: &str) -> String {
        if let std::io::ErrorKind::BrokenPipe = err.kind() {
            self.restore_session_and_try_again(input)
        } else {
            panic!("{}", err);
        }
    }
}
