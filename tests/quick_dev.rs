use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // region: -- hello tests
    // hello with no name
    hc.do_get("/hello").await?.print().await?;
    // hello with query param name
    hc.do_get("/hello?name=John").await?.print().await?;
    // hello with name in path
    hc.do_get("/hello/John").await?.print().await?;
    // endregion: -- end hello tests

    // region: -- static tests

    // hc.do_get("/src/main.rs").await?.print().await?;
    // endregion: -- static tests

    // region: -- login tests
    // login with wrong credentials
    hc.do_post("/api/login", json!({"username":"admin","pwd":"wrong"}))
        .await?
        .print()
        .await?;
    // login with correct credentials
    hc.do_post("/api/login", json!({"username":"admin","pwd":"admin"}))
        .await?
        .print()
        .await?;

    // endregion: -- login tests

    // region: -- tickets tests

    // create ticket
    hc.do_post("/api/tickets", json!({"title":"ticket 1"}))
        .await?
        .print()
        .await?;

    // list tickets
    hc.do_get("/api/tickets").await?.print().await?;

    // delete ticket 
    hc.do_delete("/api/tickets/0").await?.print().await?;

    // endregion: -- tickets tests
    Ok(())
}
