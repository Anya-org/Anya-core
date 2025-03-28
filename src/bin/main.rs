#![feature(edition2021)]
fn main() {
    println!("Anya Platform v{}", anya::version());
    
    #[cfg(feature = "bitcoin_integration")]
    {
        println!("Bitcoin module: ACTIVE");
        println!("Network: {}", anya::bitcoin::current_network());
    }
} 