pub fn load_test_env() {
    dotenv::from_filename("tests.env")
        .unwrap_or_else(|_| panic!("tests.env file has not being created or couldn't be loaded"));
}
