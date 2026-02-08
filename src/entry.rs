use serde::{Deserialize, Serialize};
use std::fs;

pub enum DataStoreError {
  RonSerializeFail,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
  pub date: String,
  pub subjects: Vec<String>,
}

impl Entry {
  pub fn new(date: String, subjects: Vec<String>) -> Self {
    Self { date, subjects }
  }

  pub fn read_from_store(date: &str) -> Option<Self> {
    let store_all: Result<Vec<Entry>, ()> = Entry::read_from_store_all();
    let entry = if let Ok(store) = store_all {
      store.into_iter().find(|s| s.date == date)
    } else {
      None
    };
    entry
  }

  pub fn read_from_store_range(start_date: String, end_date: String) -> Vec<Self> {
    todo!("read a range of entries from store");
  }

  pub fn read_from_store_all() -> Result<Vec<Self>, ()> {
    let content = include_str!("../store.ron");
    let content = ron::de::from_str::<Vec<Entry>>(content);
    match content {
      Ok(c) => Ok(c),
      Err(e) => Err(()),
    }
  }

  pub fn write_to_store(store: Self) {}

  pub fn write_to_store_all(store: Vec<Self>) -> Result<(), DataStoreError> {
    let existing_store = Entry::read_from_store_all();
    match existing_store {
      Ok(s) => {
        // s.iter().find(|&s| s.date == );
        let new_store = s.iter().chain(&store).collect::<Vec<&Self>>();
        let new_store_sorted = &mut new_store.clone();
        new_store_sorted.sort_by_key(|s| &s.date);

        let _ = fs::write(
          "./store.ron",
          ron::ser::to_string_pretty(
            // &store,
            &new_store_sorted,
            ron::ser::PrettyConfig::default(),
          )
          .expect("Ron serialize failed"),
        );
        return Ok(());
      }
      Err(e) => {
        let _ = fs::write(
          "./store.ron",
          ron::ser::to_string_pretty(
            // &store,
            &store,
            ron::ser::PrettyConfig::default(),
          )
          .expect("Ron serialize failed"),
        );
      }
    };
    return Err(DataStoreError::RonSerializeFail);
  }

  pub fn entry_exists(date: String) -> bool {
    todo!("check if entry exists")
  }
}

// TODO: convert the `read_from_store_*` to a single one, and use enum for `Action` as parameter

//TODO: have multiple functions for building the Vec<DataStore> and one function to write it all to store.ron
