fn main() {
  api_fetcher::run().unwrap_or_else(|e| {
    eprintln!("{}", e);
  });
}