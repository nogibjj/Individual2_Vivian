mod lib;

fn main() {
    lib::load("california_housing_train.csv");

    lib::create_house(-122.5, 37.7, 40, 2000, 350, 1500, 350, 8.5, 500000);

    lib::update_house(1, -122.5, 37.7, 45, 2500, 400, 1600, 400, 9.0, 550000);

    lib::read_house_by_id(10);

    lib::delete_house(1);

    lib::close_connection();
}

