#![allow(unused)]
use std::env;
use std::fs::{self,DirEntry};
use std::error::Error;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

// listing directory
fn list_dir (dir: &Path) -> Result<(), Box<dyn Error>>{
    if dir.is_dir(){
        for i in fs::read_dir(&dir)?{
            let entry = i?;
            let path = entry.path();

            if path.is_dir(){
                println!("{}", path.display());
                list_dir(&path)?;
            }
        }

    }
    
    Ok(())
}

// listing files
fn list_files (dir: &Path) -> Result<(), Box<dyn Error>>{
    if dir.is_dir(){
        for i in fs::read_dir(&dir)?{
            let entry = i?;
            let path = entry.path();

            if path.is_dir(){
                list_files(&path)?;
            }
            else{
                println!("{}", entry.path().display());
            }
        }

    }
    
    Ok(())
}

fn find_by_name (dir: &Path, name: &String) -> Result<(), Box<dyn Error>>{
    if dir.is_dir(){
        for i in fs::read_dir(&dir)?{
            let entry = i?;
            let path = entry.path();
            let _name = name;

            if path.is_dir(){
                if entry.file_name().into_string() == Ok(_name.to_string()){
                    println!("{}", path.display());
                }
                else{
                    find_by_name(&path, &_name);
                }
            }
            else{
                if path.file_name().unwrap().to_str() == Some(_name.as_str()){
                    println!("{}", path.display());
                }
            }
        }
    }

    Ok(())
}

fn grep_word(word: &String, path_to_file: &Path) -> Result<(), Box<dyn Error>>{
    let file = fs::File::open(path_to_file)?;
    let reader = BufReader::new(file);
    let word = word;

    for line in reader.lines(){
        let x = line?;
        
        if x == *word{
            println!("{:?}", x);
        }  
    }

    // bufReader?
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>>{
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
            Ok(())
        }
        "cat" =>{
            let path = env::args().nth(2).expect("no path given!");
            let content = fs::read_to_string(path).expect("Not able to read file!");

            println!("{}", content);
            Ok(())
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
            Ok(())
        }
        "find" =>{
            let mut dir_path = Path::new("./");

            if args.len() == 2{
                for i in fs::read_dir(&dir_path).unwrap(){
                    println!("{}", i.unwrap().path().display());
                }
                Ok(())
            }
            else{
                dir_path = Path::new(&args[2]);
                let options = &args[3];
                let expression = &args[4];

                if options == "-type"{
                    match expression.as_str(){
                        "f" =>{
                            list_files(&dir_path);
                        }
                        "d" =>{
                            list_dir(&dir_path);
                        }
                        _ =>{

                        }
                    }
                }

                if options == "-name"{
                    find_by_name(&dir_path, expression);
                }
                Ok(())
            }

        }
        "grep" =>{
            let word = &args[2];
            let path = Path::new(&args[3]); 

            grep_word(word,path);

            Ok(())
        }
        _=> {
            println!("");
            Ok(())
        }
    } 
    
}
