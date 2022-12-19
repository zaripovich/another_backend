use crate::models::University;
use std::error::Error;
use csv::ReaderBuilder;


pub fn init() -> Result<Vec<University>,Box<dyn Error>>{
  let mut universities: Vec<University> = Vec::<University>::new();
  let mut rdr = ReaderBuilder::new().from_path("data/dataset.csv")?;
  for record in rdr.deserialize(){
      let record_t: University = record?;
      universities.push(record_t);
  }
  return Ok(universities);
}

