// tests/test_main.rs

use sqlite_operations;

#[test]
fn test_main_function() {
    // Run the main function and assert success
    let result = sqlite_operations::main();
    assert_eq!(result, Ok(())); // assuming your main function returns Result<(), Box<dyn Error>>
}

#[test]
fn test_load_function() {
    // Assuming you've made the `load` function public
    let result = sqlite_operations::load("california_housing_train.csv");
    assert!(result.is_ok());
}

// ...similarly you can add tests for other functions

#[test]
fn test_create_house() {
    let result = sqlite_operations::create_house(-122.5, 37.7, 40, 2000, 350, 1500, 350, 8.5, 500000);
    assert!(result.is_ok());
}

// Add more tests for update_house, delete_house, and so on...
