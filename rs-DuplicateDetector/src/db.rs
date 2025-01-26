//! Contains items to wrap the usage of SQLite.
//!
//! In particular, the goal is to not see any SQL outside of this file.

use clap::ValueEnum;
use rusqlite::Connection;

///////////////////////////
// Creating a connection //
///////////////////////////

// To prevent Boolean blindness
#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum ConnectionMode {
    File,
    Memory,
}

pub const CACHE_FILE_NAME: &str = "hash-cache.db";

pub fn init_db(mode: ConnectionMode) -> crate::Result<Connection> {
    let conn = match mode {
        ConnectionMode::Memory => Connection::open_in_memory()?,
        ConnectionMode::File => Connection::open(CACHE_FILE_NAME)?,
    };
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(conn)
}

//////////////////////////
// Using the connection //
//////////////////////////

pub fn db_version(conn: &Connection) -> crate::Result<String> {
    Ok(conn.query_row("select sqlite_version()", (), |row| {
        let version: String = row.get(0)?;
        Ok(version)
    })?)
}
