use clap::{Parser, Subcommand};
use std::env;
use std::error::Error;
use std::fs;
use rand::prelude::*;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut store = Store::new()?;

    // Handle command
    match args.command {
        Some(Commands::Add { thing }) => add(store, thing),
        Some(Commands::List) => list(store),
        Some(Commands::Remove { thing }) => remove(store, thing),
        None => store.get_one(),
    }?;
    Ok(())
}

fn add(mut store: Store, thing: Option<String>) -> Result<(), Box<dyn Error>> {
    match thing {
        Some(t) => store.insert(t)?,
        None => (),
    }
    Ok(())
}

fn list(store: Store) -> Result<(), Box<dyn Error>> {
    for v in store.data {
        println!("{}", v);
    }
    Ok(())
}

fn remove(mut store: Store, thing: String) -> Result<(), Box<dyn Error>> {
    store.remove(&thing)?;
    Ok(())
}

struct Store {
    path: String,
    data: Vec<String>,
}

impl Store {
    fn new() -> Result<Store, Box<dyn Error>> {
        // Do file stuff
        let ihft_dir = format!("{}/.ihft", env::var("HOME")?);
        let ihft_store = format!("{}/store.txt", &ihft_dir);
        fs::create_dir_all(&ihft_dir)?;

        // Create file if it doesn't exist
        match fs::File::open(&ihft_store) {
            Err(_) => {
                let _ = fs::File::create(&ihft_store)?;
            }
            _ => (),
        };

        let data: Vec<String> = fs::read_to_string(&ihft_store)?
            .lines()
            .map(|l| l.to_string())
            .collect();

        Ok(Store {
            path: ihft_store,
            data,
        })
    }

    fn get_one(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let thing = match self.data.iter().choose(&mut rng) {
            Some(t) => t.to_string(),
            None => {
                return Err("store empty".into());
            }
        };
        self.remove(&thing)?;
        println!("{}", &thing);
        Ok(())
    }

    fn remove(&mut self, thing: &String) -> Result<(), Box<dyn Error>> {
        let idx = match self.data.iter().enumerate().find(|r| r.1 == thing) {
            Some(v) => v.0,
            None => {
                return Err(format!("thing: '{}' does not exist", thing).into());
            }
        };
        self.data.remove(idx);
        self.write()?;
        Ok(())
    }

    fn insert(&mut self, thing: String) -> Result<(), Box<dyn Error>> {
        self.data.push(thing);
        self.write()?;
        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        let write_data: Vec<u8> = self
            .data
            .iter()
            .flat_map(|v| format!("{}\n", v).bytes().collect::<Vec<u8>>())
            .collect();
        fs::write(&self.path, write_data)?;
        Ok(())
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a thing to the store
    Add { thing: Option<String> },
    /// List all things in store
    List,
    /// Remove a thing from the store
    Remove { thing: String },
}
