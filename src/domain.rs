use eyre::Result;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

const DATABASE: &str = include_str!("database.toml");

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

pub fn database() -> Result<Database> {
    Ok(Database(toml::from_str(DATABASE)?))
}
