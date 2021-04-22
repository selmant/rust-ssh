mod commands;

fn main() {
    let cp = commands::Commands::new("cp asd assd -r");
    let mv = commands::Commands::new("mv asd assd -p");
    println!("{:?}\n{:?}",cp,mv);
}
