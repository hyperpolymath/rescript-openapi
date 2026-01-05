// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 Hyperpolymath

//! ReScript code generation from IR
//!
//! Generates:
//! - Type definitions (records, variants, aliases)
//! - rescript-schema validators
//! - HTTP client functions using fetch

pub mod client;
pub mod schema;
pub mod types;

use crate::ir::ApiSpec;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub output_dir: PathBuf,
    pub module_prefix: String,
    pub generate_schema: bool,
    pub generate_client: bool,
}

/// Generate ReScript code from IR
pub fn generate(spec: &ApiSpec, config: &Config) -> Result<()> {
    fs::create_dir_all(&config.output_dir)?;

    // Generate Types.res - all type definitions
    let types_code = types::generate(spec, config)?;
    let types_path = config.output_dir.join(format!("{}Types.res", config.module_prefix));
    fs::write(&types_path, types_code)?;

    // Generate Schema.res - rescript-schema validators
    if config.generate_schema {
        let schema_code = schema::generate(spec, config)?;
        let schema_path = config.output_dir.join(format!("{}Schema.res", config.module_prefix));
        fs::write(&schema_path, schema_code)?;
    }

    // Generate Client.res - HTTP client functions
    if config.generate_client {
        let client_code = client::generate(spec, config)?;
        let client_path = config.output_dir.join(format!("{}Client.res", config.module_prefix));
        fs::write(&client_path, client_code)?;
    }

    Ok(())
}
