pub mod informative_table;

use pumpkin_util::math::vector2::Vector2;
use thiserror::Error;

use crate::level::LevelFolder;

use super::{compression::CompressionError, ChunkParsingError};

pub trait RawChunkReader: Sync + Send {
    fn read_raw_chunk(
        &self,
        save_file: &LevelFolder,
        at: &Vector2<i32>,
    ) -> Result<Vec<u8>, RawChunkReadingError>;
}

pub trait RawChunkWriter: Send + Sync {
    fn write_raw_chunk(
        &self,
        chunk: Vec<u8>,
        level_folder: &LevelFolder,
        at: &Vector2<i32>,
    ) -> Result<(), RawChunkWritingError>;
}

#[derive(Error, Debug)]
pub enum RawChunkReadingError {
    #[error("Io error: {0}")]
    IoError(std::io::ErrorKind),
    #[error("Invalid header")]
    InvalidHeader,
    #[error("Region is invalid")]
    RegionIsInvalid,
    #[error("Compression error {0}")]
    Compression(CompressionError),
    #[error("Tried to read chunk which does not exist")]
    ChunkNotExist,
    #[error("Failed to parse Chunk from bytes: {0}")]
    ParsingError(ChunkParsingError),
}

#[derive(Error, Debug)]
pub enum RawChunkWritingError {
    #[error("Io error: {0}")]
    IoError(std::io::ErrorKind),
    #[error("Compression error {0}")]
    Compression(CompressionError),
    #[error("Chunk serializing error: {0}")]
    ChunkSerializingError(String),
}
