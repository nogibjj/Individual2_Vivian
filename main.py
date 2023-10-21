from lib.CRUD import (
    close_connection,
    create_house,
    update_house,
    delete_house,
    read_house_by_id
)
from lib.load_data import load


def main():
    load("california_housing_train.csv")  # Import data from CSV

    #create_house
    create_house(-122.5, 37.7, 40, 2000, 350, 1500, 350, 8.5, 500000)  # Create

    # update house with id=1
    update_house(1, -122.5, 37.7, 45, 2500, 400, 1600, 400, 9.0, 550000)  # Update

    # print house with id=10
    read_house_by_id(10)  # Read after update

    # delete house with id=5
    delete_house(1)  # Delete

    # Close the connection
    close_connection()
    return 1


if __name__ == "__main__":
    main()
