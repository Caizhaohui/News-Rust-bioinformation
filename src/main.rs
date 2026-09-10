use std::path::PathBuf;

use clap::{Parser, Subcommand};
use news_rust_bioinformation::{
    catalog, digest, discover, fetch, paths, radar_md, readme, trends, validate,
};

#[derive(Parser)]
#[command(name = "nrb")]
#[command(about = "Rust Bioinformatics Radar CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate data/tools.yaml
    Validate,
    /// Run full repository verification (validate + test README consistency)
    Check,
    /// Fetch GitHub metadata and write a snapshot
    FetchMetadata,
    /// Generate a compact snapshot of repository metadata
    Snapshot,
    /// Calculate 7d/30d/90d trends from snapshots
    Trends,
    /// Generate README.md from tools.yaml and metadata.json
    BuildReadme,
    /// Generate RADAR.md from metadata plus the previous snapshot
    BuildRadar,
    /// Write an editorial outline draft
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
    /// Migrate v1 tools.yaml to v2 schema
    MigrateV2,
}

fn cmd_check(root: &std::path::Path) -> i32 {
    println!("Checking catalog schema...");
    let tools_path = paths::tools_path(root);
    let catalog = match validate::load_and_validate(&tools_path) {
        Ok((cat, report)) => {
            if !report.is_valid() {
                eprintln!("Check failed: {} schema errors found:", report.errors.len());
                for err in &report.errors {
                    eprintln!("  - {err}");
                }
                return 1;
            }
            cat
        }
        Err(err) => {
            eprintln!("Check failed: {err}");
            return 1;
        }
    };

    println!("Checking README generation consistency...");
    let metadata = news_rust_bioinformation::metadata::load_metadata(&paths::metadata_path(root));
    let trends_data = trends::load_trends(&paths::trends_path(root));
    let config = news_rust_bioinformation::config::load_config(&paths::config_path(root));
    let now = catalog::utcnow();

    let generated =
        readme::build_readme_v2(&catalog, &metadata, trends_data.as_ref(), &config, now);
    let readme_path = root.join("README.md");
    if let Ok(existing) = std::fs::read_to_string(&readme_path) {
        // Compare ignoring lines that contain updated date
        let gen_lines: Vec<&str> = generated
            .lines()
            .filter(|l| !l.contains("**Updated:**"))
            .collect();
        let exist_lines: Vec<&str> = existing
            .lines()
            .filter(|l| !l.contains("**Updated:**"))
            .collect();
        if gen_lines != exist_lines {
            eprintln!("Check failed: README.md is out of sync with data/tools.yaml. Run 'nrb build-readme' to update.");
            return 1;
        }
    }

    println!("All checks passed successfully.");
    0
}

fn cmd_snapshot(root: &std::path::Path) -> i32 {
    let metadata = news_rust_bioinformation::metadata::load_metadata(&paths::metadata_path(root));
    let today = catalog::utcnow().date_naive().to_string();
    let compact = trends::create_compact_snapshot(&metadata, &today);
    let path = paths::snapshot_dir(root).join(format!("{today}.json"));
    if let Err(e) = trends::save_compact_snapshot(&path, &compact) {
        eprintln!("Failed to save snapshot: {e}");
        return 1;
    }
    println!("Saved snapshot: {}", path.display());
    0
}

fn cmd_trends(root: &std::path::Path) -> i32 {
    let metadata = news_rust_bioinformation::metadata::load_metadata(&paths::metadata_path(root));
    let snap_dir = paths::snapshot_dir(root);
    let config = news_rust_bioinformation::config::load_config(&paths::config_path(root));
    let trends_data = trends::compute_trends(&metadata, &snap_dir, &config, catalog::utcnow());
    let path = paths::trends_path(root);
    if let Err(e) = trends::save_trends(&path, &trends_data) {
        eprintln!("Failed to save trends: {e}");
        return 1;
    }
    println!(
        "Saved trends: {} ({} repositories)",
        path.display(),
        trends_data.repositories.len()
    );
    0
}

fn cmd_migrate_v2(root: &std::path::Path) -> i32 {
    let path = paths::tools_path(root);
    match news_rust_bioinformation::migration::run_migration(&path, &path) {
        Ok(()) => {
            println!("Successfully migrated {} to v2 schema.", path.display());
            0
        }
        Err(e) => {
            eprintln!("Migration failed: {e}");
            1
        }
    }
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
        Commands::Validate => validate::cmd_validate_v2(&paths::tools_path(&root)),
        Commands::Check => cmd_check(&root),
        Commands::FetchMetadata => fetch::cmd_fetch_metadata(&root),
        Commands::Snapshot => cmd_snapshot(&root),
        Commands::Trends => cmd_trends(&root),
        Commands::BuildReadme => readme::cmd_build_readme(&root),
        Commands::BuildRadar => radar_md::cmd_build_radar(&root),
        Commands::Digest => digest::cmd_digest(&root),
        Commands::Discover {
            days,
            sources,
            output,
        } => discover::cmd_discover(&root, days, &sources, output),
        Commands::MigrateV2 => cmd_migrate_v2(&root),
    };
    std::process::exit(code);
}
