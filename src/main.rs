fn main() {
    data_fetcher::run().unwrap_or_else(|e| {
        eprintln!("{}", e);
    });
}
