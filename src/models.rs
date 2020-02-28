use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Insertable)]
pub struct User{
    pub id:String,
    pub user_name:String,
    pub password:String,
}