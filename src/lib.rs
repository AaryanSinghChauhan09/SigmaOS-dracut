// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dracut: dracut component model.
//!
//! Real implementation: dracut_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DracutItem {
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
pub enum DracutError {
    EmptyDracutItemName,
    DuplicateDracutItem(String),
    EmptyEventSubject,
}

impl fmt::Display for DracutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDracutItemName => write!(f, "dracut_item name must not be empty"),
            Self::DuplicateDracutItem(id) => write!(f, "duplicate dracut_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DracutError {}

pub struct Dracut {
    pub name: String,
    dracut_items: BTreeMap<String, DracutItem>,
    events: BTreeMap<String, Event>,
}

impl Dracut {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            dracut_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_dracut_item(&mut self, item: DracutItem) -> Result<(), DracutError> {
        if item.name.trim().is_empty() {
            return Err(DracutError::EmptyDracutItemName);
        }
        let id = item.id.clone();
        if self.dracut_items.contains_key(&id) {
            return Err(DracutError::DuplicateDracutItem(id.clone()));
        }
        self.dracut_items.insert(id, item);
        Ok(())
    }

    pub fn dracut_item_count(&self) -> usize {
        self.dracut_items.len()
    }

    pub fn get_dracut_item(&self, id: &str) -> Option<&DracutItem> {
        self.dracut_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DracutError> {
        if event.subject.trim().is_empty() {
            return Err(DracutError::EmptyEventSubject);
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

    fn sample_dracut_item(id: &str, name: &str) -> DracutItem {
        DracutItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Dracut::new("Dracut");
        assert_eq!(m.name, "Dracut");
        assert_eq!(m.dracut_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_dracut_item() {
        let mut m = Dracut::new("Dracut");
        m.add_dracut_item(sample_dracut_item("a1", "Alpha")).unwrap();
        assert_eq!(m.dracut_item_count(), 1);
        assert!(m.get_dracut_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Dracut::new("Dracut");
        assert!(m.add_dracut_item(sample_dracut_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Dracut::new("Dracut");
        m.add_dracut_item(sample_dracut_item("x", "One")).unwrap();
        assert!(m.add_dracut_item(sample_dracut_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Dracut::new("Dracut");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
