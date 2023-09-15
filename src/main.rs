use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    collections::HashSet,
    fs::{write, File},
    io::BufReader,
    path::PathBuf,
};

#[derive(Parser)]
#[command(
    about,
    version,
    max_term_width = 80,
    after_help = "\
---

# Assumptions

* The JSON dotted path keys for array and ID don't have periods.

# Examples
    
1. Use `json-split-aom -a 'Apple.Banana' -i 'id' file.json` to extract objects
   to files named `Apple.Banana-id-ID.json` given a `file.json` with content:
   `{\"Apple\":{\"Banana\":[{\"id\":12,...},...]}}`

2. Use `json-split-aom -a 'Apple' -i 'Banana.id' file.json` to extract objects
   to files named `Apple-Banana.id-ID.json` given a `file.json` with content:
   `{\"Apple\":[{\"Banana\":{\"id\":12,...}},...]}`

3. Use `json-split-aom -a 'Apple.Banana' -i 'Cherry.id' file.json` to extract
   objects to files named `Apple.Banana-Cherry.id-ID.json` given a `file.json`
   with content: `{\"Apple\":{\"Banana\":[{\"Cherry\":{\"id\":12,...},...}]}}`
"
)]
struct Cli {
    /// Dotted JSON path to an array in the input file
    #[arg(short, value_name = "JSON_PATH")]
    array_path: String,

    /// Dotted JSON path to the ID in the array element
    #[arg(short, value_name = "JSON_PATH")]
    id_path: String,

    /// Pretty print output files
    #[arg(short)]
    pretty: bool,

    /// Allow ID path collisions; still gives warnings but duplicates will overwrite previous files
    #[arg(short)]
    collisions: bool,

    /// Input file(s)
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut seen = HashSet::new();

    eprintln!("* Files");

    for path in &cli.files {
        eprint!("    * {:?}", path.display());
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let d: serde_json::Value = serde_json::from_reader(reader)?;

        eprintln!("\n        * IDs");
        match descend(&cli.array_path, &d)?.as_array() {
            Some(a) => {
                for v in a {
                    match descend(&cli.id_path, v)?.as_str() {
                        Some(id) => {
                            let id = id.to_string();
                            if seen.contains(&id) {
                                if cli.collisions {
                                    process(&id, &v, &cli, " (DUPE!)")?;
                                    seen.insert(id);
                                } else {
                                    return Err(anyhow!("ID collision: {id:?}"));
                                }
                            } else {
                                process(&id, &v, &cli, "")?;
                                seen.insert(id);
                            }
                        }
                        None => {
                            return Err(anyhow!("ID value could not convert to a string"));
                        }
                    }
                }
            }
            None => {
                return Err(anyhow!("Value is not an array"));
            }
        }
    }

    eprintln!("\nDone!");

    Ok(())
}

/**
Process an array element
*/
fn process(id: &str, v: &serde_json::Value, cli: &Cli, post: &str) -> Result<()> {
    write(
        filename(&cli.array_path, &cli.id_path, &id, ".json"),
        json(&v, cli.pretty)?,
    )?;
    eprintln!("            * {id:?}{post}");
    Ok(())
}

/**
Generate the output filename
*/
fn filename(array_path: &str, id_path: &str, id: &str, ext: &str) -> String {
    format!("{array_path}-{id_path}-{id}{ext}")
}

/**
Serialize a JSON value to string, either pretty or compact
*/
fn json(v: &serde_json::Value, pretty: bool) -> Result<String> {
    if pretty {
        Ok(serde_json::to_string_pretty(v)?)
    } else {
        Ok(serde_json::to_string(v)?)
    }
}

/**
Recursively descend into the JSON structure given a dotted path
*/
fn descend(path: &str, d: &serde_json::Value) -> Result<serde_json::Value> {
    if path.is_empty() {
        Ok(d.clone())
    } else {
        let mut s = path.split('.');
        let top = s.next().unwrap().to_string();
        let rest = s.collect::<Vec<_>>().join(".");
        match d.as_object() {
            Some(m) => match m.get(&top) {
                Some(v) => descend(&rest, v),
                None => Err(anyhow!("Invalid key: {top:?}")),
            },
            None => Err(anyhow!("Value is not an object")),
        }
    }
}
