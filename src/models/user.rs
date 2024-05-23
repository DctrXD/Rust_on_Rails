use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use super::schema::users;

#[derive(Queryable, Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)] 
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}