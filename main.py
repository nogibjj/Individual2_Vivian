mod lib;

fn main() -> rusqlite::Result<()> {
    lib::load("california_housing_train.csv")?;  // Import data from CSV

    // Create a house
    lib::create_house(-122.5, 37.7, 40.0, 2000.0, 350.0, 1500.0, 350.0, 8.5, 500000.0)?;

    // Update house with id=1
    lib::update_house(1, -122.5, 37.7, 45.0, 2500.0, 400.0, 1600.0, 400.0, 9.0, 550000.0)?;

    // Print house with id=10
    lib::read_house_by_id(10)?;

    // Delete house with id=1
    lib::delete_house(1)?;

    // Close the connection
    lib::close_connection()
}

