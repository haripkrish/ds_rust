use log::{info};
use std::io::{self, Write};
use colored::*;

use crate::ds::tree::heap::min_heap::solution::min_heap;
use crate::ds::tree::heap::max_heap::solution::max_heap;

pub fn run() {
    info!("Cmd handler");

    loop {
        // Display the menu
        match display_main_menu().as_str() {
            "1" => handle_stack(),
            "2" => handle_tree(),
            "3" => {
                println!("Exiting the application.");
                break; // Exit the loop
            }
            _ => {
                println!("Invalid selection. Please choose either 1, 2, or 3.");
            }
        }

        println!("-----------------------------------");
    }
}
// Display modules
fn display_main_menu() -> String {
    println!("Available commands:");
    println!("1. Stack");
    println!("2. Tree");
    println!("3. Exit");
    println!("Please select a command (1, 2, or 3): ");
    io::stdout().flush().unwrap();

    // Get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn display_tree_menu() -> String {
    println!("{}", "Select a tree type:".purple());
    println!("{}", "1. Min Heap".purple());
    println!("{}", "2. Max Heap".purple());
    println!("{}", "3. Exit".purple());
    println!("{}", "Please select a tree type (1, 2, or 3): ".purple());
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


// Handle sections

fn handle_stack() {
    info!("Selected Stack");
    // Here you would call the function related to Stack
    println!("Stack command executed.");
}

fn handle_tree() {
    loop {
        match display_tree_menu().as_str() { // Use as_str() to match against &str
            "1" => {
                info!("Selected Min Heap");
                min_heap();
                println!("Min Heap command executed. \n");
            }
            "2" => {
                info!("Selected Max Heap");
                max_heap();
                println!("Max Heap command executed. \n");
            }
            "3" => break, // Exit the heap type loop
            _ => println!("Invalid selection. Please choose either 1, 2, or 3.\n"),
        }

        // Optional: add a separator for clarity between commands
        println!("-----------------------------------");
    }
}
