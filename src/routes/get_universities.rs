
use rocket::serde::Deserialize;
use rocket::serde::json::{json,Json,Value};
use crate::models::SortType;
use crate::database::module;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SearchParameters {
  pub sort_type:i32
}

#[post("/getUniversities", data = "<parameters>")]
pub async fn route(conn: module::DataBase, parameters: Json<SearchParameters>) -> Value {
  let result = conn.run(move |c| module::DataProcessor::get_universities(c,SortType::try_from(parameters.sort_type).unwrap())).await;
  match result{
    Ok(ok)=> {
        let value = rocket::serde::json::to_value(&ok);
        json!({ "status": "ok", "universities": value.unwrap()})
    },
    Err(error) => json!({"status": "error", "description": error.to_string()}),
  }
}