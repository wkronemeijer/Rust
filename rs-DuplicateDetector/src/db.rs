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

pub const CACHE_SCHEMA: &str = "
begin;

create table if not exists [file] (
    [path] text not null, -- primary key
    [hash] text not null
) strict;

commit;
";

pub const CACHE_FILE_NAME: &str = "hash-cache.db";

pub fn init_db(mode: ConnectionMode) -> crate::Result<Connection> {
    let conn = match mode {
        ConnectionMode::Memory => Connection::open_in_memory()?,
        ConnectionMode::File => Connection::open(CACHE_FILE_NAME)?,
    };
    conn.execute_batch(CACHE_SCHEMA)?;
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

pub fn db_purge(conn: &Connection) -> crate::Result {
    conn.execute_batch("delete from [file]")?;
    Ok(())
}
