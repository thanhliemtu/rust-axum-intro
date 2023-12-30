#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {

    let hc = httpc_test::new_client("http://localhost:8080")?;

    // Testing GET query params
    // (uncomment the line below to test)
    // hc.do_get("/hello?name=Haruka").await?.print().await?;

    // Testing GET path params
    hc.do_get("/hello2/Hansamu").await?.print().await?;

    // Testing POST to route with String extractor 
    // (uncomment the line below to test)
    // hc.do_post("/hello3", "Hello from POST request").await?.print().await?;

    // Testing POST to route with String extractor 
    // by sending a JSON body
    // (uncomment the block below to test)
    // let req_login = hc.do_post(
    //     "/hello3",
    //     json!({
    //         "username": "demo1",
    //         "pwd": "welcome"
    //     })
    // );
    // req_login.await?.print().await?;

    // Testing static routing (uncomment the line below to test)
    // hc.do_get("/commands.txt").await?.print().await?;

    // Testing POST to /api/login route
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    );
    req_login.await?.print().await?;

    Ok(())
}