use crate::models::{University, SortType};
use crate::database::init;
use core::cmp::Reverse;
use rocket_sync_db_pools::{postgres,database};
#[database("pkpodb2")]
pub struct DataBase(postgres::Client);

pub struct DataProcessor;


impl DataProcessor{
  fn insert_university(client: &mut postgres::Client,_university:&University) -> bool{
    let mut query = String::from("INSERT INTO pkpo_temp.universities VALUES($1");
    for index in 2..9{
      query.push_str(format!(", ${}",index).as_str());
    }
    query.push_str(");");
    let result = client.execute(query.as_str(),&_university.get_params()[..]);
    match result {
      Ok(_)=> return true,
      Err(err)=>{
        println!("Error: {}",err);
        return false;
      }
    }
  }


  pub fn init(client: &mut postgres::Client) -> Result<(),String> {
    let queries:Vec<&str> = vec![
      "DROP SCHEMA IF EXISTS pkpo_temp CASCADE;",
      "CREATE SCHEMA pkpo_temp;",
      "CREATE TABLE pkpo_temp.universities( 
        index                 INT PRIMARY KEY,
        name                  text,
        location              text,
        rank                  INT,
        description           text,
        tuition_and_fees      text,
        in_state              text,
        undergrad_enrollment  text
      );"];
    for query in queries{
      let result = client.batch_execute(query);
      if result.is_err() {
        return Err(result.err().unwrap().to_string());
      }
    }
    let result = init::init();
    match result{
      Ok(mut r) => {
        for element in r.iter_mut(){
          DataProcessor::insert_university(client,element);
        };
        Ok(())
      },
      Err(err) => {
        println!("Error : {}",err);
        Err(err.to_string())
      }
    }
  }





  pub fn get_universities(client: &mut postgres::Client,sort_by: SortType) -> Result<Vec<University>,postgres::Error>{
    let mut universities:Vec<University> = Vec::<University>::new();
    let mut sort_params = String::from("");
    match sort_by {
      SortType::ByName => sort_params.push_str("ORDER BY name"),
      SortType::ByIndex => sort_params.push_str("ORDER BY index"),
      SortType::ByPrice => sort_params.push_str("")
    }
    let query = String::from(format!("SELECT * FROM pkpo_temp.universities {} ;",sort_params));
    for row in client.query(&query,&[])? {
      let _university = University{
        index                 : row.get(0),
        name                  : row.get(1),
        location              : row.get(2),
        rank                  : row.get(3),
        description           : row.get(4),
        tuition_and_fees      : row.get(5),
        in_state              : row.get(6),
        undergrad_enrollment  : row.get(7)
      };
      universities.push(_university);
    }
    if sort_by == SortType::ByPrice {
      universities.sort_by_key(|university| Reverse(university.get_tuition_and_fees_as_i32()));
    }
    Ok(universities)
  }
}


