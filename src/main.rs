use std::{env, fs, process, io::prelude::*};

fn get_name(args: &[String]) -> Result<String, &str>{
    
    if args.len() < 2 {
        return Err("missing file operand\nTry: 'cargo run <file.json>'")
    }
    
    let filename = args[1].clone();
    Ok(filename)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let filename = get_name(&args)
        .unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
    
    let path: String = match env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string(){
            Ok(data) => data + "/" + &filename,
            Err(e) => {
                eprintln!("Error!!{:?}",e);
                process::exit(1)
            }
        };

    let data = match fs::read_to_string(&path){
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: '{}' {}", &filename, e);
            process::exit(1)
        }
    };

    let mut file = fs::File::create("out.json").unwrap();

    file.write("{\n".as_bytes()).unwrap();
    for (i,line) in data.lines().enumerate(){
        file.write(format!("\t\"{}\": ",i).as_bytes()).unwrap();
        file.write(line[..].as_bytes()).unwrap();
        file.write(",\n".as_bytes()).unwrap();
    }
    file.write("\t\"done\": \"done\"\n}".as_bytes()).unwrap();
}
