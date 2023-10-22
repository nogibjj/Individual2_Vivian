// tests/test_main.rs

use sqlite_operations;

#[test]
fn test_load_function() {
    // Assuming you've made the `load` function public
    let result = sqlite_operations::load("california_housing_train.csv");
    assert!(result.is_ok());
}

