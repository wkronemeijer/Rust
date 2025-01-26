use clap::ValueEnum;
use rusqlite::Connection;

pub const CACHE_FILE_NAME: &str = "hash-cache.db";

// To prevent Boolean blindness
#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum ConnectionMode {
    File,
    Memory,
}

pub fn get_connection(mode: ConnectionMode) -> crate::Result<Connection> {
    let conn = match mode {
        ConnectionMode::Memory => Connection::open_in_memory()?,
        ConnectionMode::File => Connection::open(CACHE_FILE_NAME)?,
    };
    conn.execute_batch(include_str!("schema.sql"))?;
    Ok(conn)
}

pub fn connection_version(conn: &Connection) -> crate::Result<String> {
    Ok(conn.query_row("select sqlite_version()", (), |row| {
        let version: String = row.get(0)?;
        Ok(version)
    })?)
}
