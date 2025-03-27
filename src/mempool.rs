use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]

struct mempoolData {
    count: u64,
    vsize: u64,
    total_fee: u64,
}

pub async fn fetch_mempool_data() -> Result<mempoolData, reqwest::Error> {
    let url = "https://mempool.space/api/v1/mempool/summary";
    let response = reqwest::get(url).await?.json::mempoolData>().await?;
    Ok(response)

}