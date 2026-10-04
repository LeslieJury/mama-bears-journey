# Mama Bear's Journey - A Rust Console RPG

## Overview
This is a text-based console role-playing game written in Rust. In this game, you play as a mama black bear whose cub has wandered off. The player must navigate through different environments, collect food, and successfully retrieve their scared cub out of a dark cave before time runs out.

I made this project to practice the basics of Rust, especially control flow, structs, loops, and methods.

[Youtube Video Demonstration] (youtube)

## Features & Technical Requirements
Some key features of Rust I've used are...
* **Variables:** I use mutable variables for things that change during the game, like the bear's location and food.
* **Conditionals:** if/else statements are used for movement and deciding what happens based on the player's choices.
* **Loops:** The main game loop keeps the game running until the player wins or loses.
* **Functions & References:** Uses methods that take borrowed references (`&self` and `&mut self`) to manage data safely.
* **Object-Oriented Programming** I used a Bear struct to keep track of the player's location and food, with methods in an impl block to change and access that information.

## Built with
* **Language:** Rust
* **Build System:** Cargo

## How to Run the Game
1. Ensure you have Rust and Cargo installed on your system.
2. Open your terminal and navigate to the root directory of this project.
3. Run the following command to compile and start the game:
   `cargo run`
4. Follow the on-screen prompts and enter the number corresponding to your choice.

## Useful Websites
* [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
* [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

## Ideas for the future
* More locations
* Energy or stamina system.
* Inventory system
