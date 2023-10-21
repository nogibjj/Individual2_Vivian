# README [![CI](https://github.com/nogibjj/Project5_Vivian/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/Project5_Vivian/actions/workflows/ci.yml)
This repository features the materials for Mini-Project 5. It includes: 
- A Makefile
- A Dockerfile
- A foundational set of libraries for development operations and web applications
- GitHub Actions


# Purpose Of Project
The purpose of this project is to build an ETL-Query pipeline. I use the same dataset from my previous project "california_housing.csv" to load into a .db file, and querying it with SQLlite.
Dataset attributes: "id","longitude","latitude","housing_median_age","total_rooms","total_bedrooms","population","households","median_income","median_house_value"

<img width="368" alt="Screen Shot 2023-09-30 at 11 31 51 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/ec33d8ad-be7c-4201-a688-3e7058ebfb6b">

# Preparation 
1. open the project in codespaces
2. container built and virtual environment to be activated through requirements.txt

# Check Format & Errors
1. make format
2. make lint
4. make test

<img width="940" alt="Screen Shot 2023-09-30 at 11 43 41 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/bd7caa1e-cc41-4645-8af3-6929eef41df2">

<img width="939" alt="Screen Shot 2023-09-30 at 11 44 04 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/aa1f500f-2b0a-49b9-8dc3-27022c279d79">

# CRUD Operations
### functions in lib/CRUD.py
    close_connection: used to close the db connection
    create_house: create a new house data
    update_house: update a house data by its id
    delete_house: delete a house data by its id
    read_house_by_id: read a house data by its id
   
## functions in lib/load_data.py
    load: load the data from csv file into the SQLite database and create a .db file

## CRUD in main.py
    connect and load into SQLite database: 
       load("california_housing_train.csv")
    create_house: 
        create_house(-122.5, 37.7, 40, 2000, 350, 1500, 350, 8.5, 500000)
    update house with id=1:
        update_house(1, -122.5, 37.7, 45, 2500, 400, 1600, 400, 9.0, 550000)
    print house with id=10:
        read_house_by_id(10)
    delete house with id=5:
        delete_house(1)
    Close the connection:
        close_connection()


# Output
- House successfully create: -122.5,37.7,40,2000,350,1500,350,8.5,500000
- House with id 1 successfullu updated to:-122.5,37.7,45,2500,400,1600,400,9.0,550000
- house with id 10 has these info: (10, -114.6, 34.83, 46.0, 1497.0, 309.0, 787.0, 271.0, 2.1908, 48100.0)
- House with id 1 deleted!

Below shows the output after each CRUD operations on my dataset: 

<img width="766" alt="Screen Shot 2023-09-30 at 11 44 28 AM" src="https://github.com/nogibjj/Project5_Vivian/assets/143654445/c0f2da64-ff73-40fc-972c-890b71a06fd1">
