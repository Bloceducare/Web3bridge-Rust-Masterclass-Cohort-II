mod cli;
mod framework;
mod handlers;
mod services;
mod templates;

use clap::Parser;
use colored::*;

use cli::{Cli, SubCommand};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let cli = Cli::parse();

    println!();
    println!(
        "{}",
        "  🦀 Ngozi-Stephen- rust-scaffold — Backend Project Generator  "
            .black()
            .on_yellow()
            .bold()
    );
        println!();

   

    match cli.command.unwrap_or(SubCommand::New) {
        SubCommand::New => {
            let project_name = handlers::prompt_project_name(cli.name);
            
            let framework = handlers::prompt_framework(cli.framework);

            
            // Run the scaffolder!
           
            services::scaffold_project(&project_name, &framework)?;


            // Print the success summary
            
            handlers::print_success_summary(&project_name);

         
        }
        SubCommand::Delete => {
           
            
            let project_name = handlers::prompt_project_name(cli.name);

           
            // Ask the user to confirm before doing anything destructive
            if handlers::prompt_delete_confirmation(&project_name) {
                services::delete_project(&project_name)?;
                handlers::print_delete_success(&project_name);
            } else {
                handlers::print_delete_cancelled();
            }
        }
    }

    Ok(())
}