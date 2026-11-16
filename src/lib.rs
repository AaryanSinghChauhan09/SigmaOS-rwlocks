// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-rwlocks: rwlocks component model.
//!
//! Real implementation: rwlocks_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RwlocksItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RwlocksError {
    EmptyRwlocksItemName,
    DuplicateRwlocksItem(String),
    EmptyEventSubject,
}

impl fmt::Display for RwlocksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRwlocksItemName => write!(f, "rwlocks_item name must not be empty"),
            Self::DuplicateRwlocksItem(id) => write!(f, "duplicate rwlocks_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RwlocksError {}

pub struct Rwlocks {
    pub name: String,
    rwlocks_items: BTreeMap<String, RwlocksItem>,
    events: BTreeMap<String, Event>,
}

impl Rwlocks {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rwlocks_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_rwlocks_item(&mut self, item: RwlocksItem) -> Result<(), RwlocksError> {
        if item.name.trim().is_empty() {
            return Err(RwlocksError::EmptyRwlocksItemName);
        }
        let id = item.id.clone();
        if self.rwlocks_items.contains_key(&id) {
            return Err(RwlocksError::DuplicateRwlocksItem(id.clone()));
        }
        self.rwlocks_items.insert(id, item);
        Ok(())
    }

    pub fn rwlocks_item_count(&self) -> usize {
        self.rwlocks_items.len()
    }

    pub fn get_rwlocks_item(&self, id: &str) -> Option<&RwlocksItem> {
        self.rwlocks_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RwlocksError> {
        if event.subject.trim().is_empty() {
            return Err(RwlocksError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rwlocks_item(id: &str, name: &str) -> RwlocksItem {
        RwlocksItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Rwlocks::new("Rwlocks");
        assert_eq!(m.name, "Rwlocks");
        assert_eq!(m.rwlocks_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_rwlocks_item() {
        let mut m = Rwlocks::new("Rwlocks");
        m.add_rwlocks_item(sample_rwlocks_item("a1", "Alpha")).unwrap();
        assert_eq!(m.rwlocks_item_count(), 1);
        assert!(m.get_rwlocks_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Rwlocks::new("Rwlocks");
        assert!(m.add_rwlocks_item(sample_rwlocks_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Rwlocks::new("Rwlocks");
        m.add_rwlocks_item(sample_rwlocks_item("x", "One")).unwrap();
        assert!(m.add_rwlocks_item(sample_rwlocks_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Rwlocks::new("Rwlocks");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
