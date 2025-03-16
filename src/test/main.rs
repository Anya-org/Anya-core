mod bitcoin_tests;
mod dao_tests;
mod web5_tests;
mod ml_tests;
mod system_tests;
mod compliance;

use clap::{App, Arg, SubCommand};
use log::{info, error};

fn main() {
    // Initialize testing environment
    let matches = App::new("Anya-Core Tester")
        .version("3.1.0")
        .author("Anya-Core Team")
        .about("BPC-3 compliant testing framework")
        .subcommand(SubCommand::with_name("component")
            .about("Test specific components")
            .arg(Arg::with_name("component")
                .required(true)
                .help("Component to test: bitcoin, dao, web5, ml")))
        .subcommand(SubCommand::with_name("system")
            .about("Run full system test"))
        .subcommand(SubCommand::with_name("compliance")
            .about("Verify compliance with standards")
            .arg(Arg::with_name("standard")
                .help("Standard to verify: BPC-3, DAO-4, AIS-3, etc.")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("component") {
        let component = matches.value_of("component").unwrap();
        match component {
            "bitcoin" => run_bitcoin_tests(),
            "dao" => run_dao_tests(),
            "web5" => run_web5_tests(),
            "ml" => run_ml_tests(),
            _ => {
                error!("Unknown component: {}", component);
                std::process::exit(1);
            }
        }
    } else if let Some(_) = matches.subcommand_matches("system") {
        run_system_tests();
    } else if let Some(matches) = matches.subcommand_matches("compliance") {
        let standard = matches.value_of("standard").unwrap_or("all");
        verify_compliance(standard);
    } else {
        // Default behavior: run all tests
        run_all_tests();
    }
}

fn run_bitcoin_tests() {
    info!("Running Bitcoin component tests (BPC-3 compliance)...");
    bitcoin_tests::run_all();
}

fn run_dao_tests() {
    info!("Running DAO component tests (DAO-4 compliance)...");
    dao_tests::run_all();
}

fn run_web5_tests() {
    info!("Running Web5 component tests...");
    web5_tests::run_all();
}

fn run_ml_tests() {
    info!("Running ML component tests...");
    ml_tests::run_all();
}

fn run_system_tests() {
    info!("Running full system integration tests...");
    system_tests::run_all();
}

fn verify_compliance(standard: &str) {
    info!("Verifying compliance with {}...", standard);
    match standard {
        "BPC-3" => compliance::verify_bpc3(),
        "DAO-4" => compliance::verify_dao4(),
        "AIS-3" => compliance::verify_ais3(),
        "all" => compliance::verify_all(),
        _ => {
            error!("Unknown standard: {}", standard);
            std::process::exit(1);
        }
    }
}

fn run_all_tests() {
    run_bitcoin_tests();
    run_dao_tests();
    run_web5_tests();
    run_ml_tests();
    run_system_tests();
    verify_compliance("all");
} 