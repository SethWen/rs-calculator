use std::num::NonZeroI64;

use rusqlite::{params, Connection, Result};

fn learning() -> Result<()> {
    println!("Hello, learning!");
    let conn = Connection::open_with_flags(
        "../tmp/oridb/example.db",
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
    )?;

    println!("Opened database successfully");

    conn.query_row(
        "PRAGMA key = \"x'8F50E143AFEA4124856FD9AB40C20B64E251C35674C946EBA8F5E38E31F3AC77'\";",
        [],
        |_| Ok(()),
    )?;
    // conn.query_row("PRAGMA key = '8F50E143AFEA4124856FD9AB40C20B64E251C35674C946EBA8F5E38E31F3AC77';", [], |_| Ok(()))?;
    println!("set key {}", "PRAGMA key = \"x'8F50E143AFEA4124856FD9AB40C20B64E251C35674C946EBA8F5E38E31F3AC77'\";");

    // conn.execute("PRAGMA cipher_page_size = 4096;", [])?;
    // conn.execute("PRAGMA kdf_iter = 64000;", [])?;
    conn.query_row("PRAGMA kdf_iter;", [], |row| {
        println!("kdf_iter: {:?}", row);
        Ok(())
    })?;

    // conn.execute("PRAGMA cipher_page_size = 4096;", [])?;
    conn.query_row("PRAGMA cipher_page_size;", [], |row| {
        println!("cipher_page_size: {:?}", row);
        Ok(())
    })?;

    conn.execute("PRAGMA cipher_plaintext_header_size = 0;", [])?;
    println!("set cipher_plaintext_header_size successfully");
    conn.query_row("PRAGMA cipher_plaintext_header_size;", [], |row| {
        println!("cipher_plaintext_header_size: {:?}", row);
        Ok(())
    })?;

    // 创建一个表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )?;
    print!("Created table successfully");

    conn.query_row("SELECT COUNT(*) FROM users;", [], |row| {
        println!("count: {:?}", row);
        Ok(())
    })?;

    // 插入一些数据
    // conn.execute("INSERT INTO users (name, email) VALUES (?1, ?2)", params!["Jack", "jack@example.com"])?;
    // conn.execute("INSERT INTO users (name, email) VALUES (?1, ?2)", params!["Tom", "tom@example.com"])?;
    // println!("Inserted data successfully");

    // // 查询数据
    // let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
    // let user_iter = stmt.query_map([], |row| Ok(User { id: row.get(0)?, name: row.get(1)?, email: row.get(2)? }))?;

    // for user in user_iter {
    //     println!("Found user: {:?}", user?);
    // }

    Ok(())
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

fn read_wechat_db(sz: i32) -> Result<()> {
    let conn = Connection::open_with_flags(
        "../tmp/oridb/FunctionMsg1.db",
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
        // | rusqlite::OpenFlags::SQLITE_OPEN_PRIVATE_CACHE,
    )?;

    println!("Opened database successfully");

    conn.query_row(
        "PRAGMA key = \"x'8F50E143AFEA4124856FD9AB40C20B64E251C35674C946EBA8F5E38E31F3AC77'\";",
        [],
        |_| Ok(()),
    )?;
    // conn.query_row("PRAGMA key = '8F50E143AFEA4124856FD9AB40C20B64E251C35674C946EBA8F5E38E31F3AC77';", [], |_| Ok(()))?;
    println!("set key {}", "PRAGMA key = \"x'8F50E143AFEA4124856FD9AB40C20B64E251C35674C946EBA8F5E38E31F3AC77'\";");

    // conn.execute("PRAGMA cipher_page_size = 4096;", [])?;
    conn.execute("PRAGMA kdf_iter = 64000;", [])?;
    conn.query_row("PRAGMA kdf_iter;", [], |row| {
        println!("kdf_iter: {:?}", row);
        Ok(())
    })?;

    conn.execute("PRAGMA cipher_page_size = 4096;", [])?;
    conn.query_row("PRAGMA cipher_page_size;", [], |row| {
        println!("cipher_page_size: {:?}", row);
        Ok(())
    })?;

    conn.execute(&format!("PRAGMA cipher_plaintext_header_size = {};", sz), [])?;
    println!("Set PRAGMA successfully");

    // conn.execute("PRAGMA page_size = 4096;", [])?;
    // println!("Set page_size successfully");
    // conn.query_row("PRAGMA page_size;", [], |row| {
    //     println!("page_size: {:?}", row);
    //     Ok(())
    // })?;

    // conn.execute("PRAGMA temp_store = 2;", [])?;
    // println!("Set temp_store successfully");
    // conn.query_row("PRAGMA temp_store;", [], |row| {
    //     println!("temp_store_size: {:?}", row);
    //     Ok(())
    // })?;

    conn.query_row("SELECT COUNT(*) FROM FunctionMsg;", [], |row| {
        println!("count: {:?}", row);
        Ok(())
    })?;

    // conn.execute("PRAGMA cache_size = 2000;", [])?;
    // println!("Set cache_size successfully");

    // 查询数据
    // let mut stmt = conn.prepare(
    //     "SELECT localId, Type, SubType, CompressContent FROM PublicMsg WHERE Type = 49 AND SubType = 5 LIMIT 10",
    // )?;
    // println!("Prepared statement successfully");
    // let user_iter = stmt.query_map([], |row| {
    //     Ok((row.get::<usize, NonZeroI64>(0)?, row.get::<usize, NonZeroI64>(1)?, row.get::<usize, NonZeroI64>(2)?))
    // })?;

    let data_iter = conn.query_row("SELECT nCreateTime, nHashId, strFunctionMsgId FROM FunctionMsg;", [], |row| {
        Ok((row.get::<usize, NonZeroI64>(0)?, row.get::<usize, NonZeroI64>(1)?, row.get::<usize, String>(2)?))
    })?;
    // let mut stmt = conn.prepare(
    //     "SELECT nCreateTime, nHashId, strFunctionMsgId FROM FunctionMsg",
    // )?;
    // println!("Prepared statement successfully");
    // let item_iter = stmt.query_map([], |row| {
    //     Ok((row.get::<usize, NonZeroI64>(0)?, row.get::<usize, NonZeroI64>(1)?, row.get::<usize, String>(2)?))
    // })?;
    println!("query_map finished {:?}", data_iter);

    // for item in item_iter {
    //     println!("Found user: {:?}", item?);
    // }

    Ok(())
}

#[test]
pub fn test_learning() {
    match learning() {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Failed ----> {e}"),
    }
}

#[test]
pub fn test_read_wechat_db() {
    let sqlite_file_header = b"SQLite format 3\0";
    for c in sqlite_file_header {
        print!("{:x} ", c);
    }
    println!("==============");

    for i in 0..=256 {
        match read_wechat_db(i) {
            Ok(_) => {
                println!("Success! {i}");
                break;
            }
            Err(e) => println!("Failed {i} ----> {e}"),
        }
    }
}
