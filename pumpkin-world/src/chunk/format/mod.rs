pub mod anvil;

use pumpkin_util::math::vector2::Vector2;
use thiserror::Error;

use super::{ChunkData, ChunkParsingError};

pub trait ChunkReader: Sync + Send {
    fn read_chunk(
        &self,
        chunk_bytes: Vec<u8>,
        at: &Vector2<i32>,
    ) -> Result<ChunkData, ChunkReadingError>;
}

pub trait ChunkWriter: Send + Sync {
    fn write_chunk(
        &self,
        chunk_data: &ChunkData,
        at: &Vector2<i32>,
    ) -> Result<Vec<u8>, ChunkWritingError>;
}

#[derive(Error, Debug)]
pub enum ChunkReadingError {
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
pub enum ChunkWritingError {
    #[error("Io error: {0}")]
    IoError(std::io::ErrorKind),
    #[error("Compression error {0}")]
    Compression(CompressionError),
    #[error("Chunk serializing error: {0}")]
    ChunkSerializingError(String),
}

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("Compression scheme not recognised")]
    UnknownCompression,
    #[error("Error while working with zlib compression: {0}")]
    ZlibError(std::io::Error),
    #[error("Error while working with Gzip compression: {0}")]
    GZipError(std::io::Error),
    #[error("Error while working with LZ4 compression: {0}")]
    LZ4Error(std::io::Error),
}
