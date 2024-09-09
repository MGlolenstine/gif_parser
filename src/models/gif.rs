use bypar::prelude::UnsizedVec;
use bypar_derive::{FromBytes, ToBytes};

#[derive(Clone, Debug, ToBytes)]
pub struct Gif {
    magic: UnsizedVec<u8, 6>,

    descriptor: LogicalScreenDescriptor,
    global_colour_table: GlobalColourTable,
}

impl bypar::FromBytes for Gif {
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
        let mut magic = UnsizedVec::default();

        if bytes.len() < 6 {
            return Err(bypar::prelude::Error::FailedToParse);
        }

        for _ in 0..6 {
            let (element, remaining_bytes) = u8::from_byte_stream(bytes)?;
            bytes = remaining_bytes;
            magic.push(element);
        }

        let (descriptor, remaining_bytes) = LogicalScreenDescriptor::from_byte_stream(bytes)?;
        let (global_colour_table, remaining_bytes) =
            GlobalColourTable::from_byte_stream(remaining_bytes)?;

        Ok((
            Self {
                magic,
                descriptor,
                global_colour_table,
            },
            remaining_bytes,
        ))
    }
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
    magic: u8, // This always has to be 0
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
        let (magic, remaining_bytes) = u8::from_byte_stream(bytes)?;
        bytes = remaining_bytes;

        let mut ret = UnsizedVec::default();
        for colour_index in 0..255u8 {
            let (entry, remaining_bytes) = ColourTableEntry::from_byte_stream(bytes)?;

            ret.push(GlobalColourTableEntry {
                index: colour_index,
                colour: entry,
            });

            bytes = remaining_bytes;
        }

        Ok((
            Self {
                magic,
                entries: ret,
            },
            bytes,
        ))
    }
}

#[derive(Clone, Default, Debug, ToBytes)]
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
