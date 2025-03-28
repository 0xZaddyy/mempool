Bitcoin Mempool Monitor

This Rust-based Mempool Monitor tracks unconfirmed Bitcoin transactions, providing insights into network congestion, transaction volume, and fee estimations.

Features

Fetches real-time Bitcoin mempool data.

Tracks the number of unconfirmed transactions (count).

Understanding vsize and count

vsize (Virtual Size)

Definition: The virtual size of a Bitcoin transaction, measured in vbytes (vB).

Importance: Transaction fees are based on vsize (sat/vB).

Calculation: It is derived from transaction weight:

vsize = weight / 4

where weight is measured in weight units (WU).

count (Transaction Count)

Definition: The total number of unconfirmed transactions in the mempool.

Why it matters: Higher count indicates network congestion.

Fee Implications: More pending transactions often result in higher fees required for quick confirmations.

Prerequisites

Ensure you have the following installed:

Rust and Cargo (Install Rust)

Installation

Clone the repository and navigate into the project directory:

git clone <repository-url>
cd mempool-monitor

Usage

Run the monitor using Cargo:

cargo run

Example Output

Mempool Transactions: 52,300
Total vsize: 127,000,000 vB
Estimated Fee Rate: 22 sat/vB

Dependencies

This project uses:

reqwest for making API requests

serde_json for parsing JSON responses

tokio for async runtime support

Ensure these dependencies are included in Cargo.toml:

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }

Security Notes

Do not rely on mempool data for high-value transactions without verification.

Always cross-check fee rates with reliable sources.
