use serde::{Deserialize, Serialize};

use crate::errors::*;

/* #region Date */
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(into = "String", try_from = "String")]
pub struct Date {
  year: u16, // 2026
  month: u8, // 12
  day: u8,   // 01
}

impl From<Date> for String {
  fn from(date: Date) -> Self {
    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
  }
}

impl TryFrom<String> for Date {
  type Error = String;

  fn try_from(date: String) -> Result<Self, Self::Error> {
    let mut parts = date.split('-');

    let year: u16 = parts
      .next()
      .ok_or("missing year")?
      .parse()
      .map_err(|_| "invalid year")?;

    let month: u8 = parts
      .next()
      .ok_or("missing month")?
      .parse()
      .map_err(|_| "invalid month")?;

    let day: u8 = parts
      .next()
      .ok_or("missing day")?
      .parse()
      .map_err(|_| "invalid day")?;

    if parts.next().is_some() {
      return Err("too many components in date".into());
    }

    if !(1..=12).contains(&month) {
      return Err("month must be 1..=12".into());
    }

    if !(1..=31).contains(&day) {
      return Err("day must be 1..=31".into());
    }

    Ok(Date { year, month, day })
  }
}
/* #endregion */

/* #region DateRange */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct DateRange {
  pub start: Date,
  pub end: Date,
}

impl DateRange {
  pub fn new(start: Date, end: Date) -> Result<Self, String> {
    if start > end {
      return Err("Start date must not be after the end date".to_string());
    };
    Ok(Self { start, end })
  }
  pub fn contains(&self, date: Date) -> bool {
    self.start <= date && date <= self.end
  }
}

impl TryFrom<String> for DateRange {
  type Error = String;
  fn try_from(value: String) -> Result<Self, Self::Error> {
    let (start_str, end_str) = value
      .split_once("..")
      .ok_or("date range must be in format YYYY-MM-DD..YYYY-MM-DD")?;

    let start = Date::try_from(start_str.to_string())?;
    let end = Date::try_from(end_str.to_string())?;

    DateRange::new(start, end)
  }
}

impl From<DateRange> for String {
  fn from(range: DateRange) -> Self {
    let start: String = range.start.into();
    let end: String = range.end.into();
    format!("{start}..{end}")
  }
}
/* #endregion */

/* #region Subject */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub enum Subject {
  Dms,
  Tc,
  Mpi,
  Dbms,
  Toc,
  Dccn,
  MpiLab,
  DbmsLab,
  NpLab,
  LinuxLab,
  JavaLab,
}

impl From<Subject> for String {
  fn from(value: Subject) -> Self {
    match &value {
      Subject::Dms => "Dms".to_string(),
      Subject::Tc => "Tc".to_string(),
      Subject::Mpi => "Mpi".to_string(),
      Subject::Dbms => "Dbms".to_string(),
      Subject::Toc => "Toc".to_string(),
      Subject::Dccn => "Dccn".to_string(),
      Subject::MpiLab => "MpiLab".to_string(),
      Subject::DbmsLab => "DbmsLab".to_string(),
      Subject::NpLab => "NpLab".to_string(),
      Subject::LinuxLab => "LinuxLab".to_string(),
      Subject::JavaLab => "JavaLab".to_string(),
    }
  }
}

impl TryFrom<String> for Subject {
  type Error = String;

  fn try_from(value: String) -> Result<Subject, Self::Error> {
    let val = value.as_str();
    match val {
      "Dms" => Ok(Subject::Dms),
      "Tc" => Ok(Subject::Tc),
      "Mpi" => Ok(Subject::Mpi),
      "Dbms" => Ok(Subject::Dbms),
      "Toc" => Ok(Subject::Toc),
      "Dccn" => Ok(Subject::Dccn),
      "MpiLab" => Ok(Subject::MpiLab),
      "DbmsLab" => Ok(Subject::DbmsLab),
      "NpLab" => Ok(Subject::NpLab),
      "LinuxLab" => Ok(Subject::LinuxLab),
      "JavaLab" => Ok(Subject::JavaLab),
      _ => Err("Subject not found".to_owned()),
    }
  }
}
/* #endregion */

pub enum ReadQuery {
  All,
  ByDate(Date),
  ByRange(DateRange),
  BySubject(Subject),
}

pub enum WriteQuery {
  All(Vec<Entry>),
  AtDate(Date, Vec<Subject>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
  pub date: Date,
  pub subjects: Vec<Subject>,
}

/* #region DataStore */
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataStore(Vec<Entry>);

impl DataStore {
  pub fn load(path: &str) -> Result<DataStore, DataStoreError> {
    match std::fs::read(path) {
      Ok(content) => ron::de::from_bytes(&content).map_err(DataStoreError::RonSpannedError),
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(DataStore(Vec::new())),
      Err(e) => Err(DataStoreError::IOError(e)),
    }
  }

  pub fn read(query: ReadQuery) -> Result<Vec<Entry>, DataStoreError> {
    let content = DataStore::load("./store.ron")?;

    match query {
      ReadQuery::All => Ok(content.0),
      ReadQuery::ByDate(date) => Ok(
        content
          .0
          .into_iter()
          .filter(|s| s.date == date)
          .collect::<Vec<Entry>>(),
      ),
      ReadQuery::ByRange(range) => Ok(
        content
          .0
          .into_iter()
          .filter(|entry| entry.date >= range.start && entry.date <= range.end)
          .collect::<Vec<Entry>>(),
      ),
      ReadQuery::BySubject(subject) => Ok(
        content
          .0
          .into_iter()
          .filter(|s| s.subjects.contains(&subject))
          .collect::<Vec<Entry>>(),
      ),
    }
  }

  pub fn write(query: WriteQuery) -> Result<(), DataStoreError> {
    let mut existing_store = DataStore::read(ReadQuery::All)?;
    match query {
      WriteQuery::All(items) => {
        existing_store = items;
      }
      WriteQuery::AtDate(date, subjects) => {
        if let Some(entry) = existing_store.iter_mut().find(|e| e.date == date) {
          entry.subjects.extend(subjects);
          entry.subjects.dedup();
        } else {
          existing_store.push(Entry { date, subjects });
        }
      }
    }

    existing_store.sort_by_key(|s| s.date);
    let res = ron::ser::to_string_pretty(
      &existing_store,
      ron::ser::PrettyConfig::default().indentor("  "),
    );
    if let Err(e) = res {
      return Err(DataStoreError::RonError(e));
    }
    let res = std::fs::write("./store.ron", res.unwrap());

    match res {
      Ok(()) => Ok(()),
      Err(e) => Err(DataStoreError::IOError(e)),
    }
  }
}
/* #endregion */
