use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Anya Platform v{}", env!("CARGO_PKG_VERSION"));

    #[cfg(feature = "bitcoin_integration")]
    {
        println!("Bitcoin module: ACTIVE");
        println!("Network: {}", anya::bitcoin::current_network());
    }

    Ok(())
}
