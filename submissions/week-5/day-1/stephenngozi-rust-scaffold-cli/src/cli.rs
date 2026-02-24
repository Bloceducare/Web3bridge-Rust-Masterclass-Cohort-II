use clap::{Parser, Subcommand};



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
     #[command(subcommand)]
    pub command: Option<SubCommand>,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub framework: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand  {
    New ,
    Delete ,
}