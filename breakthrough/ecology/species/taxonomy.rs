// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use super::*;

/// TaxonomicRank: The hierarchical rank of a species in the cognitive taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxonomicRank {
    Domain,
    Kingdom,
    Phylum,
    Class,
    Order,
    Family,
    Genus,
    Species,
}

/// CognitiveTaxonomy: Hierarchical classification system for cognitive species.
pub struct CognitiveTaxonomy {
    pub levels: Vec<TaxonomicLevel>,
    pub root: SpeciesId,
}

impl CognitiveTaxonomy {
    /// Constructs a new CognitiveTaxonomy with the given root species.
    pub fn new(root: SpeciesId) -> Self {
        Self {
            levels: Vec::new(),
            root,
        }
    }

    /// Adds a taxonomic level to the hierarchy.
    ///
    /// Returns an error if a level with the same rank already exists
    /// or if any member ID is already present in another level.
    pub fn add_level(&mut self, rank: TaxonomicRank, members: Vec<SpeciesId>) -> Result<(), SpeciesError> {
        if self.levels.iter().any(|l| l.rank == rank) {
            return Err(SpeciesError::InvalidTaxonomy(format!(
                "Duplicate taxonomic rank {:?}",
                rank
            )));
        }
        for member in &members {
            if self
                .levels
                .iter()
                .any(|l| l.members.contains(member))
            {
                return Err(SpeciesError::InvalidTaxonomy(format!(
                    "SpeciesId {:?} already exists in another level",
                    member.0
                )));
            }
        }
        self.levels.push(TaxonomicLevel { rank, members });
        Ok(())
    }

    /// Returns an immutable reference to the members at the given rank, if present.
    pub fn get_members(&self, rank: TaxonomicRank) -> Option<&Vec<SpeciesId>> {
        self.levels.iter().find(|l| l.rank == rank).map(|l| &l.members)
    }

    /// Returns a mutable reference to the members at the given rank, if present.
    pub fn get_members_mut(&mut self, rank: TaxonomicRank) -> Option<&mut Vec<SpeciesId>> {
        self.levels.iter_mut().find(|l| l.rank == rank).map(|l| &mut l.members)
    }

    /// Finds the taxonomic rank and member index for a given SpeciesId.
    ///
    /// Returns `Some((rank, index))` if found, `None` otherwise.
    pub fn find_species(&self, species_id: SpeciesId) -> Option<(TaxonomicRank, usize)> {
        for level in &self.levels {
            if let Some(idx) = level.members.iter().position(|m| *m == species_id) {
                return Some((level.rank, idx));
            }
        }
        None
    }

    /// Removes a species from all taxonomic levels.
    ///
    /// Returns the rank it was removed from, or `None` if not found.
    pub fn remove_species(&mut self, species_id: SpeciesId) -> Option<TaxonomicRank> {
        for i in 0..self.levels.len() {
            let before = self.levels[i].members.len();
            self.levels[i].members.retain(|m| *m != species_id);
            if self.levels[i].members.len() < before {
                return Some(self.levels[i].rank);
            }
        }
        None
    }

    /// Returns the number of taxonomic levels in the hierarchy.
    pub fn level_count(&self) -> usize {
        self.levels.len()
    }

    /// Validates the taxonomy structure.
    ///
    /// Returns an error if no levels exist, if the root is not the first member
    /// of the domain level, or if there are duplicate members.
    pub fn validate(&self) -> Result<(), SpeciesError> {
        if self.levels.is_empty() {
            return Err(SpeciesError::InvalidTaxonomy(
                "Taxonomy must contain at least one level".to_string(),
            ));
        }
        let mut seen = std::collections::HashSet::new();
        for level in &self.levels {
            for member in &level.members {
                if !seen.insert(member.0) {
                    return Err(SpeciesError::InvalidTaxonomy(format!(
                        "Duplicate SpeciesId {:?} across levels",
                        member.0
                    )));
                }
            }
        }
        if let Some(domain_level) = self.levels.first() {
            if domain_level.rank != TaxonomicRank::Domain {
                return Err(SpeciesError::InvalidTaxonomy(
                    "First level must be Domain".to_string(),
                ));
            }
            if !domain_level.members.contains(&self.root) {
                return Err(SpeciesError::InvalidTaxonomy(
                    "Root species must be in the Domain level".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// TaxonomicLevel: A single level in the taxonomic hierarchy.
pub struct TaxonomicLevel {
    pub rank: TaxonomicRank,
    pub members: Vec<SpeciesId>,
}

impl TaxonomicLevel {
    /// Constructs a new TaxonomicLevel with the given rank and members.
    pub fn new(rank: TaxonomicRank, members: Vec<SpeciesId>) -> Self {
        Self { rank, members }
    }

    /// Adds a member to this level.
    ///
    /// Returns `true` if the member was added, `false` if it already existed.
    pub fn add_member(&mut self, species_id: SpeciesId) -> bool {
        if self.members.contains(&species_id) {
            return false;
        }
        self.members.push(species_id);
        true
    }

    /// Removes a member from this level.
    ///
    /// Returns `true` if the member was removed, `false` if it was not present.
    pub fn remove_member(&mut self, species_id: SpeciesId) -> bool {
        let before = self.members.len();
        self.members.retain(|m| *m != species_id);
        self.members.len() < before
    }

    /// Returns the number of members in this level.
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Checks if a species is a member of this level.
    pub fn contains(&self, species_id: SpeciesId) -> bool {
        self.members.contains(&species_id)
    }

    /// Validates the level: rank must be valid and members must be unique.
    pub fn validate(&self) -> Result<(), SpeciesError> {
        let mut seen = std::collections::HashSet::new();
        for member in &self.members {
            if !seen.insert(member.0) {
                return Err(SpeciesError::InvalidTaxonomy(format!(
                    "Duplicate SpeciesId {:?} in level {:?}",
                    member.0, self.rank
                )));
            }
        }
        Ok(())
    }
}

impl TaxonomicRank {
    /// Returns the string representation of the rank.
    pub fn as_str(&self) -> &'static str {
        match self {
            TaxonomicRank::Domain => "Domain",
            TaxonomicRank::Kingdom => "Kingdom",
            TaxonomicRank::Phylum => "Phylum",
            TaxonomicRank::Class => "Class",
            TaxonomicRank::Order => "Order",
            TaxonomicRank::Family => "Family",
            TaxonomicRank::Genus => "Genus",
            TaxonomicRank::Species => "Species",
        }
    }

    /// Returns the ordinal order of the rank (0 = Domain, 7 = Species).
    pub fn order(&self) -> usize {
        match self {
            TaxonomicRank::Domain => 0,
            TaxonomicRank::Kingdom => 1,
            TaxonomicRank::Phylum => 2,
            TaxonomicRank::Class => 3,
            TaxonomicRank::Order => 4,
            TaxonomicRank::Family => 5,
            TaxonomicRank::Genus => 6,
            TaxonomicRank::Species => 7,
        }
    }

    /// Returns true if `self` is at or above `other` in the hierarchy.
    ///
    /// Domain is the highest (most general) rank; Species is the lowest (most specific).
    pub fn is_above_or_equal(&self, other: TaxonomicRank) -> bool {
        self.order() <= other.order()
    }

    /// Returns true if `self` is strictly below `other` in the hierarchy.
    pub fn is_below(&self, other: TaxonomicRank) -> bool {
        self.order() > other.order()
    }

    /// Returns the next rank down in the hierarchy, if one exists.
    pub fn next_rank(&self) -> Option<TaxonomicRank> {
        let next = self.order() + 1;
        if next > 7 {
            None
        } else {
            Some(match next {
                0 => TaxonomicRank::Domain,
                1 => TaxonomicRank::Kingdom,
                2 => TaxonomicRank::Phylum,
                3 => TaxonomicRank::Class,
                4 => TaxonomicRank::Order,
                5 => TaxonomicRank::Family,
                6 => TaxonomicRank::Genus,
                7 => TaxonomicRank::Species,
                _ => unreachable!(),
            })
        }
    }
}