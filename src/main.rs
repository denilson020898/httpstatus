use std::io::BufRead;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open("urls.txt")?;
    let buffile = std::io::BufReader::new(file);

    println!("URL,Status");

    let client = reqwest::Client::new();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut count = 0;

    for line in buffile.lines() {
        let line = line?;

        let client = client.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let msg = client.get(&line).send().await.map(|resp| {
                println!("{},{}", line, resp.status().as_u16());
            });
            tx.send(msg).unwrap();
        });

        count += 1;
    }

    std::mem::drop(tx);

    let mut i = 0;
    loop {
        match rx.recv().await {
            None => {
                assert_eq!(i, count);
                break Ok(());
            }
            Some(Ok(())) => {
                assert!(i < count);
            }
            Some(Err(e)) => {
                assert!(i < count);
                return Err(From::from(e));
            }
        }
        i += 1;
    }
}
