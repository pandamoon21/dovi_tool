use bitvec_helpers::{bitvec_reader::BitVecReader, bitvec_writer::BitVecWriter};

#[cfg(feature = "serde_feature")]
use serde::Serialize;

use super::{ExtMetadataBlock, ExtMetadataBlockInfo};

///  Creative intent trim passes per target display peak brightness
#[repr(C)]
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde_feature", derive(Serialize))]
pub struct ExtMetadataBlockLevel8 {
    pub target_display_index: u8,
    pub trim_slope: u16,
    pub trim_offset: u16,
    pub trim_power: u16,
    pub trim_chroma_weight: u16,
    pub trim_saturation_gain: u16,
    pub ms_weight: u16,
}

impl ExtMetadataBlockLevel8 {
    pub fn parse(reader: &mut BitVecReader) -> ExtMetadataBlock {
        ExtMetadataBlock::Level8(Self {
            target_display_index: reader.get_n(8),
            trim_slope: reader.get_n(12),
            trim_offset: reader.get_n(12),
            trim_power: reader.get_n(12),
            trim_chroma_weight: reader.get_n(12),
            trim_saturation_gain: reader.get_n(12),
            ms_weight: reader.get_n(12),
        })
    }

    pub fn write(&self, writer: &mut BitVecWriter) {
        writer.write_n(&self.target_display_index.to_be_bytes(), 8);
        writer.write_n(&self.trim_slope.to_be_bytes(), 12);
        writer.write_n(&self.trim_offset.to_be_bytes(), 12);
        writer.write_n(&self.trim_power.to_be_bytes(), 12);
        writer.write_n(&self.trim_chroma_weight.to_be_bytes(), 12);
        writer.write_n(&self.trim_saturation_gain.to_be_bytes(), 12);
        writer.write_n(&self.ms_weight.to_be_bytes(), 12);
    }
}

impl ExtMetadataBlockInfo for ExtMetadataBlockLevel8 {
    fn level(&self) -> u8 {
        8
    }

    fn bytes_size(&self) -> u64 {
        10
    }

    fn required_bits(&self) -> u64 {
        80
    }

    fn sort_key(&self) -> (u8, u16) {
        (self.level(), self.target_display_index as u16)
    }
}
