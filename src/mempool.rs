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

pub fn check_congestion(count: u64,vsize: u64) -> &'static str {
    if count > 50_000 || vsize > 100_000_000 {
        "High congestion, transaction will be delayed"
    } else if count > 20_000 || vsize > 50_000_000 {
        "Moderate congestion, Slight delay possible"
        
    } else{
        "Low Congestion - Transactions Should Confirm Quickly"
    }
}