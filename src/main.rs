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

const WORKERS: usize = 4;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let file = std::fs::File::open("urls.txt")?;
    let buffile = std::io::BufReader::new(file);
    println!("URL,status");

    let client = reqwest::Client::new();
    let mut workers = workers::Workers::new();
    let (tx, rx) = async_channel::bounded(WORKERS * 2);

    workers.spawn(async move {
        for line in buffile.lines() {
            let line = line?;
            tx.send(line).await.unwrap();
        }
        Ok(())
    });

    for _ in 0..WORKERS {
        let client = client.clone();
        let rx = rx.clone();
        workers.spawn(async move {
            loop {
                match rx.recv().await {
                    Err(_) => break Ok(()),
                    Ok(line) => {
                        let resp = client.get(&line).send().await?;
                        println!("{}.{}", line, resp.status().as_u16());
                    }
                }
            }
        })
    }

    workers.run().await
}
