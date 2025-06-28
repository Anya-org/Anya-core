// Simple test to check RGB module import

#[cfg(test)]
mod test_rgb_import {
    #[test]
    fn test_rgb_import() {
        // Try to import the RGB module
        use anya_core::bitcoin::layer2::rgb;

        // If we get here, the import works
        println!("RGB module imported successfully");
    }
}
