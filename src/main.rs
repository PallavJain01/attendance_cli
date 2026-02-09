use app::{App, Commands};
use clap::Parser;

use errors::DataStoreError;
use store::{DataStore, Date, DateRange, Entry, ReadQuery, Subject, WriteQuery};

mod app;
mod errors;
mod store;

fn main() {
  let app: App = App::parse();

  match &app.command {
    Commands::Add { date, subjects } => {
      let date = Date::try_from(date.clone()).expect("Invalid date format");
      let res = DataStore::write(WriteQuery::AtDate(
        date,
        subjects
          .split(",")
          .map(|s| s.trim())
          .map(|s| Subject::try_from(s.to_owned()).unwrap())
          .collect(),
      ));
      if let Err(e) = res {
        panic!("Encountered error while adding new entry: {:?}", e);
      }
      println!("Successfully Added new entry")
    }
    Commands::List { mode } => match mode {
      app::ListMode::All => {
        handle_print_list(DataStore::read(ReadQuery::All));
      }
      app::ListMode::Date { date } => {
        let date = Date::try_from(date.clone()).expect("Invalid date format");
        handle_print_list(DataStore::read(ReadQuery::ByDate(date)));
      }
      app::ListMode::Range { range } => {
        let range = DateRange::try_from(range.clone()).expect("Invalid range format");
        handle_print_list(DataStore::read(ReadQuery::ByRange(range)));
      }
      app::ListMode::Subject { subject } => {
        let subject = Subject::try_from(subject.to_owned());
        match subject {
          Ok(sub) => {
            handle_print_list(DataStore::read(ReadQuery::BySubject(sub)));
          }
          Err(e) => {
            panic!("Encountered error while listing by subject: {:?}", e)
          }
        }
      }
    },
  }
  fn handle_print_list(res: Result<Vec<Entry>, DataStoreError>) {
    match res {
      Ok(list) => {
        println!("{:#?}", list);
      }
      Err(e) => {
        panic!("Encountered error while listing entries: {:?}", e);
      }
    }
  }
}
