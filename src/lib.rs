use clap::{Parser, Subcommand};
use std::env;
use std::error::Error;
use std::fs;
use rand::prelude::*;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // Load and create directory
    let ihft_dir = format!("{}/.local/share/ihft", env::var("HOME")?);
    fs::create_dir_all(&ihft_dir)?;

    let data = Data::new(ihft_dir)?;

    // Handle command
    match args.command {
        Some(Commands::Add { thing }) => add(data, thing),
        Some(Commands::List) => list(data),
        Some(Commands::Remove { thing }) => remove(data, thing),
        Some(Commands::Undo) => undo(data),
        None => ihft(data),
    }?;
    Ok(())
}

fn add(mut data: Data, thing: Option<String>) -> Result<(), Box<dyn Error>> {
    match thing {
        Some(t) => {
            data.things.insert(&t)?;
            data.hist.insert(&format!("add {}", &t))?;
        },
        None => (),
    }
    Ok(())
}

fn list(data: Data) -> Result<(), Box<dyn Error>> {
    for v in &data.things.data {
        println!("{}", v);
    }
    if &data.things.data.len() == &(0 as usize) {
        println!("No things stored");
    }
    Ok(())
}

fn remove(mut data: Data, thing: String) -> Result<(), Box<dyn Error>> {
    match data.things.remove(&thing) {
        Ok(_) => data.hist.insert(&format!("remove {}", &thing)),
        Err(e) => Err(e),
    }
}

fn undo(mut data: Data) -> Result<(), Box<dyn Error>> {
    let mut i = match data.hist.data.first() {
        Some(v) => v.split(' '),
        None => return Err("nothing to undo".into()),
    };

    let cmd = match i.next() {
        Some(v) => v,
        None => return Err("hist file corrupted".into()),
    };

    let v = match i.next() {
        Some(v) => v.to_string(),
        None => return Err("hist file corrupted".into()),
    };

    match cmd {
        "add" => data.things.remove(&v),
        "remove" => data.things.insert(&v),
        _ => return Err("hist file corrupted".into()),
    }?;

    data.hist.data.remove(0);
    data.hist.write()?;
    
    Ok(())
}

fn ihft(mut data: Data) -> Result<(), Box<dyn Error>> {
    match data.things.get_one() {
        Ok(t) => data.hist.insert(&format!("remove {}", t)),
        Err(e) => return Err(e),
    }?;
    Ok(())
}

struct Data {
    things: Store,
    hist: Store,
}

impl Data {
    fn new(ihft_dir: String) -> Result<Data, Box<dyn Error>> {
        Ok(
            Data {
                things: Store::new(&ihft_dir, &String::from("things"))?,
                hist: Store::new(&ihft_dir, &String::from("hist"))?,
            }
            )
    }
}

struct Store {
    path: String,
    data: Vec<String>,
}

impl Store {
    fn new(ihft_dir: &String, file: &String) -> Result<Store, Box<dyn Error>> {
        // Do file stuff
        let ihft_store = format!("{}/{}", ihft_dir, file);

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

    fn get_one(&mut self) -> Result<String, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let thing = match self.data.iter().choose(&mut rng) {
            Some(t) => t.to_string(),
            None => {
                return Err("store empty".into());
            }
        };
        self.remove(&thing)?;
        println!("{}", &thing);
        Ok(thing)
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

    fn insert(&mut self, thing: &String) -> Result<(), Box<dyn Error>> {
        self.data.insert(0, thing.to_string());
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
    /// Undo the last action you took
    Undo,
}
