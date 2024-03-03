//! Staticly available information and information generated using the build script that scrapes the downloaded html 
//! tables.

use std::{collections::HashSet, iter::FusedIterator};
use enumflags2::BitFlags;
use serde::{Serialize, Deserialize};
use super::generated::{LANE_DATA, ALL_SKINSET_NAMES, CHAMPS_TO_SKINSETS};

/// Re-export the lane enum.
pub use crate::generated::Lane;

/// The ID used to refer to a league of legends champ.
/// Under the hood this is just an index into the [LANE_DATA] constant. 
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ChampId(usize);

impl ChampId {
    /// The highest valid champion ID.
    pub const MAX: Self = ChampId(LANE_DATA.len() -  1);

    /// Return an iterator over all the valid [ChampId]s. 
    pub fn iter_all() -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator + FusedIterator {
        (0..LANE_DATA.len()).map(ChampId)
    }

    /// Get the name of the champion that this ID refers to. 
    #[inline]
    pub const fn champ_name(self) -> &'static str {
        LANE_DATA[self.0].0
    }

    /// Get the default lanes of the champion that this ID refers to.
    #[inline]
    pub const fn default_lanes(self) -> BitFlags<Lane> {
        LANE_DATA[self.0].1
    }

    /// Get an iterator over all the skinsets available for the champ referred to by this ID.
    pub fn skinsets(self) -> impl DoubleEndedIterator<Item = SkinsetId> + ExactSizeIterator + FusedIterator {
        CHAMPS_TO_SKINSETS[self.0].iter().map(|index: &usize| SkinsetId(*index))
    }
}


/// The ID used to refer to a league of legends skinset. 
/// Under the hood this is just an index into the [ALL_SKINSET_NAMES] constant. 
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct SkinsetId(usize);

impl SkinsetId {
    /// The highest valid [SkinsetId].
    pub const MAX: Self = SkinsetId(ALL_SKINSET_NAMES.len() - 1);

    /// The "Legacy" skinset, which should be excluded by default. 
    const LEGACY: Self = Self::id_of_skinset("Legacy");

    /// The "N/A" skinset which shoudl be excluded by default. 
    const NA: Self = Self::id_of_skinset("N/A");

    /// The skinsets excluded by default because they are not visually cohesive. 
    pub const DEFAULT_EXCLUDED_SKINSETS: &'static [SkinsetId] = &[Self::LEGACY, Self::NA];

    /// Compile-time function to get the index of a skinset by name. 
    const fn id_of_skinset(skinset_name: &'static str) -> Self {
        let mut index: usize = 0;

        while index < ALL_SKINSET_NAMES.len() {
            // Work-around for string equality not being available in const contexts.
            if const_str::equal!(ALL_SKINSET_NAMES[index], skinset_name) {
                return SkinsetId(index);
            }
            index += 1;
        }

        // Panic here if we don't find it -- we cannot use nice format args to make a nice panic message so just do 
        // this.
        panic!("BAD SKINSET ID PASSED TO CONST FN");
    }

    /// Get an iterator over all the valid [SkinsetId]s. 
    pub fn iter_all() -> impl DoubleEndedIterator<Item = Self> + FusedIterator + ExactSizeIterator {
        (0..ALL_SKINSET_NAMES.len()).map(SkinsetId)
    }

    /// Get the name of the skinset this ID refers to. 
    #[inline]
    pub const fn skinset_name(self) -> &'static str {
        ALL_SKINSET_NAMES[self.0]
    }

    /// Generate the default set of all included skinsets. 
    pub fn generate_default_included_skinsets() -> HashSet<SkinsetId> {
        SkinsetId::iter_all()
            .filter(|skinset_id| !SkinsetId::DEFAULT_EXCLUDED_SKINSETS.contains(skinset_id))
            .collect()
    }

    /// Get the underlying [usize] for this [SkinsetId].
    pub const fn inner(self) -> usize {
        self.0
    }
}
