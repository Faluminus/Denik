#![feature(linked_list_remove)]

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

        //for formatting date into way i want it to look, panics if wrong data are passed


        let date_split:Vec<&str> = input.trim().split('.').collect();


        self.change(date_split[0].parse::<i32>().unwrap(), date_split[1].parse::<i32>().unwrap(), date_split[2].parse::<i32>().unwrap());

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

        //self explanatory

        self.datum.format_date(&input);
    }
    fn get_value(&mut self,text:String){

        //self explanatory

        self.value = text;
    }
}
fn main(){

    //user interface
    let mut list:LinkedList<Data> = LinkedList::new();
    let mut iter = list.iter();
    let mut counter:usize = 0;
    let mut iternow:&Data  = &Data::new();
    let mut input:String = String::new();
    let exit_code = 0;

    loop{
        print_commands();
        if let Data = &iternow {
            println!(
                "Date {}.{}.{}  Text {}",
                Data.datum.day, Data.datum.month, Data.datum.year, Data.value
            );
        }
        print!("->");
            input = user_input(input);
            match input.to_lowercase().trim(){
                "zavri" => std::process::exit(exit_code), //exits
                "predchozi" => {iternow = iter.clone().last().unwrap();if counter >=0 { counter-=1;} }, //go back
                "dalsi" => {iternow = iter.clone().next().unwrap(); if counter <= iter.len() -1 {counter += 1}},//go front
                "novy" => novy(&mut list), //create new
                "uloz" => uloz(&list), //save
                "smaz" => smaz(&mut list,counter), //delete
                _ => ()
            }
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
fn print_zaznam_info(iternow:&Data,list:&LinkedList<Data>){
    println!("Pocet zaznamu: {}",list.iter().count());

    println!("Date: {}.{}.{}", iternow.datum.day,iternow.datum.month,iternow.datum.year ) ;
    println!("Value:");
    println!("{}",iternow.value);
}
fn novy(list:&mut LinkedList<Data>){

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
}
fn uloz(list:&LinkedList<Data>) {

}
fn smaz(list:&mut LinkedList<Data>,index:usize){
    list.remove(index);
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





