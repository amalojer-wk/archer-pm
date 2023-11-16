use crate::error::{APMError, APMErrorType};

use std::fs::{File, OpenOptions};
use std::io::{copy, Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

pub fn compress_directory(
    path: &str,
    track_file_names: bool,
    dont_strip_base_dir: bool,
) -> Result<(Vec<u8>, Option<Vec<(String, String)>>), APMError> {
    let mut buffer = Vec::new();
    let options = FileOptions::default();
    let mut zip_writer = ZipWriter::new(Cursor::new(&mut buffer));
    let mut file_names = {
        if track_file_names {
            Some(Vec::new())
        } else {
            None
        }
    };

    for entry in WalkDir::new(path).into_iter() {
        let entry = entry.map_err(|e| APMErrorType::WalkdirError.into_apm_error(e.to_string()))?;

        let name = entry.path().display().to_string();

        // Skip the current directory
        if name == path {
            continue;
        }

        let stripped_file_name = if dont_strip_base_dir {
            name.clone()
        } else {
            entry
                .path()
                .strip_prefix(path)
                .map(|p| p.display().to_string())
                .unwrap_or(name.clone())
        };

        if entry.file_type().is_symlink() {
            return Err(APMErrorType::SymlinkFoundError.into_apm_error(format!(
                "Found symlink at path {}\nSymlinks cannot be compressed.",
                entry.file_name().to_str().unwrap_or("PATH_UNKNOWN")
            )));
        } else if entry.file_type().is_dir() {
            zip_writer
                .add_directory(&stripped_file_name, options)
                .map_err(|e| APMErrorType::ZIPAddDirectoryError.into_apm_error(e.to_string()))?;

            if let Some(file_names) = &mut file_names {
                file_names.push((name, stripped_file_name));
            }
        } else if entry.file_type().is_file() {
            add_file_to_archive(&mut zip_writer, &name, &stripped_file_name, Some(options))?;

            if let Some(file_names) = &mut file_names {
                file_names.push((name, stripped_file_name));
            }
        }
    }

    zip_writer
        .finish()
        .map_err(|e| APMErrorType::ZIPFinishError.into_apm_error(e.to_string()))?;

    drop(zip_writer);

    return Ok((buffer, file_names));
}

pub fn read_archive(path: &str) -> Result<ZipArchive<File>, APMError> {
    let f = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .map_err(|e| APMErrorType::FileOpenError.into_apm_error(e.to_string()))?;

    return Ok(ZipArchive::new(f)
        .map_err(|e| APMErrorType::ZIPArchiveOpenError.into_apm_error(e.to_string()))?);
}

pub fn read_archive_from_bytes(bytes: &[u8]) -> Result<ZipArchive<Cursor<&[u8]>>, APMError> {
    return ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| APMErrorType::ZIPArchiveOpenError.into_apm_error(e.to_string()));
}

pub fn add_file_to_archive<A: Read + Seek + Write, P: AsRef<Path>>(
    archive: &mut ZipWriter<A>,
    file_path: P,
    file_name: &str,
    options: Option<FileOptions>,
) -> Result<(), APMError> {
    let options = options.unwrap_or(FileOptions::default());

    let mut f = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .map_err(|e| {
            APMErrorType::FileOpenError.into_apm_error(format!(
                "{}\nFile:{}",
                e.to_string(),
                file_path.as_ref().display()
            ))
        })?;

    archive
        .start_file(file_name, options)
        .map_err(|e| APMErrorType::ZIPStartFileError.into_apm_error(e.to_string()))?;

    copy(&mut f, archive)
        .map_err(|e| APMErrorType::ZIPFileCopyError.into_apm_error(e.to_string()))?;

    return Ok(());
}

pub fn extract_file_from_archive<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
    name: &str,
) -> Result<Vec<u8>, APMError> {
    let mut f = archive
        .by_name(name)
        .map_err(|e| APMErrorType::ZIPArchiveFileFindError.into_apm_error(e.to_string()))?;

    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .map_err(|e| APMErrorType::ZIPFileReadError.into_apm_error(e.to_string()))?;

    return Ok(buf);
}
