use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Scaffoldify {
    #[clap(subcommand)]
    pub commands: Subcommands,
}


#[derive(Debug, Subcommand)]
pub enum Subcommands {
   //  create differnt scaffolds
    Create(Scaffolds),
}

#[derive(Debug,  Args)]
pub struct Scaffolds {
    pub name: Option<String>,

    #[clap(subcommand)]
    pub available_scaffolds: AvailableScaffolds,

}

#[derive(Debug, Subcommand)]
pub enum AvailableScaffolds { 
    React,
    Lit,
    Go
}

fn create_go_project(project_name: String) {
    


std::fs::create_dir_all(&project_name);

let output = std::process::Command::new("go")
    .arg("mod")
    .arg("init")
    .arg(&project_name)
    .current_dir(format!("/{}", project_name));
}

fn get_name(project_name: Option<String>, default: &str) -> String {
    match project_name {
        Some(p) => p,
        None => default.to_string()
    }
}

fn main() {
    let cli = Scaffoldify::parse();

   
    match cli.commands {
        Subcommands::Create(scaffold) => {
    
            match scaffold.available_scaffolds {
                AvailableScaffolds::React => {
                    println!("Writing in react!")
                },
                AvailableScaffolds::Lit => {
                    println!("Writing in Lit!")
                },
                AvailableScaffolds::Go => {
                    create_go_project(get_name(scaffold.name, "new_go"));
                },
            }
        }
    }
}
