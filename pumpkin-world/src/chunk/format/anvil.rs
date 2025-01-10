use fastnbt::LongArray;
use indexmap::IndexMap;
use pumpkin_util::math::ceil_log2;
use pumpkin_util::math::vector2::Vector2;
use std::collections::HashSet;

use crate::block::block_registry::BLOCK_ID_TO_REGISTRY_ID;
use crate::chunk::{
    ChunkData, ChunkNbt, ChunkSection, ChunkSectionBlockStates, ChunkStatus, PaletteEntry,
    WORLD_DATA_VERSION,
};

use super::{ChunkReader, ChunkReadingError, ChunkWriter, ChunkWritingError};

#[derive(Clone, Default)]
pub struct AnvilChunkFormat;

impl ChunkReader for AnvilChunkFormat {
    fn read_chunk(
        &self,
        chunk_bytes: Vec<u8>,
        at: &Vector2<i32>,
    ) -> Result<ChunkData, ChunkReadingError> {
        ChunkData::from_bytes(&chunk_bytes, *at).map_err(ChunkReadingError::ParsingError)
    }
}

impl ChunkWriter for AnvilChunkFormat {
    fn write_chunk(
        &self,
        chunk_data: &ChunkData,
        _at: &Vector2<i32>,
    ) -> Result<Vec<u8>, super::ChunkWritingError> {
        let mut sections = Vec::new();

        for (i, blocks) in chunk_data.subchunks.array_iter().enumerate() {
            // get unique blocks
            let unique_blocks: HashSet<_> = blocks.iter().collect();

            let palette: IndexMap<_, _> = unique_blocks
                .into_iter()
                .enumerate()
                .map(|(i, block)| {
                    let name = BLOCK_ID_TO_REGISTRY_ID.get(block).unwrap().as_str();
                    (block, (name, i))
                })
                .collect();

            // Determine the number of bits needed to represent the largest index in the palette
            let block_bit_size = if palette.len() < 16 {
                4
            } else {
                ceil_log2(palette.len() as u32).max(4)
            };

            let mut section_longs = Vec::new();
            let mut current_pack_long: i64 = 0;
            let mut bits_used_in_pack: u32 = 0;

            // Empty data if the palette only contains one index https://minecraft.fandom.com/wiki/Chunk_format
            // if palette.len() > 1 {}
            // TODO: Update to write empty data. Rn or read does not handle this elegantly
            for block in blocks.iter() {
                // Push if next bit does not fit
                if bits_used_in_pack + block_bit_size as u32 > 64 {
                    section_longs.push(current_pack_long);
                    current_pack_long = 0;
                    bits_used_in_pack = 0;
                }
                let index = palette.get(block).expect("Just added all unique").1;
                current_pack_long |= (index as i64) << bits_used_in_pack;
                bits_used_in_pack += block_bit_size as u32;

                assert!(bits_used_in_pack <= 64);

                // If the current 64-bit integer is full, push it to the section_longs and start a new one
                if bits_used_in_pack >= 64 {
                    section_longs.push(current_pack_long);
                    current_pack_long = 0;
                    bits_used_in_pack = 0;
                }
            }

            // Push the last 64-bit integer if it contains any data
            if bits_used_in_pack > 0 {
                section_longs.push(current_pack_long);
            }

            sections.push(ChunkSection {
                y: i as i8 - 4,
                block_states: Some(ChunkSectionBlockStates {
                    data: Some(LongArray::new(section_longs)),
                    palette: palette
                        .into_iter()
                        .map(|entry| PaletteEntry {
                            name: entry.1 .0.to_owned(),
                            properties: None,
                        })
                        .collect(),
                }),
            });
        }

        let nbt = ChunkNbt {
            data_version: WORLD_DATA_VERSION,
            x_pos: chunk_data.position.x,
            z_pos: chunk_data.position.z,
            status: ChunkStatus::Full,
            heightmaps: chunk_data.heightmap.clone(),
            sections,
        };

        fastnbt::to_bytes(&nbt).map_err(|e| ChunkWritingError::ChunkSerializingError(e.to_string()))
    }
}

/*#[cfg(test)]
mod tests {
    use pumpkin_util::math::vector2::Vector2;
    use std::fs;
    use std::path::PathBuf;

    use crate::chunk::ChunkWriter;
    use crate::generation::{get_world_gen, Seed};
    use crate::{
        chunk::{format::anvil::AnvilChunkFormat, ChunkReader, ChunkReadingError},
        level::LevelFolder,
    };

    #[test]
    fn not_existing() {
        let region_path = PathBuf::from("not_existing");
        let result = AnvilChunkFormat.read_chunk(
            &LevelFolder {
                root_folder: PathBuf::from(""),
                region_folder: region_path,
            },
            &Vector2::new(0, 0),
        );
        assert!(matches!(result, Err(ChunkReadingError::ChunkNotExist)));
    }

    #[test]
    fn test_writing() {
        let generator = get_world_gen(Seed(0));
        let level_folder = LevelFolder {
            root_folder: PathBuf::from("./tmp"),
            region_folder: PathBuf::from("./tmp/region"),
        };
        if fs::exists(&level_folder.root_folder).unwrap() {
            fs::remove_dir_all(&level_folder.root_folder).expect("Could not delete directory");
        }

        fs::create_dir_all(&level_folder.region_folder).expect("Could not create directory");

        // Generate chunks
        let mut chunks = vec![];
        for x in -5..5 {
            for y in -5..5 {
                let position = Vector2::new(x, y);
                chunks.push((position, generator.generate_chunk(position)));
            }
        }

        for i in 0..5 {
            println!("Iteration {}", i + 1);
            for (at, chunk) in &chunks {
                AnvilChunkFormat
                    .write_chunk(chunk, &level_folder, at)
                    .expect("Failed to write chunk");
            }

            let mut read_chunks = vec![];
            for (at, _chunk) in &chunks {
                read_chunks.push(
                    AnvilChunkFormat
                        .read_chunk(&level_folder, at)
                        .expect("Could not read chunk"),
                );
            }

            for (at, chunk) in &chunks {
                let read_chunk = read_chunks
                    .iter()
                    .find(|chunk| chunk.position == *at)
                    .expect("Missing chunk");
                assert_eq!(chunk.subchunks, read_chunk.subchunks, "Chunks don't match");
            }
        }

        fs::remove_dir_all(&level_folder.root_folder).expect("Could not delete directory");

        println!("Checked chunks successfully");
    }
}*/
