use std::env;
use std::process;
use stephenngozi_build_cli_project::Config;


fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err |{
        eprintln!("Problem parsing argumemts: {}", err);
        process::exit(1);
    });


    println!(" searching for {}", config.query);
    println!("In File {}", config.filename);

    if let Err(e) = stephenngozi_build_cli_project::run(config){
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
  
}




