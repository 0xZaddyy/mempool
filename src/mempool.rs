use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]

pub struct MempoolData {
    pub count: u64,
    pub vsize: u64,
    pub total_fee: u64,
}

pub async fn fetch_mempool_data() -> Result<MempoolData, reqwest::Error> {
    let url = "https://mempool.space/api/mempool";
    let response = reqwest::get(url).await?.json::<MempoolData>().await?;
    Ok(response)

}