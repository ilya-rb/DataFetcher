use api_fetcher::run;

fn main() {
  run().unwrap_or_else(|e| {
    eprintln!("{}", e);
  });
}