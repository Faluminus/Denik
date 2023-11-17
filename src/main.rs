use std::io;
use std::io::Write;
use std::collections::LinkedList;

fn main(){
    let mut input:String = String::new();
    let cmd = String::from("commands");
    let exit_code = real_main();
    let mut zaznam = String::new();
    loop{
        print!("->");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        zaznam = String::from(match input.trim(){
            "commands" => print_commands(),
            "zavri" => std::process::exit(exit_code),
            "predchozi" => predchozi(),
            "dalsi" => dalsi(),
             _ => ""
        });
    }
}
fn dalsi(){
    String::from("n");
}
fn predchozi(){
    "n";
}
fn real_main() -> i32{
    0
}
fn print_commands(){
    println!("Commands:");
    println!("  Predchozi");
    println!("  Dalsi");
    println!("  Novy");
    println!("  Uloz");
    println!("  Smaz");
    println!("  Zavri");
    "";
}





