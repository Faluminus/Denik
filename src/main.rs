use std::io;
use std::io::Write;
use std::collections::LinkedList;



struct Date{
    day: i32,
    month: i32,
    year:i32
}

impl Date{
    fn new() -> Self{

        //generates new Date object

        Date {
            year: 0,
            month: 0,
            day: 0,
        }
    }
    fn change(&mut self,_day:i32,_month:i32,_year:i32 ){

        //for changing value ind Date object

        self.year = _year;
        self.month =_month;
        self.day = _day;
    }
    fn format_date(&mut self,input:&String){

        //for formating date into way i want it to look, panics if wrong data are passed

       
        let date_splited:Vec<&str> = input.trim().split('.').collect();


        self.change(date_splited[0].parse::<i32>().unwrap(),date_splited[1].parse::<i32>().unwrap(),date_splited[2].parse::<i32>().unwrap());

        //check for stupidities xd if some are found, panics
        let month_arr:[i32;12] = [31,28,31,30,31,30,31,31,30,31,30,31];
        if self.month < 1 || self.month > 12{
            panic!();
        }
        if self.day > month_arr[(self.month - 1) as usize] || self.day < 0{
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

        //generates new Data

        let datum:Date = Date::new();
        let value = String::from("");
        Data{
            datum,
            value
        }
    }
    fn get_date(&mut self, input:&String){

        //self explainatory

        self.datum.format_date(&input);
    }
    fn get_value(&mut self,text:String){

        //self explainatory

        self.value = text;
    }
}
fn main(){

    //user interface

    let mut list:LinkedList<Data> = LinkedList::new();
    let mut input:String = String::new();
    let exit_code = 0;


    loop{
        print_commands();
        print!("->");
        input = user_input(input);
        match input.to_lowercase().trim(){
            "zavri" => std::process::exit(exit_code),
            "predchozi" => list = predchozi(list),
            "dalsi" => list = dalsi(list),
            "novy" => list = novy(list),
            "uloz" => list = uloz(list),
            "smaz" => list = smaz(list),
             _ => ()
        }

        //input fix
        input.clear();
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
   list
}
fn predchozi( list:LinkedList<Data>) -> LinkedList<Data>{
    list
}
fn novy(mut list:LinkedList<Data>) -> LinkedList<Data>{

    let mut data:Data = Data::new();
    let mut input:String = String::new();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Like this: 22.11.2005");
    print!("Date: ");
    input = user_input(input);
    data.get_date(&input);
    println!();
    print!("Text: ");
    input = user_input(input);
    data.get_value(input);
    list.push_front(data);
    list


}
fn uloz(list:LinkedList<Data>) -> LinkedList<Data>{
    list
}
fn smaz(list:LinkedList<Data>) -> LinkedList<Data>{
    list
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





