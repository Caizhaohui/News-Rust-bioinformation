use std::path::PathBuf;

use clap::{Parser, Subcommand};
use news_rust_bioinformation::{catalog, digest, discover, fetch, paths, radar_md, readme};

#[derive(Parser)]
#[command(name = "nrb")]
#[command(about = "Living catalog and weekly radar of Rust in bioinformatics")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate data/tools.yaml
    Validate,
    /// Fetch GitHub metadata and write a snapshot
    FetchMetadata,
    /// Generate README.md from tools.yaml and metadata.json
    BuildReadme,
    /// Generate RADAR.md from metadata plus the previous snapshot
    BuildRadar,
    /// Write a Chinese editorial outline
    Digest,
    /// Discover candidate tools without editing tools.yaml
    Discover {
        #[arg(long, default_value_t = 14)]
        days: i64,
        #[arg(long, default_value = "github,biorxiv")]
        sources: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let root = match paths::find_root() {
        Ok(root) => root,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
    let code = match cli.command {
        Commands::Validate => catalog::cmd_validate(&root),
        Commands::FetchMetadata => fetch::cmd_fetch_metadata(&root),
        Commands::BuildReadme => readme::cmd_build_readme(&root),
        Commands::BuildRadar => radar_md::cmd_build_radar(&root),
        Commands::Digest => digest::cmd_digest(&root),
        Commands::Discover {
            days,
            sources,
            output,
        } => discover::cmd_discover(&root, days, &sources, output),
    };
    std::process::exit(code);
}
