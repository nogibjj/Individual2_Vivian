import sqlite3

# Establish a connection
conn = sqlite3.connect('houses.db')
cursor = conn.cursor()

# CREATE: Insert a new house entry
def create_house(longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value):
    cursor.execute("INSERT INTO houses (longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", 
                   (longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value))
    conn.commit()
    numbers=[longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value]
    print("House successfully create: "+','.join(map(str, numbers)))

# READ: Get all houses
def read_all_houses():
    cursor.execute("SELECT * FROM houses")
    return cursor.fetchall()

def read_house_by_id(house_id):
    cursor.execute("SELECT * FROM houses WHERE id=?", (house_id,))
    # Fetch the result
    house = cursor.fetchone()
    print("house with id "+str(house_id)+" has these info: "+str(house))


# UPDATE: Update a house's data based on its ID 
def update_house(id, longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value):
    cursor.execute("UPDATE houses SET longitude=?, latitude=?, housing_median_age=?, total_rooms=?, total_bedrooms=?, population=?, households=?, median_income=?, median_house_value=? WHERE id=?", 
                   (longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value, id))
    conn.commit()
    numbers=[longitude, latitude, housing_median_age, total_rooms, total_bedrooms, population, households, median_income, median_house_value]
    print("House with id "+str(id)+" successfullu updated to:"+','.join(map(str, numbers)))

# DELETE: Delete a house by its ID
def delete_house(id):
    cursor.execute("DELETE FROM houses WHERE id=?", (id,))
    conn.commit()
    print("House with id "+str(id)+" deleted!")

# Close the connection (Make sure to call this when done with database operations)
def close_connection():
    conn.close()

# create_house(-122.23, 37.88, 41.0, 880.0, 129.0, 322.0, 126.0, 8.3252, 452600.0)
# print(read_all_houses())
# update_house(1, -122.23, 37.88, 45.0, 880.0, 130.0, 322.0, 126.0, 8.3252, 452600.0)
# delete_house(1)
# close_connection()
