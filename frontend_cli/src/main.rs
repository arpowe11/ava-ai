/*
* Description: AVA AI command line interface that lets you load a pdf 
*              document and chat with ava about the document.
* Author: Alexander Powell
* Version: v1.0
* Dependencies: See 'Cargo.toml' file
*/

// TODO: Add a ASCII title from a file
// TODO: Clean up the inputs and outputs (especially the API responses)

use std::io::{self, Write};
use std::fs::copy;
use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::env;
use dotenv::dotenv;
use reqwest::blocking::Client; 
use serde_json::{json, Value};
use std::time::Duration;

enum MenuOptions {
    Chat,
    LoadFile,
    Help,
    Options,
    Exit,
}

impl MenuOptions {
    fn get_choice(choice: String) -> Option<MenuOptions> {
        match choice.trim().to_lowercase().as_str() {
            "chat" => Some(MenuOptions::Chat),
            "loadfile" => Some(MenuOptions::LoadFile),
            "help" => Some(MenuOptions::Help),
            "options" => Some(MenuOptions::Options),
            "exit" => Some(MenuOptions::Exit),
            _ => None,
        }
    }

    fn display_menu() {
        println!("Options Menu");
        println!("-------------");
        println!("| -Chat     |");
        println!("| -LoadFile |");
        println!("| -Help     |");
        println!("| -Options  |");
        println!("| -Exit     |");
        println!("-------------");
    }
}


fn load_file() {

    // Get the documents directory path from environment variable
    let documents_dir_str: String = match env::var("DOCUMENT_PATH") {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Error: DOCUMENT_PATH environment variable not found.");
            return; 
        }
    };

    let documents_dir: &Path = Path::new(&documents_dir_str);

    // Open a file dialog to let the user select a file
    let file_path: Option<PathBuf> = FileDialog::new()
        .add_filter("PDF Files", &["pdf"])
        .pick_file();

    if let Some(path) = file_path {
        // Get the filename from the path
        let file_name: String = path.file_name().unwrap().to_str().unwrap().to_string();

        // Set the new name for the file
        let changed_name: String = String::from("doc"); 
        let new_file_name: String = format!("{}.pdf", changed_name); 

        // Construct the destination path within the "documents" directory
        let destination_path: PathBuf = documents_dir.join(&new_file_name); 

        // Copy the file to the "documents" directory
        match copy(&path, &destination_path) {
            Ok(_) => {
                println!("File '{}' loaded successfully as '{}'!", file_name, new_file_name);

                //-------------
                // API LOGIC
                //-------------

                // Send API message to notify about successful file copy
                let api_url: String = match env::var("API_LOAD_DOC") {
                    Ok(url ) => url,
                    Err(_) => {
                        eprintln!("Error: API_LOAD_DOC environment variable not found.");
                        return;
                    }
                };

                // Create the client and body to send
                let client = Client::builder()
                    .timeout(Duration::from_secs(60)) // Set the timeout to 60 seconds
                    .build()
                    .expect("Failed to build client");

                let body: Value = json!({
                    "message": "File copied successfully",
                    "original_name": file_name,
                    "new_name": new_file_name,
                    "destination_path": destination_path.to_str().unwrap(),
                });

                // Match on the client to send the body to the api
                match client.post(&api_url).json(&body).send() {
                    Ok(response) => {
                        // Get the response back and print the result of the response
                        if response.status().is_success() {
                            println!("{}", response.text().unwrap());
                        } else {
                            eprintln!("Failed to notify API. Status: {}", response.status());
                        }
                    }
                    Err(e) => eprintln!("Error sending API request: {}", e),
                }
            }
            Err(e) => eprintln!("Error copying file: {}", e),
        }
    } else {
        println!("No file selected.");
    }
}


fn chat_with_ava() {
    loop {
        let mut question: String = String::new();

        // Print the prompt
        print!("\nAsk AVA a question (type 'quit' to quit): ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Get input from user
        io::stdin()
            .read_line(&mut question)
            .expect("Faild to read line...");
        
        if question.trim().to_lowercase() == "quit" {
            break;
        }

        //-------------
        // API LOGIC
        //------------
        // Send API message to notify about successful file copy
        let api_url: String = match env::var("API_SEND_ASNWER") {
            Ok(url ) => url,
            Err(_) => {
                eprintln!("Error: API_SEND_ASNWER environment variable not found.");
                return;
            }
        };

        // Create the client and body to send
        let client = Client::builder()
            .timeout(Duration::from_secs(60)) // Set the timeout to 60 seconds
            .build()
            .expect("Failed to build client");

        let body: Value = json!({
            "message": question.trim(),
        });

        // Match on the client to send the body to the api
        match client.post(&api_url).json(&body).send() {
            Ok(response) => {
                // Get the response back and print the result of the response
                if response.status().is_success() {
                    println!("{}", response.text().unwrap());
                } else {
                    eprintln!("Failed to notify API. Status: {}", response.status());
                    println!("Did you load a document?");
                }
            }
            Err(e) => eprintln!("Error sending API request: {}", e),
        }

    }
}


fn main() {

    // Load environment variables from .env file
    dotenv().ok(); 

    MenuOptions::display_menu();

    loop {
        let mut choice: String = String::new();

        // Print the prompt
        print!("\nPlease enter a choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Get input from user
        io::stdin()
            .read_line(&mut choice)
            .expect("Faild to read line...");

        let item: Option<MenuOptions> = MenuOptions::get_choice(choice);

        match item {
            Some(MenuOptions::Chat) => chat_with_ava(),
            Some(MenuOptions::LoadFile) => load_file(),
            Some(MenuOptions::Exit) => break,
            Some(MenuOptions::Help) => println!("Help is on the way"),  // TODO: Add help menu
            Some(MenuOptions::Options) => MenuOptions::display_menu(),
            None => println!("No choice selected"),
        }
    }

    println!("Exiting system...")

}