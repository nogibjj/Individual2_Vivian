extern crate rusqlite;
extern crate csv;

use rusqlite::{params, Connection, Result};

// Create a struct to represent a House.
#[derive(Debug)]
pub struct House {
    id: i32,
    longitude: f64,
    latitude: f64,
    housing_median_age: i32,
    total_rooms: i32,
    total_bedrooms: i32,
    population: i32,
    households: i32,
    median_income: f64,
    median_house_value: f64,
}

pub fn create_house(conn: &Connection, house: &House) -> Result<()> {
    conn.execute(
        "INSERT INTO houses (longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![house.longitude, house.latitude, house.housing_median_age, house.total_rooms, house.total_bedrooms, house.population, house.households, house.median_income, house.median_house_value]
    )?;
    println!("House successfully created: {:?}", house);
    Ok(())
}

pub fn read_all_houses(conn: &Connection) -> Result<Vec<House>> {
    let mut stmt = conn.prepare("SELECT * FROM houses")?;
    let house_iter = stmt.query_map([], |row| {
        Ok(House {
            id: row.get(0)?,
            longitude: row.get(1)?,
            latitude: row.get(2)?,
            housing_median_age: row.get(3)?,
            total_rooms: row.get(4)?,
            total_bedrooms: row.get(5)?,
            population: row.get(6)?,
            households: row.get(7)?,
            median_income: row.get(8)?,
            median_house_value: row.get(9)?,
        })
    })?;
    
    let mut houses = vec![];
    for house in house_iter {
        houses.push(house?);
    }
    Ok(houses)
}

pub fn update_house(conn: &Connection, house: &House) -> Result<()> {
    conn.execute(
        "UPDATE houses SET longitude=?, latitude=?, housing_median_age=?, total_rooms=?, total_bedrooms=?, population=?, households=?, median_income=?, median_house_value=? WHERE id=?", 
        params![house.longitude, house.latitude, house.housing_median_age, house.total_rooms, house.total_bedrooms, house.population, house.households, house.median_income, house.median_house_value, house.id]
    )?;
    println!("House with id {} successfully updated to: {:?}", house.id, house);
    Ok(())
}

pub fn delete_house(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM houses WHERE id=?", params![id])?;
    println!("House with id {} deleted!", id);
    Ok(())
}



pub fn load(dataset: &str) -> Result<()> {
    let conn = Connection::open("houses.db")?;

    // Create a table named 'houses'
    conn.execute(
        r"
        CREATE TABLE IF NOT EXISTS houses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            longitude DOUBLE,
            latitude DOUBLE,
            housing_median_age DOUBLE,
            total_rooms DOUBLE,
            total_bedrooms DOUBLE,
            population DOUBLE,
            households DOUBLE,
            median_income DOUBLE,
            median_house_value DOUBLE
        )
        ", 
        []
    )?;

    // Read the CSV file and insert data into the SQLite database
    let mut rdr = csv::Reader::from_path(dataset).unwrap();
    for result in rdr.deserialize() {
        let row: (f64, f64, f64, f64, f64, f64, f64, f64, f64) = result.unwrap();
        conn.execute(
            "INSERT INTO houses (longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![row.0, row.1, row.2, row.3, row.4, row.5, row.6, row.7, row.8]
        )?;
    }

    Ok(())
}

