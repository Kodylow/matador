use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // hello tests
    // hello with no name
    hc.do_get("/hello").await?.print().await?;
    // hello with query param name
    hc.do_get("/hello?name=John").await?.print().await?;
    // hello with name in path
    hc.do_get("/hello2/John").await?.print().await?;

    // static tests
    hc.do_get("src/main.rs").await?.print().await?;
    Ok(())
}
