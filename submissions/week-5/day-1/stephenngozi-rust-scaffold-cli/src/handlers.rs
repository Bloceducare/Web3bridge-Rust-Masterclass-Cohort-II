

use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::framework::Framework;


// Prompt user for the project name


pub fn prompt_project_name(provided: Option<String>) -> String {
    provided.unwrap_or_else(|| {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project name")
            .validate_with(|input: &String| {
                if input.is_empty() {
                    Err("Project name cannot be empty".to_string())
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .expect("Failed to read project name")
    })
}


// Prompt user to choose a framework
pub fn prompt_framework(provided: Option<String>) -> Framework {
    if let Some(f) = provided {
        Framework::from_str(&f).unwrap_or_else(|| {
            eprintln!(
                "{} Unknown framework '{}'. Use 'axum' or 'actix'.",
                "Error:".red().bold(),
                f
            );
            std::process::exit(1);
        })
    } else {
        let frameworks = vec!["Axum", "Actix-web"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a framework")
            .items(&frameworks)
            .default(0)
            .interact()
            .expect("Failed to read selection");
        match selection {
            0 => Framework::Axum,
            1 => Framework::Actix,
            _ => unreachable!("Invalid selection"),
        }
    }
}

// Print the success summary after scaffolding completes
pub fn print_success_summary(project_name: &str) {
    println!();
    println!("  {}", " Project scaffolded successfully!".green().bold());
    println!();
    println!("  {}", "Generated structure:".bold());
    println!("  {}/", project_name.bright_cyan());
    println!("  ├── {}", "Cargo.toml".yellow());
    println!("  ├── {}", ".env".yellow());
    println!("  ├── {}", ".gitignore".yellow());
    println!("  └── src/");
    println!("      ├── {}", "main.rs".yellow());
    println!("      ├── handlers/");
    println!("      │   └── {}", "mod.rs".yellow());
    println!("      ├── models/");
    println!("      │   └── {}", "mod.rs".yellow());
    println!("      ├── services/");
    println!("      │   └── {}", "mod.rs".yellow());
    println!("      └── router/");
    println!("          └── {}", "mod.rs".yellow());

    println!();
    println!("  {}", "Next steps:".bold());
    println!("    {} cd {}", "$".dimmed(), project_name.bright_cyan());
    println!("    {} cargo build", "$".dimmed());
    println!("    {} cargo run", "$".dimmed());
    println!();
}


// Prompt user to confirm deletion
pub fn prompt_delete_confirmation(project_name: &str) -> bool {
    use dialoguer::Confirm;

    println!();
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Are you sure you want to delete '{}'? This cannot be undone.",
            project_name.bright_cyan()
        ))
        .default(false) 
        .interact()
        .expect("Failed to read confirmation")
}

// Print the deletion success message
pub fn print_delete_success(project_name: &str) {
    println!();
    println!(
        "  {} '{}' has been deleted.",
        "🗑️  Deleted:".red().bold(),
        project_name.bright_cyan()
    );
    println!();
}


// Print the deletion cancelled message
pub fn print_delete_cancelled() {
    println!();
    println!("  {}", "Deletion cancelled.".dimmed());
    println!();
}
