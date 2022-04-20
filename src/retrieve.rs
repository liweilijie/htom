use std::path::Path;
use std::time::Duration;
use tokio::task::spawn_blocking;
use crate::Result;

// retrieve html then to path
pub async fn retrieve(url: &str, p: impl AsRef<Path>) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let resp = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    // convert html to markdown
    let md = spawn_blocking(move || html2md::parse_html(resp.as_str()))
        .await?;

    // write to file
    tokio::fs::write(p, md).await?;

    Ok(())
}
