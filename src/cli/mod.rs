use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod codeql;
pub mod setup;
pub mod build;
pub mod labels;
pub mod ai_labels;
pub mod github;
pub mod clean;

#[derive(Parser)]
#[clap(name = "anya-tools")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run CodeQL analysis
    #[clap(name = "analyze")]
    CodeQlAnalysis(codeql::CodeQlArgs),
    
    /// Set up development environment
    #[clap(name = "setup")]
    Setup(setup::SetupArgs),
    
    /// Build Anya Core
    #[clap(name = "build")] 
    Build(build::BuildArgs),
    
    /// Synchronize labels across repositories
    #[clap(name = "sync-labels")]
    SyncLabels(labels::LabelArgs),

    /// Update AI labeling references
    UpdateAiLabels(ai_labels::UpdateArgs),

    /// Validate AI labels
    ValidateAiLabels(ai_labels::ValidateArgs),

    /// Update GitHub URLs
    UpdateGitHub(github::UpdateArgs),

    /// Clean project
    Clean(clean::CleanArgs),
}
