use crate::db::Entry;

/// An entry's history
#[derive(Debug, Default, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize))]
pub struct History {
    pub(crate) entries: Vec<Entry>,
}

impl History {
    /// Add a new entry to the end of the history.
    ///
    /// KeePass keeps the history oldest first, so the newest version is the
    /// last one; this is also the order in the file.
    pub fn add_entry(&mut self, mut entry: Entry) {
        // Remove the history from the new history entry to avoid having
        // an exponential number of history entries.
        entry.history.take();

        self.entries.push(entry);
    }

    /// Get the history entries, oldest first.
    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}
