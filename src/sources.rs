use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::{
    error::Ewwow,
    fnt::{self, FntFile},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SourceId {
    Image(usize),
    Fnt(usize),
}

impl SourceId {
    fn index(self) -> usize {
        match self {
            SourceId::Image(i) => i,
            SourceId::Fnt(i) => i,
        }
    }
}

#[derive(Debug)]
pub struct Sources {
    pub images: Vec<(PathBuf, image::DynamicImage)>,
    pub fnt_files: Vec<(PathBuf, fnt::FntFile)>,

    pub source_file_aliases: HashMap<String, SourceId>,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            images: Vec::new(),
            fnt_files: Vec::new(),
            source_file_aliases: HashMap::new(),
        }
    }

    pub fn insert_alias(&mut self, id: SourceId, alias: String) -> anyhow::Result<()> {
        // We only need the value inside the if scope below, but we move this up here
        // to always check that `id` is valid!
        let this_path = self
            .get_path(id)
            .with_context(|| format!("Failed to insert source alias '{}'", &alias))?;

        // Check if the alias has been registered already
        if let Some(other_id) = self.source_file_aliases.get(&alias) {
            let other_path = self.get_path(*other_id).expect(
                "The invariance of `source_file_aliases` only listing valid ids has been broken!",
            );

            if this_path == other_path {
                println!(
                    "INFO: Source file alias '{}' has been inserted already for path {}",
                    &alias,
                    this_path.to_str().unwrap(),
                );

                return Ok(());
            }

            println!(
                "WARN: Source file alias exists already! It refers to the file '{}'. Aborting insertion of alias.",
                other_path.to_str().unwrap(),
            );

            return Ok(());
        }

        self.source_file_aliases.insert(alias, id);

        Ok(())
    }

    pub fn get_path(&self, id: SourceId) -> anyhow::Result<&Path> {
        match id {
            SourceId::Image(index) => self
                .images
                .get(index)
                .map(|(path_buf, _)| path_buf.as_path()),
            SourceId::Fnt(index) => self
                .fnt_files
                .get(index)
                .map(|(path_buf, _)| path_buf.as_path()),
        }
        .ok_or(Ewwow)
        .with_context(|| format!("Invalid source id {id:?}"))
    }

    pub fn get_relative_path(&self, id: SourceId, file: &str) -> anyhow::Result<PathBuf> {
        let path = self.get_path(id)?;
        Ok(path.with_file_name(file))
    }

    pub fn try_load_source<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<SourceId> {
        let path: &Path = path.as_ref();

        // 1. Check the file extension
        let ext = path
            .extension()
            .ok_or(Ewwow)
            .with_context(|| {
                format!(
                    "Failed to determine extension of source file '{}'.",
                    path.to_str().unwrap(),
                )
            })?
            .to_str()
            .unwrap();

        let path_str = path.to_str().unwrap();

        let id = match ext {
            "fnt" => self.try_load_fnt_source_file(path),
            "png" => self.try_load_image_source_file(path),
            _ => {
                Ewwow
                    .raise()
                    .with_context(|| format!("Unrecognized source file extension '{ext}'"))?;

                unreachable!();
            }
        }
        .with_context(|| format!("Failed to load source file '{path_str}'"))?;

        Ok(id)
    }

    fn try_load_fnt_source_file(&mut self, path: &Path) -> anyhow::Result<SourceId> {
        // Check if the file has been loaded already
        let file_name = path
            .file_name()
            .expect(
                "We already checked that the path has an extension, so it should have a file name.",
            )
            .to_str()
            .unwrap()
            .to_string();

        if let Some(id) = self.source_file_aliases.get(&file_name) {
            println!("INFO: Source file '{file_name}' has been loaded already");
            return Ok(*id);
        }

        // Load the file
        let file_contents = std::fs::read_to_string(path)?;
        let fnt_file = FntFile::try_parse(&file_contents)?;

        // Register the file in the vec
        let id = SourceId::Fnt(self.fnt_files.len());
        let canonical_path_name = PathBuf::from(path)
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize path '{}'", path.to_str().unwrap()))?;
        self.fnt_files.push((canonical_path_name, fnt_file));

        // Register the file name as an alias
        self.source_file_aliases.insert(file_name, id);

        // Recursively load dependencies
        self.try_load_fnt_file_dependencies(id).with_context(|| {
            format!(
                "Failed loading dependencies of '{}'",
                path.to_str().unwrap()
            )
        })?;

        Ok(id)
    }

    fn try_load_fnt_file_dependencies(&mut self, id: SourceId) -> anyhow::Result<()> {
        let fnt_file = &self.fnt_files[id.index()].1;

        for dep in fnt_file.dependencies() {
            let path = self.get_relative_path(id, &dep)?;

            let _ = self
                .try_load_source(path.as_path())
                .with_context(|| format!("Failed loading dependency '{dep}'"))?;
        }

        Ok(())
    }

    fn try_load_image_source_file(&mut self, path: &Path) -> anyhow::Result<SourceId> {
        // Check if the file has been loaded already
        let file_name = path
            .file_name()
            .expect(
                "We already checked that the path has an extension, so it should have a file name.",
            )
            .to_str()
            .unwrap()
            .to_string();

        if let Some(id) = self.source_file_aliases.get(&file_name) {
            println!("INFO: Source file '{file_name}' has been loaded already");
            return Ok(*id);
        }

        // Load the image
        let image = image::open(path)
            .with_context(|| format!("Failed to read png image '{}'", &file_name))?;

        let id = SourceId::Image(self.images.len());
        let canonical_path_name = PathBuf::from(path)
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize path '{}'", path.to_str().unwrap()))?;
        self.images.push((canonical_path_name, image));

        self.source_file_aliases.insert(file_name, id);

        Ok(id)
    }
}
