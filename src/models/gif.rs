use bypar::prelude::UnsizedVec;
use bypar_derive::{FromBytes, ToBytes};

#[derive(Clone, Debug, FromBytes, ToBytes)]
pub struct Gif {
    // magic_part1: u32,
    // magic_part2: u16,
    magic: UnsizedVec<u8, 6>,

    descriptor: LogicalScreenDescriptor,
    global_colour_table: GlobalColourTable,
}

#[derive(Clone, Debug, FromBytes, ToBytes)]
pub struct LogicalScreenDescriptor {
    width: u16,
    height: u16,
    gct: u8,
    background_color: u8,
    pixel_aspect_ratio: u8,
}

#[derive(Clone, Debug, ToBytes)]
pub struct GlobalColourTable {
    entries: UnsizedVec<GlobalColourTableEntry, 255>,
}

impl bypar::FromBytes for GlobalColourTable {
    fn from_bytes(bytes: &[u8]) -> Result<Self, bypar::prelude::Error>
    where
        Self: Sized,
    {
        Ok(Self::from_byte_stream(bytes)?.0)
    }

    fn from_byte_stream(mut bytes: &[u8]) -> Result<(Self, &[u8]), bypar::prelude::Error>
    where
        Self: Sized,
    {
        let mut ret = UnsizedVec::default();
        for _ in 0..255u8 {
            let (entry, remaining_bytes) = GlobalColourTableEntry::from_byte_stream(bytes)?;

            ret.push(entry);

            bytes = remaining_bytes;
        }

        Ok((Self { entries: ret }, bytes))
    }
}

#[derive(Clone, Default, Debug, FromBytes, ToBytes)]
pub struct GlobalColourTableEntry {
    index: u8,
    colour: ColourTableEntry,
}

#[derive(Clone, Default, Debug, FromBytes, ToBytes)]
pub struct ColourTableEntry {
    red: u8,
    green: u8,
    blue: u8,
}
