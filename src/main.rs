use std::io;
use std::io::Write;
use std::collections::LinkedList;

struct Date{
    year:i32,
    month: i32,
    day: i32
}
impl Date{
    fn new(year:i32,month:i32,day: i32) -> Date{
        Date{
            year,
            month,
            day
        }
    }
    fn format_date(&mut self){
        let month_arr:[i32;12] = [31,28,31,30,31,30,31,31,30,31,30,31];
        if self.month < 1 || self.month > 12{
            panic!();
        }

        if self.day != month_arr[(self.month - 1) as usize]{
            panic!();
        }
    }
}
struct Data{
    datum: Date,
    value: String,
}
impl Data{
    fn new() -> Data{
        let datum:Date = Date::new(0,0,0);
        let value = String::from("");
        Data{
            datum,
            value
        }
    }
    fn get_date(&mut self,year:i32,month:i32,day:i32){
        self.datum = Date::new(year,month,day);
        self.datum.format_date();
    }
    fn get_value(&mut self,text:String){
        self.value = text;
    }
}
fn main(){
    let mut input:String = String::new();
    let exit_code = 0;
    loop{
        print_commands();
        print!("->");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.to_lowercase().trim(){
            "zavri" => std::process::exit(exit_code),
            "predchozi" => predchozi(),
            "dalsi" => dalsi(),
            "novy" => novy(),
            "uloz" => uloz(),
            "smaz" => smaz(),
             _ => ()
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

fn dalsi(){

}
fn predchozi(){

}
fn novy(){

}
fn uloz(){

}
fn smaz(){

}
fn print_commands(){
    println!("----------------");
    println!("Commands:");
    println!("  Predchozi");
    println!("  Dalsi");
    println!("  Novy");
    println!("  Uloz");
    println!("  Smaz");
    println!("  Zavri");
    println!("----------------");

}





