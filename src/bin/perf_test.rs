/// Performance testing command line tool
use std::error::Error;

use anya_core::testing::performance::runner::{
    run_comprehensive_test_suite,
    run_targeted_test
};
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "perf_test", about = "Performance testing tool for Anya-Core")]
struct Opt {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run a comprehensive performance test suite
    Comprehensive {
        /// Output directory for reports
        #[arg(short, long, default_value = "reports")]
        output_dir: PathBuf,
    },
    
    /// Run a specific performance test
    Test {
        /// Test name (transaction_throughput, database_access, cache_performance)
        #[arg(short, long)]
        name: String,
        
        /// Number of iterations
        #[structopt(short, long, default_value = "1000")]
        iterations: usize,
        
        /// Output directory for reports
        #[structopt(short, long, parse(from_os_str), default_value = "reports")]
        output_dir: PathBuf,
    },
}

fn main()  -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    
    match opt {
        Opt::Comprehensive { output_dir } => {
            println!("Running comprehensive performance test suite...");
            if let Err(e) = run_comprehensive_test_suite(&output_dir) {
                eprintln!("Error running tests: {}", e);
                std::process::exit(1);
            }
        }
        Opt::Test { name, iterations, output_dir } => {
            println!("Running {} test with {} iterations...", name, iterations);
            if let Err(e) = run_targeted_test(&name, iterations, &output_dir) {
                eprintln!("Error running test: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    println!("Performance testing completed successfully!");
} 
