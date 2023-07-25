use eyre::Result;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display, fs::File, io::BufReader};
use zip::{self, ZipArchive};

// const DATABASE: &str = include_str!("database.toml");
const DATABASE: &str = "sde.zip";

#[derive(Deserialize)]
pub struct Type {
    base_price: Option<f32>,
    capacity: Option<f32>,
    description: Option<HashMap<String, String>>,
    faction_id: Option<u32>,
    graphic_id: Option<u32>,
    market_group_id: Option<u32>,
    mass: Option<f32>,
    masteries: Option<HashMap<u32, Vec<u32>>>,
    meta_group_id: Option<u32>,
    name: HashMap<String, String>,
    portion_size: Option<u32>,
    published: bool,
    race_id: Option<u32>,
    radius: Option<f32>,
    sof_faction_name: Option<String>,
    sound_id: Option<u32>,
    volume: Option<f32>,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.get("en").unwrap())
    }
}

#[derive(Deserialize)]
pub struct Types(HashMap<u32, Type>);

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.0.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Industry {
    Raw,
    Produced {
        input: HashMap<String, u32>,
        outcome: u32,
    },
}

impl Display for Industry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")
    }
}

pub struct Database(HashMap<String, Industry>);

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, industry) in self.0.iter() {
            match industry {
                Industry::Raw => writeln!(f, "[{}]", id)?,
                Industry::Produced { input, outcome } => {
                    writeln!(f, "[{}] = {}", id, outcome)?;

                    for (id, amount) in input {
                        writeln!(f, "{} = {}", id, amount)?;
                    }
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn database() -> Result<Types> {
    let f = File::open(DATABASE)?;
    let reader = BufReader::new(f);
    let mut zip = ZipArchive::new(reader)?;

    let type_reader = zip.by_name("sde/fsd/typeIDs.yaml")?;

    let types = serde_yaml::from_reader(type_reader)?;
    Ok(types)
}
