use crate::cache::Cache;
use crate::hash::WasmHash;
use memmap::Mmap;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;
use thiserror::Error;
use wasmer::{DeserializeError, Module, SerializeError, Store};

#[derive(Error, Debug)]
/// The Io Serialization error
pub enum IoSerializeError {
    /// An IO error
    #[error(transparent)]
    Io(#[from] io::Error),
    /// A serialization error
    #[error(transparent)]
    Serialize(#[from] SerializeError),
}

#[derive(Error, Debug)]
/// The Io Deserialization error
pub enum IoDeserializeError {
    /// An IO error
    #[error(transparent)]
    Io(#[from] io::Error),
    /// A serialization error
    #[error(transparent)]
    Deserialize(#[from] DeserializeError),
}

/// Representation of a directory that contains compiled wasm artifacts.
///
/// The `FileSystemCache` type implements the [`Cache`] trait, which allows it to be used
/// generically when some sort of cache is required.
///
///
/// # Usage:
///
/// ## Store
/// ```
/// use wasmer_cache::{Cache, IoSerializeError, FileSystemCache, WasmHash};
///
/// # use wasmer::{Module};
/// fn store_module(module: Module) -> Result<Module, IoSerializeError> {
///     // Create a new file system cache.
///     let mut fs_cache = FileSystemCache::new("some/directory/goes/here")?;
///     // Compute a key for a given WebAssembly binary
///     let key = WasmHash::generate(&[]);
///     // Store a module into the cache given a key
///     fs_cache.store(key, module.clone())?;
///     Ok(module)
/// }
/// ```
pub struct FileSystemCache {
    path: PathBuf,
}

impl FileSystemCache {
    /// Construct a new `FileSystemCache` around the specified directory.
    /// The contents of the cache are stored in sub-versioned directories.
    pub fn new<P: Into<PathBuf>>(path: P) -> io::Result<Self> {
        let path: PathBuf = path.into();
        if path.exists() {
            let metadata = path.metadata()?;
            if metadata.is_dir() {
                if !metadata.permissions().readonly() {
                    Ok(Self { path })
                } else {
                    // This directory is readonly.
                    Err(io::Error::new(
                        io::ErrorKind::PermissionDenied,
                        format!("the supplied path is readonly: {}", path.display()),
                    ))
                }
            } else {
                // This path points to a file.
                Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!(
                        "the supplied path already points to a file: {}",
                        path.display()
                    ),
                ))
            }
        } else {
            // Create the directory and any parent directories if they don't yet exist.
            create_dir_all(&path)?;
            Ok(Self { path })
        }
    }
}

impl Cache for FileSystemCache {
    type DeserializeError = IoDeserializeError;
    type SerializeError = IoSerializeError;

    unsafe fn load(&self, store: &Store, key: WasmHash) -> Result<Module, Self::DeserializeError> {
        let filename = key.to_string();
        let mut new_path_buf = self.path.clone();
        new_path_buf.push(filename);

        let file = File::open(new_path_buf)?;
        // Note: this is unsafe, but as the `load` function is unsafe
        // we don't need to wrap it again with the `unsafe { .. }`
        let mmap = Mmap::map(&file)?;

        Ok(Module::deserialize(&store, &mmap[..])?)
    }

    fn store(&mut self, key: WasmHash, module: Module) -> Result<(), Self::SerializeError> {
        let filename = key.to_string();
        let mut new_path_buf = self.path.clone();

        let buffer = module.serialize()?;

        new_path_buf.push(filename);
        let mut file = File::create(new_path_buf)?;
        file.write_all(&buffer)?;

        Ok(())
    }
}
