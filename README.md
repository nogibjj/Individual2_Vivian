# README [![Rust CI/CD Pipeline](https://github.com/nogibjj/Individual2_Vivian/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/Individual2_Vivian/actions/workflows/ci.yml)
This repository features the materials for Individual Project 2. It includes: 
- A Makefile
- A Dockerfile
- A foundational set of libraries for development operations and web applications
- GitHub Actions
- Cargo.toml to handle Rust dependencies
- Rust lib and main scripts in /src
- Rust test script in /tests


## Purpose Of Project
- The purpose of this project is to demonstrate CRUD (Create, Read, Update, Delete) operations with SQLite database and use Github Copilot to convert Python code into Rust. 
- I use the same dataset from my previous project "california_housing.csv" to load into a .db file, and querying it with SQLlite.
- Dataset attributes: "id","longitude","latitude","housing_median_age","total_rooms","total_bedrooms","population","households","median_income","median_house_value"

<img width="368" alt="Screen Shot 2023-09-30 at 11 31 51 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/ec33d8ad-be7c-4201-a688-3e7058ebfb6b">

## Preparation 
1. open the project in codespaces
2. container built and virtual environment to be activated through requirements.txt
3. build Rust: cargo build
4. run Rust: cargo run


## Check Format & Errors
1. make format
2. make lint
4. make test

<img width="702" alt="Screen Shot 2023-10-22 at 12 32 40 PM" src="https://github.com/nogibjj/Individual2_Vivian/assets/143654445/cdfa509a-3a9e-4cd2-b122-a10060315bf1">


## CRUD Operations
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


## Output
- House successfully created: House { id: 0, longitude: -122.5, latitude: 37.7, housing_median_age: 40.0, total_rooms: 2000.0, total_bedrooms: 350.0, population: 1500.0, households: 350.0, median_income: 8.5, median_house_value: 500000.0 }
- House with id 1 successfully updated to: House { id: 1, longitude: -122.5, latitude: 37.7, housing_median_age: 45.0, total_rooms: 2500.0, total_bedrooms: 400.0, population: 1600.0, households: 400.0, median_income: 9.0, median_house_value: 550000.0 }
- Check house with id 10: House { id: 10, longitude: -114.6, latitude: 34.83, housing_median_age: 46.0, total_rooms: 1497.0, total_bedrooms: 309.0, population: 787.0, households: 271.0, median_income: 2.1908, median_house_value: 48100.0 }
- Delete house with id 1: House with id 1 deleted!

Below shows the output after each CRUD operations on my dataset: 
<img width="924" alt="Screen Shot 2023-10-22 at 1 30 43 PM" src="https://github.com/nogibjj/Individual2_Vivian/assets/143654445/490289c2-28f7-461b-a1aa-e129cc0a7c18">

## Optimized Rust Binary
I included the action of optimized-binary in my CICD process:
- name: Archive Binary
      uses: actions/upload-artifact@v2
      with:
        name: optimized-binary
        path: target/release/jeremy_tan_sqlite
  
Note: You can find and download the uploaded artifact from actions' latest workflow run.

## Use of Copilot
I integrated GitHub Copilot into my IDE and codespace to assist with code writing. While developing the Rust application, I started with a python script containing specific functions and operations and ask Copilot to provided real-time suggestions based on the comments I provided, and it helped me convert the code into Rust. 

The code provided, especially functions like create_house, read_all_houses, update_house, and delete_house, was influenced by Copilot suggestions. The data structure House and how it was integrated with the SQLite functions was largely generated with the help of Copilot. Additionally, the intricate handling with the SQLite library and error management was also shaped by Copilot's assistance.

some examples where I use copilot in my code:

//Impelement function create_house to Rust

//create a data structure House 

//handle the loading csv error

## Youtube Video

