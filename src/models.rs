use serde::{Deserialize,Serialize};

use rocket_sync_db_pools::postgres;
use strum_macros::EnumIter;


#[derive(Deserialize,PartialEq,EnumIter)]
#[serde(crate = "rocket::serde")]
pub enum SortType{
  ByName,
  ByIndex,
  ByPrice
}


impl TryFrom<i32> for SortType {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
      if value == 0 {
          return Ok(SortType::ByName)
      }
      if value == 1 {
          return Ok(SortType::ByIndex)
      } 
      if value == 2 {
        return Ok(SortType::ByPrice)
    } 
      Err(())
  }
}

impl TryFrom<SortType> for String {
  type Error = ();

  fn try_from(value: SortType) -> Result<Self, Self::Error> {
      if value == SortType::ByName {
          return Ok("ByName".to_string())
      }
      if value == SortType::ByIndex {
          return Ok("ByIndex".to_string())
      }
      if value == SortType::ByPrice {
        return Ok("ByPrice".to_string())
    }
      Err(())
  }
}

#[derive(Debug, Deserialize,Serialize,Clone)]
pub struct University{
  //#[serde(rename(deserialize = "Index"))]
  pub index                   : i32,
  #[serde(rename(deserialize = "Name"))]
  pub name                    : String,
  #[serde(rename(deserialize = "Location"))]
  pub location                : String,
  #[serde(rename(deserialize = "Rank"))]
  pub rank                    : i32,
  #[serde(rename(deserialize = "Description"))]
  pub description             : String,
  #[serde(rename(deserialize = "Tuition and fees"))]
  pub tuition_and_fees        : String,
  #[serde(rename(deserialize = "In-state"))]
 // #[serde(skip_serializing_if = "Option::is_none")]
  pub in_state                : Option<String>,
  #[serde(rename(deserialize = "Undergrad Enrollment"))]
  pub undergrad_enrollment    : String    
}

impl University{
  pub fn get_params(&self)->Vec::<&(dyn postgres::types::ToSql + Sync)>{
    let mut params = Vec::<&(dyn postgres::types::ToSql + Sync)>::new();
    params.push(&self.index);
    params.push(&self.name);
    params.push(&self.location);
    params.push(&self.rank);
    params.push(&self.description);
    params.push(&self.tuition_and_fees);
    params.push(&self.in_state);
    params.push(&self.undergrad_enrollment);
    
    return params;
  }

  pub fn get_tuition_and_fees_as_i32(&self)->i32{
    let mut temp = self.tuition_and_fees.clone();
    temp = temp.replace(",","");
    temp = temp.replace(" ","");
    temp = temp.replace("$","");
    return temp.parse::<i32>().unwrap();
  }
}


