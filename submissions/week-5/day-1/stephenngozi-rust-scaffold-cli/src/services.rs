// ============================================================
// services.rs — File writing and scaffolding business logic
// ============================================================


use std::fs;
use std::path::Path;
use colored::*;

use crate::framework::Framework;
use crate::templates::{self};


pub fn scaffold_project(
    project_name: &str,
    framework: &Framework,
) -> Result<(), Box<dyn std::error::Error>> {
    let _root = Path::new(project_name);
    println!();
    println!(
        "{}",
        format!("  Scaffolding {} with {}  ", project_name, framework.display_name())
            .black()
            .on_bright_cyan()
            .bold()
    );
    println!();

    // Create directory structure

    let dirs = [
        format!("{}/src/handlers", project_name),
        format!("{}/src/models", project_name),
        format!("{}/src/services", project_name),
        format!("{}/src/router", project_name),
    ];

    // fs::create_dir_all creates ALL directories in the path,
    
    for dir in &dirs {
        fs::create_dir_all(dir)?;
        println!("  {} {}", "Created".green().bold(), dir.dimmed());
    }

    
    // Cargo.toml
    
    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

{deps}"#,
        name = project_name,
        deps = framework.dependencies()
    );

    write_file(&format!("{}/Cargo.toml", project_name), &cargo_toml)?;
    // Write .env file
    
    write_file(&format!("{}/.env", project_name), framework.env_content())?;
    
    // Write .gitignore
    
    write_file(&format!("{}/.gitignore", project_name), templates::GITIGNORE)?;


    // Write src/main.rs

    write_file(
        &format!("{}/src/main.rs", project_name),
        &framework.main_rs_content(project_name),
    )?;

   
    // Write module mod.rs files
    
     let router_mod = framework.router_mod_content();
    let module_files: Vec<(String, &str)> = vec![
        (
            format!("{}/src/handlers/mod.rs", project_name),
            framework.handlers_mod_content(),
        ),
        (
            format!("{}/src/services/mod.rs", project_name),
            templates::SERVICES_MOD,
        ),
        (
            format!("{}/src/models/mod.rs", project_name),
            templates::MODELS_MOD,
        ),
        (
            format!("{}/src/router/mod.rs", project_name),
            &router_mod,
        ),
    ];

    for (path, content) in &module_files {
        write_file(path, content)?;
    }

    Ok(())
}



pub fn write_file(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, content)?;
    println!("  {} {}", "✓ Wrote".green().bold(), path.dimmed());
    Ok(())
}



// Delete a scaffolded project directory
pub fn delete_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(project_name);

    if !path.exists() {
        return Err(format!("Project '{}' not found in the current directory.", project_name).into());
    }

    fs::remove_dir_all(path)?;
    Ok(())
}