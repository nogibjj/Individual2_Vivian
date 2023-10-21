import csv
import sqlite3

def load(dataset="california_housing_train.csv"):
    # Connect to SQLite database
    conn = sqlite3.connect('houses.db')
    cursor = conn.cursor()

    # Create a table named 'houses'
    cursor.execute('''
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
    ''')

    # Read the CSV file and insert data into the SQLite database
    with open(dataset, 'r') as csv_file:
        csv_reader = csv.DictReader(csv_file)
        for row in csv_reader:
            cursor.execute('''
            INSERT INTO houses (longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ''', (row["longitude"], row["latitude"], row["housing_median_age"], row["total_rooms"], row["total_bedrooms"], row["population"], row["households"], row["median_income"], row["median_house_value"]))

    # Commit changes and close the SQLite connection
    conn.commit()
    conn.close()
