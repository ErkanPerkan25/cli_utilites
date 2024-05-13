use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    
    if command == "echo"{
        let options = &args[2..];

        let mut content:String = String::from("");

        for (i,x) in options.iter().enumerate(){
            if i == options.len()-1{
                content.push_str(x);
                break;
            }
            else{
                content.push_str(x);
                content.push_str(" ");
            }
        }
        println!("{}", content);
    }

    if command == "cat"{
        let path = env::args().nth(2).expect("no path given!");
        let content = fs::read_to_string(path).expect("Not able to read file!");

        println!("{}", content);
    }
    
    // Debugging
    //println!("Command: {}", command);
    //println!("Options: {}", content);
}
