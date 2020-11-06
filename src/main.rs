mod workers;

use std::io::BufRead;

#[derive(Debug)]
enum AppError {
    IO(std::io::Error),
    Reqwest(reqwest::Error),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IO(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Reqwest(e)
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let file = std::fs::File::open("urls.txt")?;
    let buffile = std::io::BufReader::new(file);
    println!("URL,status");

    let client = reqwest::Client::new();
    let mut workers = workers::Workers::new();

    for line in buffile.lines() {
        let line = line?;
        let client = client.clone();
        workers.spawn(async move {
            let resp = client.get(&line).send().await?;
            println!("{},{}", line, resp.status().as_u16());
            Ok(())
        });
    }

    workers.run().await
}
