use reqwest;
use serde::Deserialize;

#[derive(Deserialize,Debug )]

struct mempoolData {
    count: u64,
    vsize: u64,
    total_fee: u64,
}

