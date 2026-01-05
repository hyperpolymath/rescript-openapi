// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 Hyperpolymath

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use rescript_openapi::{codegen, ir, parser};

#[derive(Parser)]
#[command(name = "rescript-openapi")]
#[command(about = "Generate type-safe ReScript clients from OpenAPI specifications")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate ReScript code from an OpenAPI specification
    Generate {
        /// Path to OpenAPI spec (JSON or YAML)
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for generated code
        #[arg(short, long, default_value = "src/api")]
        output: PathBuf,

        /// Module name prefix
        #[arg(short, long, default_value = "Api")]
        module: String,

        /// Generate rescript-schema validators
        #[arg(long, default_value = "true")]
        with_schema: bool,

        /// Generate HTTP client functions
        #[arg(long, default_value = "true")]
        with_client: bool,
    },

    /// Validate an OpenAPI specification
    Validate {
        /// Path to OpenAPI spec
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Print information about an OpenAPI specification
    Info {
        /// Path to OpenAPI spec
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            input,
            output,
            module,
            with_schema,
            with_client,
        } => {
            let config = codegen::Config {
                output_dir: output,
                module_prefix: module,
                generate_schema: with_schema,
                generate_client: with_client,
            };

            let spec = parser::parse_spec(&input)?;
            let ir = ir::lower(&spec)?;
            codegen::generate(&ir, &config)?;

            println!("✓ Generated ReScript code in {:?}", config.output_dir);
        }

        Commands::Validate { input } => {
            let spec = parser::parse_spec(&input)?;
            let diagnostics = parser::validate(&spec);

            if diagnostics.is_empty() {
                println!("✓ OpenAPI spec is valid");
            } else {
                for d in &diagnostics {
                    eprintln!("⚠ {}", d);
                }
                std::process::exit(1);
            }
        }

        Commands::Info { input } => {
            let spec = parser::parse_spec(&input)?;
            println!("Title: {}", spec.info.title);
            println!("Version: {}", spec.info.version);
            if let Some(desc) = &spec.info.description {
                println!("Description: {}", desc);
            }
            println!("Paths: {}", spec.paths.paths.len());

            let schema_count = spec
                .components
                .as_ref()
                .map(|c| c.schemas.len())
                .unwrap_or(0);
            println!("Schemas: {}", schema_count);
        }
    }

    Ok(())
}
