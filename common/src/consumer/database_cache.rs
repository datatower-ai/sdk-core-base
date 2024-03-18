#[derive(Debug)]
struct DatabaseCacheConsumer {
    path: String,
}

impl DatabaseCacheConsumer {
    fn new(path: String) -> Self {
        DatabaseCacheConsumer {
            path
        }
    }
}