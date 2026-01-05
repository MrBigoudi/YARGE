use crate::log_warn;

/// The representation of an index in a generational indices structure
pub(crate) type GenerationalIndex = usize;

/// The representation of a generation in a generational indices structure
pub(crate) type GenerationalGeneration = u64;

/// The representation of a generational index
#[derive(Debug, Clone, Copy)]
pub struct GenerationalKey {
    /// The index
    pub index: GenerationalIndex,
    /// The generation
    pub generation: GenerationalGeneration,
}

pub enum Entry<T> {
    Free { next_free: GenerationalIndex },
    Occupied { value: T },
}

/// The representation of a generational structure entry
pub struct GenerationalEntry<T> {
    /// The stored entry
    pub entry: Entry<T>,
    /// The entry generation used to check the validity of the entry
    pub generation: GenerationalGeneration,
}

/// A generational indices collection
pub struct GenerationalVec<T> {
    /// The list of entries
    pub entries: Vec<GenerationalEntry<T>>,
    /// The first free entry
    pub free_head: GenerationalIndex,
}

impl<T> GenerationalVec<T> {
    /// Initiates a generational indices list
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_head: 0,
        }
    }

    /// Inserts a new element to the list
    pub fn insert(&mut self, value: T) -> GenerationalKey {
        match self.entries.get_mut(self.free_head) {
            Some(GenerationalEntry { entry, generation }) => {
                // Update
                if let Entry::Free { next_free } = entry {
                    let key = GenerationalKey {
                        index: self.free_head,
                        generation: *generation,
                    };
                    self.free_head = *next_free;
                    *entry = Entry::Occupied { value };
                    key
                } else {
                    panic!("Failed to insert a new entry in a generational indices list");
                }
            }
            None => {
                // Insert
                let generation = 0;
                let key = GenerationalKey {
                    index: self.entries.len(),
                    generation,
                };
                let entry = Entry::Occupied { value };
                let gen_entry = GenerationalEntry { entry, generation };
                self.entries.push(gen_entry);
                self.free_head = key.index + 1;
                key
            }
        }
    }

    /// Removes an element from the list
    pub fn remove(&mut self, key: &GenerationalKey) {
        match self.entries.get_mut(key.index) {
            None => {
                log_warn!(
                    "Trying to remove a non existing entry in a generational indices structure"
                );
            }
            Some(GenerationalEntry { entry, generation }) => match entry {
                Entry::Free { .. } => {
                    log_warn!(
                        "Trying to remove an already removed entry in a generational indices structure"
                    );
                }
                Entry::Occupied { .. } => {
                    if *generation != key.generation {
                        log_warn!(
                            "Trying to remove an older generation in a generational indices structure"
                        );
                    } else {
                        *generation += 1;
                        *entry = Entry::Free {
                            next_free: self.free_head,
                        };
                        self.free_head = key.index;
                    }
                }
            },
        }
    }

    /// Gets an element in the list
    pub fn get(&self, key: &GenerationalKey) -> Option<&T> {
        match self.entries.get(key.index) {
            None => {
                log_warn!(
                    "Trying to access a non existing entry in a generational indices structure"
                );
                None
            }
            Some(GenerationalEntry { entry, generation }) => match entry {
                Entry::Free { .. } => {
                    log_warn!(
                        "Trying to access an empty entry in a generational indices structure"
                    );
                    None
                }
                Entry::Occupied { value } => {
                    if *generation != key.generation {
                        log_warn!(
                            "Trying to access an older generation entry in a generational indices structure"
                        );
                        None
                    } else {
                        Some(value)
                    }
                }
            },
        }
    }

    /// Mutable getter
    pub fn get_mut(&mut self, key: &GenerationalKey) -> Option<&mut T> {
        match self.entries.get_mut(key.index) {
            None => {
                log_warn!(
                    "Trying to access a non existing entry in a generational indices structure"
                );
                None
            }
            Some(GenerationalEntry { entry, generation }) => match entry {
                Entry::Free { .. } => {
                    log_warn!(
                        "Trying to access an empty entry in a generational indices structure"
                    );
                    None
                }
                Entry::Occupied { value } => {
                    if *generation != key.generation {
                        log_warn!(
                            "Trying to access an older generation entry in a generational indices structure"
                        );
                        None
                    } else {
                        Some(value)
                    }
                }
            },
        }
    }
}
