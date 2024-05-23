use std::env;
use std::fs;
//use std::fs::{self, DirEntry};
//use std::path::Path;

// enum for commands ?
/*
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> std::io::Result<()>{
    if dir.is_dir(){
        for i in fs::read_dir(dir)?{
            let entry = i?;
            let path = entry.path();
            
            if path.is_dir(){
                visit_dirs(&path, cb)?;
            }
            else{
                cb(&entry);
            }
        }
    }

    Ok(())
}
*/

fn main(){
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    match command.as_str(){
        "echo" =>{
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
        "cat" =>{
            let path = env::args().nth(2).expect("no path given!");
            let content = fs::read_to_string(path).expect("Not able to read file!");

            println!("{}", content);
        }

        "ls" =>{
            let path_cur_dir = String::from("./");

            if  args.len() > 2{
                let paths = env::args().nth(2).unwrap();
                let path = fs::read_dir(paths).unwrap();
                for i in path{
                    println!("{}",i.unwrap().path().display());
                }
            }
            else{
                let path = fs::read_dir(path_cur_dir).unwrap();
                for i in path{
                    println!("{}", i.unwrap().path().display());
                }
            }
        }
        "find" =>{
            let mut path = ".";

            if args.len() == 2{
                for i in fs::read_dir(&path).unwrap(){
                    println!("{}", i.unwrap().path().display());
                }
                return;
            }
            else{
                path = &args[2];

                let options = &args[3];
                let expression = &args[4];

                if options == "-type"{
                    match expression.as_str(){
                        "f" =>{
                        }
                        "d" =>{

                        }
                        _ =>{

                        }
                    }
                }

                if options == "-name"{
                    for i in fs::read_dir(&path).unwrap().into_iter(){
                        let file_name = i.unwrap().file_name();
                        
                        if file_name == expression.as_str(){
                            println!("{}/{}", &path, &file_name.into_string().unwrap());
                            break;
                        }
                    }
                }
            }

        }
        "grep" =>{
            /*
            let word = &args[2];
            let file = fs::read_to_string(&args[3]).unwrap(); 
            */
        }
        _=> print!("")
    } 
    
}
