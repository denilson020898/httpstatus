use std::io::BufRead;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("urls.txt")?;
    let buffile = std::io::BufReader::new(file);
    println!("URL, Status");

    let client = reqwest::Client::new();

    let mut handles = Vec::new();

    for line in buffile.lines() {
        let line = line?;

        let client = client.clone();
        let handle = tokio::spawn(async move {
            let resp = client.get(&line).send().await.unwrap();
            println!("{},{}", line, resp.status().as_u16());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
