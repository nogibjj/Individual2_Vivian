extern crate rusqlite;
extern crate csv;
use sqlite_operations::{load,create_house,update_house,delete_house,read_house_by_id};


fn main() -> Result<(), rusqlite::Error> {
    load("california_housing_train.csv")?; // Import data from CSV

    let conn = rusqlite::Connection::open("houses.db")?;

    // Create house
    let new_house = sqlite_operations::House {
        id: 0,  // Placeholder, as actual ID will be generated by the database
        longitude: -122.5,
        latitude: 37.7,
        housing_median_age: 40.0,
        total_rooms: 2000.0,
        total_bedrooms: 350.0,
        population: 1500.0,
        households: 350.0,
        median_income: 8.5,
        median_house_value: 500000.0,
    };
    create_house(&conn, &new_house)?;

    // Update house with id=1
    let updated_house = sqlite_operations::House {
        id: 1,  // This ID specifies which record to update
        longitude: -122.5,
        latitude: 37.7,
        housing_median_age: 45.0,
        total_rooms: 2500.0,
        total_bedrooms: 400.0,
        population: 1600.0,
        households: 400.0,
        median_income: 9.0,
        median_house_value: 550000.0,
    };
    update_house(&conn, &updated_house)?;

    // Print house with id=10
    read_house_by_id(&conn, 10)?;

    // Delete house with id=1
    delete_house(&conn, 1)?;
    
    Ok(())
}

