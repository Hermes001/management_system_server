use serde::{Deserialize, Serialize};

use crate::schema::assortment;
use crate::schema::commodity;
use crate::schema::harddisk_type;
use crate::schema::user;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "commodity"]
pub struct Commodity {
    pub id: String,
    pub assortment_id: String,
    pub model: Option<String>,
    pub description: Option<String>,
    pub price_retrieve: i32,
    pub price_sale: Option<i32>,
    pub user_id: String,
    pub memory: Option<i16>,
    pub harddisk: Option<i16>,
    pub harddisk_type_id: Option<String>,
    pub gpu: Option<String>,
    pub cpu: Option<String>,
    pub price_retrieve_external: Option<i32>,
    pub price_sale_external: Option<i32>,
    pub photo_url: Option<String>,
    pub quantity: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "assortment"]
pub struct Assortment {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "harddisk_type"]
pub struct HarddiskType {
    pub id: String,
    pub name: String,
}
