use clap::{Parser, Subcommand};
use std::env;
use std::error::Error;
use std::fs;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut store = Store::new()?;

    // Handle command
    match args.command {
        Some(Commands::Add { thing }) => add(thing, store),
        Some(Commands::List) => list(store),
        None => Ok(()),
    }?;

    Ok(())
}

fn add(thing: Option<String>, mut store: Store) -> Result<(), Box<dyn Error>> {
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

    fn insert(&mut self, thing: String) -> Result<(), Box<dyn Error>> {
        self.data.push(thing);
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
    Add { thing: Option<String> },
    List,
}
