fn main() {
    #[cfg(not(any(feature = "log-consumer-server", feature = "db-cache-consumer-client", feature = "async-upload-server")))]
    compile_error!("At least one consumer needs to be included by --feature!");

    #[cfg(not(any(feature = "cat_server", feature = "cat_client")))]
    compile_error!("At least one category needs to be included by --feature!");

    #[cfg(any(
        all(feature = "cat_server", any(feature = "cat_client")),
        all(feature = "cat_client", any(feature = "cat_server"))
    ))]
    compile_error!("Exactly one category can be included by --feature!");
}