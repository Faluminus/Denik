use std::io;
use std::io::Write;
use std::collections::LinkedList;



struct Date{
    year:i32,
    month: i32,
    day: i32
}

impl Date{
    fn new() -> Self{
        Date {
            year: 0,
            month: 0,
            day: 0,
        }
    }
    fn change(mut self,_year:i32,_month:i32,_day: i32 ){
        self.year = _year;
        self.month =_month;
        self.day = _day;
    }
    fn format_date(self){
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
        let datum:Date = Date::new();
        let value = String::from("");
        Data{
            datum,
            value
        }
    }
    fn get_date(mut self, year:i32,month:i32,day:i32){
        self.datum.change(year,month,day);
        self.datum.format_date();
    }
    fn get_value(mut self,text:String){
        self.value = text;
    }
}
fn main(){
    let mut list:LinkedList<Data> = LinkedList::new();
    let mut input:String = String::new();
    let exit_code = 0;
    loop{
        print_commands();
        print!("->");
        input = user_input(input);
        match input.to_lowercase().trim(){
            "zavri" => std::process::exit(exit_code),
            "predchozi" => predchozi(list),
            "dalsi" => dalsi(list),
            "novy" => novy(list),
            "uloz" => uloz(list),
            "smaz" => smaz(list),
             _ => ()
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
fn user_input(mut input:String) -> String{
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
fn dalsi(list:LinkedList<Data>) -> LinkedList<Data>{

}
fn predchozi(list:LinkedList<Data>) -> LinkedList<Data>{

}
fn novy(mut list:LinkedList<Data>) -> LinkedList<Data>{
    let mut input:String = String::new();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("Date: ");
    input = user_input(input);

}
fn uloz(mut list:LinkedList<Data>) -> LinkedList<Data>{

}
fn smaz(mut list:LinkedList<Data>) -> LinkedList<Data>{

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





