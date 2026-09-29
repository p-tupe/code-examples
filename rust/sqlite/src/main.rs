//! Same usage of sqlite (single thread) in Rust using a well known crate.
//! Refer https://docs.rs/rusqlite/latest/rusqlite/index.html
//! WIP

use std::{error::Error, fs};

use rusqlite::{Connection, params};
fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open("./test.db")?;

    // CRUD
    sample_crud(&mut conn)?;

    // Prepared Statements
    sample_prepped(&mut conn)?;

    // Transactions
    sample_transaction(&mut conn)?;

    Ok(fs::remove_file("./test.db")?)
}

fn sample_crud(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    conn.execute_batch("create table x (a text); create table y (b text);")?;
    conn.execute("insert into x (a) values (?)", params!["something"])?;
    conn.execute("update x set a = (?);", params!["something else"])?;
    let a_val = conn.execute("select * from x where a = (?);", params!["something else"])?;
    println!("returned {}", a_val);
    Ok(())
}

fn sample_prepped(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn sample_transaction(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    let tx = conn.transaction();

    Ok(())
}
