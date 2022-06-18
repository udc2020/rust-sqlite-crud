use rusqlite::{Connection, Result};
use rusqlite::params;


// Db Location
const URL_DB:&str = "./test.db";


// The Struct of data must be the same of comumns in table
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[allow(dead_code)]
pub fn add_new(id: i32, username: &str, password: &str) -> Result<()> {
   let connection = Connection::open(URL_DB)?;

   let u = User {
       id: id,
       username: username.to_string(),
       password: password.to_string(),
   };
   connection.execute(
       "
   INSERT INTO users (id,username,password) VALUES (?, ?, ?)
   ",
       params![u.id, u.username, u.password],
   )?;
   Ok(())
}

#[allow(dead_code)]
pub fn update_one( id: i32, username: &str, password: &str) -> Result<()> {
   let connection = Connection::open(URL_DB)?;

   let u = User {
       id: id,
       username: username.to_string(),
       password: password.to_string(),
   };
   connection.execute(
       "UPDATE users SET username= ? , password= ? WHERE id= ? ;",
       params![&u.username, &u.password, &u.id],
   )?;

   Ok(())
}

#[allow(dead_code)]
pub fn remove_one( id: i32) -> Result<()> {
   let connection = Connection::open(URL_DB)?;
   connection.execute("DELETE FROM users WHERE id= ? ;", params![&id])?;

   Ok(())
}

#[allow(dead_code)]
 pub fn find_all  ()-> Result<Vec<User>>{

   let connection = Connection::open(URL_DB)?;
   
   let mut get_all = connection.prepare("select * from users;")?;

   let mut all_data: Vec<User> = Vec::new();

   let iter_all = get_all.query_map([], |row| {
       all_data.push(User {
           id: row.get(0)?,
           username: row.get(1)?,
           password: row.get(2)?,
       });

       Ok(User {
           id: row.get(0)?,
           username: row.get(1)?,
           password: row.get(2)?,
       })
   })?;

   for user in iter_all {
       drop(user);
   }

   Ok(all_data)
 }

 #[allow(dead_code)]
 pub fn find_one(id:i32)-> Result<Vec<User>>{
   let connection = Connection::open(URL_DB)?;
   

   let sql = format!("select id,username,password from users where id = {} limit 1",id);
   let mut get_all = connection.prepare(&sql)?;

   let mut all_data: Vec<User> = Vec::new();

   let iter_all = get_all.query_map([], |row| {
      all_data.push(User {
          id: row.get(0)?,
          username: row.get(1)?,
          password: row.get(2)?,
      });

      Ok(User {
          id: row.get(0)?,
          username: row.get(1)?,
          password: row.get(2)?,
      })
  })?;

  for user in iter_all {
      drop(user);
  }

  Ok(all_data)
 }