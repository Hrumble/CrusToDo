# Crustodo
*I asked chatgpt:
"My github is empty, I'd like to get started doing some projects, give me a few ideas"
And one of the first answer was "Make a todo list", so here we are.*

Crustodo is a CLI todo list manager written in rust, i'd like to put "blazingly fast" like every other Rust project out there, but unfortunately, I have no clue if it is.
This is my first Rust project, and I spent all night on it, glad it worked out.

# Installation
## Requirements
To use this project, you'll need:
- Rust, see [Installing Rust](https://www.rust-lang.org/tools/install)
**that's it** 
## Setup 

Clone the repository using 
```sh 
$ git clone https://github.com/Hrumble/CrusToDo
```

Then cd into the directory and build it
```sh
$ cd CrusToDo && cargo build --release
```

The executable will be located at: `./target/release/crustodo`

Crustodo will also create a file `crustodo.json` the first time you run it inside `$HOME/crustodo/crustodo.json`.
This file contains your actual tasks, you can modify them there, but it's not recommended as I didn't
include any failsafe, and **if the program can't parse the file, it will reset it.**
# Usage

You use crustodo from your terminal, you can:
- Create lists
- Create tasks inside those lists
- Set tasks completed or todo
- remove tasks
*It's a todo list I didn't lie*

the syntax is as follow
```sh
$ crustodo [TODOLIST_NAME|create|remove] [set|add|TODOLIST_NAME] <TASK_ID>
```
You can print the help screen at any time by running 
```sh
$ crustodo help
```

Some examples:
```sh
$ crustodo create groceries ; Creates a new crustodo list called groceries

$ crustodo groceries ; displays all tasks inside the groceries list

$ crustodo groceries add ; prompts the program to create a new task

$ crustodo groceries set 2 complete ; marks the task with id=2 to complete

$ crustodo groceries remove 2 ; removes the task with id=2

$ crustodo remove groceries ; removes the entire list called "groceries"
```
# Personal thoughts 

Creating the todolist logic in itself was pretty simple and straightforward, the two things I struggled most with are:
- Reading and Writing to the file system
- Error handling

Coming from a Python background, you can guess these were pretty arduous tasks. As a personal note, I will never try to use TOML again for storing complicated data structures, JSON is and will always be my baby.

Overall happy to have made an actual functioning program for once.
