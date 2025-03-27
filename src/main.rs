mod mempool;
use mempool::fetch_mempool_data;

#[tokio::main]

async fn main() {
    match fetch_mempool_data().await {
        Ok(data) => {
            println!("Count: {}", data.count);
            println!("Vsize: {}", data.vsize);
            println!("Total Fee: {}", data.total_fee);    
         }
         Err(e) => eprintln!("Failed to fetch mempool data: {}", e),
        
    }
    

}
