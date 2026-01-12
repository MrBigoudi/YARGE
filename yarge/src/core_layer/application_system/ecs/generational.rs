#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// The representation of an index in a generational indices structure
pub(crate) type GenerationalIndex = usize;

/// The representation of a generation in a generational indices structure
pub(crate) type GenerationalGeneration = u64;

/// The representation of a generational index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationalKey {
    /// The index
    pub(crate) index: GenerationalIndex,
    /// The generation
    pub(crate) generation: GenerationalGeneration,
}

pub(crate) enum Entry<T> {
    Free { next_free: GenerationalIndex },
    Occupied { value: Option<T> },
}

/// The representation of a generational structure entry
pub(crate) struct GenerationalEntry<T> {
    /// The stored entry
    pub(crate) entry: Entry<T>,
    /// The entry generation used to check the validity of the entry
    pub(crate) generation: GenerationalGeneration,
}

/// A generational indices collection
pub(crate) struct GenerationalVec<T> {
    /// The list of entries
    pub(crate) entries: Vec<GenerationalEntry<T>>,
    /// The first free entry
    pub(crate) free_head: GenerationalIndex,
}

impl<T> GenerationalVec<T> {
    #[allow(unused)]
    /// Initializes an generational indices list
    pub(crate) fn init_empty() -> Self {
        Self {
            entries: Vec::new(),
            free_head: 0,
        }
    }

    #[allow(unused)]
    /// Initializes a generational indices list filled with empty entries
    pub(crate) fn init_filled_with_empty_entries(nb_new_entries: usize) -> Result<Self, ErrorType> {
        let mut new_list = Self::init_empty();
        if nb_new_entries > 0
            && let Err(err) = new_list.insert_empty_entries(nb_new_entries, false)
        {
            log_error!(
                "Failed to initialize a new filled generational indices list: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        Ok(new_list)
    }

    #[allow(unused)]
    /// Inserts many empty elements to the list
    pub(crate) fn insert_empty_entries(
        &mut self,
        nb_new_entries: usize,
        should_return_new_keys: bool,
    ) -> Result<Option<Vec<GenerationalKey>>, ErrorType> {
        if nb_new_entries == 0 {
            log_error!("Can't create 0 new empty entries in a generational list");
            return Err(ErrorType::WrongArgument(String::from(
                "Valid parameter must be greater or equal to 1",
            )));
        }
        let mut next_free = self.free_head;
        let mut new_keys = vec![];
        let mut nb_added_entries = 0;
        // Fill up the holes
        'loop_on_free: while nb_added_entries < nb_new_entries
            && let Some(GenerationalEntry { entry, generation }) = self.entries.get_mut(next_free)
        {
            if let Entry::Free {
                next_free: next_next_free,
            } = entry
            {
                let next_next_free = *next_next_free;
                *entry = Entry::Occupied { value: None };
                *generation += 1;
                if should_return_new_keys {
                    let new_key = GenerationalKey {
                        index: next_free,
                        generation: *generation,
                    };
                    new_keys.push(new_key);
                }
                next_free = next_next_free;
                nb_added_entries += 1;
            } else {
                break 'loop_on_free;
            }
        }

        // Sanity check
        if self.entries.len() < next_free {
            log_error!(
                "Failed to insert empty entries in a generational list: expected at most `{:?}' for the next free, got `{:?}'",
                self.entries.len(),
                next_free
            );
            return Err(ErrorType::Unknown);
        }

        // Add the other entries
        let mut new_entries: Vec<GenerationalEntry<T>> = (nb_added_entries..nb_new_entries)
            .map(|_| {
                let new_entry = GenerationalEntry {
                    entry: Entry::Occupied { value: None },
                    generation: 0,
                };
                if should_return_new_keys {
                    let new_key = GenerationalKey {
                        index: next_free,
                        generation: 0,
                    };
                    new_keys.push(new_key);
                }
                next_free += 1;
                new_entry
            })
            .collect();

        self.entries.append(&mut new_entries);

        self.free_head = next_free;

        if should_return_new_keys {
            Ok(Some(new_keys))
        } else {
            Ok(None)
        }
    }

    #[allow(unused)]
    /// Inserts a new element to the list
    /// Returns the newly associated key
    pub(crate) fn insert(&mut self, value: T) -> Result<GenerationalKey, ErrorType> {
        match self.entries.get_mut(self.free_head) {
            Some(GenerationalEntry { entry, generation }) => {
                // Update
                if let Entry::Free { next_free } = entry {
                    let key = GenerationalKey {
                        index: self.free_head,
                        generation: *generation,
                    };
                    self.free_head = *next_free;
                    *entry = Entry::Occupied { value: Some(value) };
                    Ok(key)
                } else {
                    log_error!("Failed to insert a new entry in a generational indices list");
                    Err(ErrorType::Unknown)
                }
            }
            None => {
                // Insert
                let generation = 0;
                let key = GenerationalKey {
                    index: self.entries.len(),
                    generation,
                };
                let entry = Entry::Occupied { value: Some(value) };
                let gen_entry = GenerationalEntry { entry, generation };
                self.entries.push(gen_entry);
                self.free_head = key.index + 1;
                Ok(key)
            }
        }
    }

    #[allow(unused)]
    /// Removes an element from the list
    pub(crate) fn remove(&mut self, key: &GenerationalKey) -> Result<(), ErrorType> {
        match self.entries.get_mut(key.index) {
            None => {
                log_warn!(
                    "Trying to remove a non existing entry in a generational indices structure"
                );
                Err(ErrorType::DoesNotExist)
            }
            Some(GenerationalEntry { entry, generation }) => match entry {
                Entry::Free { .. } => {
                    log_warn!(
                        "Trying to remove an already removed entry in a generational indices structure"
                    );
                    Err(ErrorType::DoesNotExist)
                }
                Entry::Occupied { .. } => {
                    if *generation != key.generation {
                        log_warn!(
                            "Trying to remove an older generation in a generational indices structure"
                        );
                        Err(ErrorType::DoesNotExist)
                    } else {
                        *generation += 1;
                        *entry = Entry::Free {
                            next_free: self.free_head,
                        };
                        self.free_head = key.index;
                        Ok(())
                    }
                }
            },
        }
    }

    #[allow(unused)]
    /// Gets the value of an entry in the list
    pub(crate) fn get_value(&self, key: &GenerationalKey) -> Result<Option<&T>, ErrorType> {
        match self.entries.get(key.index) {
            None => {
                log_error!(
                    "Trying to access a non existing entry in a generational indices structure"
                );
                Err(ErrorType::InvalidIndex)
            }
            Some(GenerationalEntry { entry, generation }) => match entry {
                Entry::Free { .. } => {
                    log_error!(
                        "Trying to access an empty entry in a generational indices structure"
                    );
                    Err(ErrorType::InvalidIndex)
                }
                Entry::Occupied { value } => {
                    if *generation != key.generation {
                        log_error!(
                            "Trying to access an older generation entry in a generational indices structure"
                        );
                        Err(ErrorType::InvalidIndex)
                    } else {
                        Ok(value.as_ref())
                    }
                }
            },
        }
    }

    #[allow(unused)]
    /// Mutable getter to a value of an entry in the list
    pub(crate) fn get_mut_value(
        &mut self,
        key: &GenerationalKey,
    ) -> Result<Option<&mut T>, ErrorType> {
        match self.entries.get_mut(key.index) {
            None => {
                log_error!(
                    "Trying to access a non existing entry in a generational indices structure"
                );
                Err(ErrorType::InvalidIndex)
            }
            Some(GenerationalEntry { entry, generation }) => match entry {
                Entry::Free { .. } => {
                    log_error!(
                        "Trying to access an empty entry in a generational indices structure"
                    );
                    Err(ErrorType::InvalidIndex)
                }
                Entry::Occupied { value } => {
                    if *generation != key.generation {
                        log_error!(
                            "Trying to access an older generation entry in a generational indices structure"
                        );
                        Err(ErrorType::InvalidIndex)
                    } else {
                        Ok(value.as_mut())
                    }
                }
            },
        }
    }

    #[allow(unused)]
    /// Gets an entry in the list
    pub(crate) fn get_entry(&self, key: &GenerationalKey) -> Result<&Entry<T>, ErrorType> {
        match self.entries.get(key.index) {
            None => {
                log_error!(
                    "Trying to access a non existing entry in a generational indices structure"
                );
                Err(ErrorType::InvalidIndex)
            }
            Some(GenerationalEntry { entry, .. }) => Ok(entry),
        }
    }

    #[allow(unused)]
    /// Mutable getter to an entry in the list
    pub(crate) fn get_mut_entry(
        &mut self,
        key: &GenerationalKey,
    ) -> Result<&mut Entry<T>, ErrorType> {
        match self.entries.get_mut(key.index) {
            None => {
                log_error!(
                    "Trying to access a non existing entry in a generational indices structure"
                );
                Err(ErrorType::InvalidIndex)
            }
            Some(GenerationalEntry { entry, .. }) => Ok(entry),
        }
    }
}

// TODO: add tests for generational lists
