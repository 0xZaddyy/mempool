mod mempool;
use mempool::fetch_mempool_data;
use crate::mempool::check_congestion;

#[tokio::main]

async fn main() {
    match fetch_mempool_data().await {
        Ok(data) => {
            println!("🚀 Bitcoin Mempool Monitor");
            println!("📌 Total Unconfirmed Transactions:{}", data.count);
            println!("Total vsize:{}", data.vsize);
            println!("Total Fee: {}", data.total_fee); 

            let congestion = check_congestion(data.count, data.vsize);
            println!("⚠️ Network Status: {}", congestion);   
         }
         Err(e) => eprintln!("Failed to fetch mempool data: {}", e),
        
    }

   
    
    

}
