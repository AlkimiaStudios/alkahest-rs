use serde_derive::Deserialize;
use std::path::PathBuf;
use std::fs;
use toml;

use crate::trace;

#[derive(Debug, Deserialize)]
pub(crate) struct ProjectMetadata {
    pub name: String,
    pub directory: PathBuf,
}

#[derive(Debug)]
pub(crate) struct ProjectContext {
    pub metadata: ProjectMetadata,    
}

pub(crate) fn init(project_dir: &String) -> Result<ProjectContext, Box<dyn std::error::Error>> {
    let mut p = PathBuf::from(project_dir);
    // Load the .alkahest/project.toml file for project metadata
    p.push(".alkahest/project.toml");

    let metadata = match fs::read_to_string(p) {
        Ok(c) => c,
        Err(_) => String::from(""),
    };
    let metadata: ProjectMetadata = toml::from_str(&metadata)?;
    trace!("Loaded project metadata from {}", metadata.directory.display());

    Ok(ProjectContext { metadata })
}
