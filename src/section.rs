use context::Context;
use failure::Error;
use serde_json;

pub enum Exec {
    SQL,
    Shell,
    Python,
}

pub struct Section {
    pub include: Option<String>,
    pub reference: String,
    pub format: String,
    pub process: Option<Exec>,
    pub body: String,
}

impl Section {
    pub fn parse(txt: &str) -> Result<Vec<Section>, Error> {
        unimplemented!()
    }

    pub fn apply<T>(&self, value: serde_json::Value, ctx: &T) -> Result<serde_json::Value, Error>
    where
        T: Context,
    {
        unimplemented!()
    }
}