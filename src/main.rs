#![allow(unused)]
use clap::{Parser, Subcommand, ArgAction};
use std::path::Path;
use std::fs;
use std::process::Command;
use std::string::String;
use std::io::Write;
use chrono::{DateTime, NaiveDateTime, NaiveTime, offset, Local};
use chrono::offset::TimeZone;
use std::time::SystemTime;

/// Personal assistant CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Open {
        title: String
    },
    List {},
    Tasks {
        #[clap(subcommand)]
        subcmd: Option<TasksSubcommands>,
    },
    Log {
    }
}


#[derive(Subcommand)]
enum TasksSubcommands {
    Add {
        title: String,
        due: String,
    },
}

fn init(app_path: &str) {
    fs::create_dir_all(&app_path).unwrap();
    fs::create_dir_all(format!("{}/logs", &app_path)).unwrap();
    fs::create_dir_all(format!("{}/notes", &app_path)).unwrap();
}

fn main() {
    let app_path = format!("{}/.local/share/perdrix/", std::env::var("HOME").unwrap());
    let cli = Cli::parse();
    let editor = std::env::var("EDITOR").unwrap();
    init(&app_path);
    match &cli.command {
        Some(Commands::Open { title }) => {
            // create note file
            let path = format!("{}notes/{}.md", app_path, title);
            let f_exists = Path::new(&path).exists();
            if !f_exists {
                fs::File::create(&path).unwrap();
            }
            Command::new(editor)
                    .arg(&path)
                    .status()
                    .expect("Something went wrong");
        }
        Some(Commands::List {}) => {
            // list notes
            let files = fs::read_dir(format!("{app_path}notes")).unwrap();
            for file in files {
                let fpath = format!("{}", file.unwrap().path().display());
                println!("{}", fpath.replace(&app_path, "").replace(".md", ""));
            }
        }
        Some(Commands::Log {}) => {
            // log time
            let now: DateTime<Local> = Local::now();
            let path = format!("{app_path}logs/{}.md", now.format("%Y-%m-%d"));
            let f_exists = Path::new(&path).exists();
            if !f_exists {
                fs::File::create(&path).unwrap();
            }
            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&path)
                .unwrap();
            let time = now.format("%H:%M:%S");
            writeln!(file, "---\n{}\nhappiness:  \n", time).unwrap();
            Command::new(editor)
                .arg(&path)
                .status()
                .expect("Something went wrong");
        }
        Some(Commands::Tasks {subcmd}) => {
            match subcmd {
                Some(TasksSubcommands::Add {title, due}) => {
                    // add task
                    let path = format!("{}tasks.md", app_path);
                    let f_exists = Path::new(&path).exists();
                    if !f_exists {
                        fs::File::create(&path).unwrap();
                    }
                    let mut file = fs::OpenOptions::new()
                        .append(true)
                        .open(&path)
                        .unwrap();
                    let task = format!("- {} due: {}\n", title, due);
                    file.write_all(task.as_bytes()).unwrap();
                    println!("Task added");
                }
                None => {
                    // list tasks
                    let path = format!("{app_path}tasks.md");
                    let f_exists = Path::new(&path).exists();
                    if !f_exists {
                        fs::File::create(&path).unwrap();
                    }
                    let file = fs::read_to_string(&path).unwrap();
                    // display tasks that are in the future
                    for line in file.lines() {
                        if line.contains("due:") {
                            let due = line.split("due: ").collect::<Vec<&str>>()[1];
                            let due = NaiveDateTime::parse_from_str(&due, "%d/%m/%Y/%H:%M").unwrap();
                            let due = Local.from_local_datetime(&due).unwrap();
                            if due > offset::Local::now() {
                                println!("{}", line);
                            }
                        }
                    }
                }
            }
        }
        None => {} 
    }
}
