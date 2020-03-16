use serde::{Deserialize, Serialize};

use crate::schema::users;

<<<<<<< HEAD
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub password: String,
}
=======
#[derive(Debug,Clone,Serialize,Deserialize,Queryable,Insertable)]
pub struct User{
    pub id:String,
    pub user_name:String,
    pub password:String,
}
>>>>>>> b6f927e0b194c5fd0f473b07e8a415bd797bdd75
