use std::io::stdin;
const SERVER_IP: &str = "http://192.168.0.17:3000";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_session = login().expect("Couldn't connect to server.");

    input_loop(&user_session)?;
    Ok(())
}

fn input_loop(user_session : &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Console is active. Please enter your command (type quit/q for exit");
    let mut input = String::new();
    loop {
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");

        if input.eq("q") || input.eq("quit") {
            break;
        }

        let output: String = execute_command(&input, user_session)?;
        println!("{}", output.as_str());
    }
    Ok(())
}

fn login() -> Result<String, ureq::Error> {
    let user_uuid: String = ureq::get(format!("{}/login", SERVER_IP).as_str())
        //.set("Example-Header", "header value")
        //.query("username", "selman")
        .call()?
        .into_string()?;
    Ok(user_uuid)
}

fn execute_command(input: &String, user_session : &str) -> Result<String, ureq::Error> {
    todo!()
}
