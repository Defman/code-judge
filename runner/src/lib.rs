use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Artifact;

impl Artifact {
    pub fn download(&self) -> Result<PathBuf> {
        todo!()
    }
}

pub trait Runner {
    fn run(&self, artifact: &Artifact, input: &str) -> Result<String>;
}

