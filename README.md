# README [![Rust CI/CD Pipeline](https://github.com/nogibjj/Individual2_Vivian/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/Individual2_Vivian/actions/workflows/ci.yml)
This repository features the materials for Individual Project 2. It includes: 
- A Makefile
- A Dockerfile
- A foundational set of libraries for development operations and web applications
- GitHub Actions
- Cargo.toml to handle Rust dependencies
- Rust lib and main scripts in /src
- Rust test script in /tests


# Purpose Of Project
- The purpose of this project is to demonstrate CRUD (Create, Read, Update, Delete) operations with SQLite database and use Github Copilot to convert Python code into Rust. 
- I use the same dataset from my previous project "california_housing.csv" to load into a .db file, and querying it with SQLlite.
- Dataset attributes: "id","longitude","latitude","housing_median_age","total_rooms","total_bedrooms","population","households","median_income","median_house_value"

<img width="368" alt="Screen Shot 2023-09-30 at 11 31 51 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/ec33d8ad-be7c-4201-a688-3e7058ebfb6b">

# Preparation 
1. open the project in codespaces
2. container built and virtual environment to be activated through requirements.txt
3. build Rust: cargo build
4. run Rust: cargo run


# Check Format & Errors
1. make format
2. make lint
4. make test



# CRUD Operations
### functions in src/lib.rs
    close_connection: used to close the db connection
    create_house: create a new house data
    update_house: update a house data by its id
    delete_house: delete a house data by its id
    read_house_by_id: read a house data by its id
    load: load the data from csv file into the SQLite database and create a .db file

### query in src/main.rs
    connect and load into SQLite database: 
        load("california_housing_train.csv")
    create_house with the following information:
        longitude: -122.5,
        latitude: 37.7,
        housing_median_age: 40,
        total_rooms: 2000,
        total_bedrooms: 350,
        population: 1500,
        households: 350,
        median_income: 8.5,
        median_house_value: 500000.0
    update house with id=1 and the following information:
        longitude: -122.5,
        latitude: 37.7,
        housing_median_age: 45,
        total_rooms: 2500,
        total_bedrooms: 400,
        population: 1600,
        households: 400,
        median_income: 9.0,
        median_house_value: 550000.0
    read house with id=10
    delete house with id=1


# Output
- House successfully create: -122.5,37.7,40,2000,350,1500,350,8.5,500000
- House with id 1 successfullu updated to:-122.5,37.7,45,2500,400,1600,400,9.0,550000
- house with id 10 has these info: (10, -114.6, 34.83, 46.0, 1497.0, 309.0, 787.0, 271.0, 2.1908, 48100.0)
- House with id 1 deleted!

Below shows the output after each CRUD operations on my dataset: 

<img width="766" alt="Screen Shot 2023-09-30 at 11 44 28 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/c0f2da64-ff73-40fc-972c-890b71a06fd1">
