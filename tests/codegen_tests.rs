// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 Hyperpolymath

//! Snapshot tests for code generation

use rescript_openapi::{codegen, ir, parser};
use std::path::{Path, PathBuf};

fn generate_from_spec(spec_path: &str) -> (String, String, String) {
    let spec = parser::parse_spec(Path::new(spec_path)).expect("Failed to parse spec");
    let api = ir::lower(&spec).expect("Failed to lower spec");

    let config = codegen::Config {
        output_dir: PathBuf::from("/tmp"),
        module_prefix: "Api".to_string(),
        generate_schema: true,
        generate_client: true,
    };

    let types = codegen::types::generate(&api, &config).expect("Failed to generate types");
    let schema = codegen::schema::generate(&api, &config).expect("Failed to generate schema");
    let client = codegen::client::generate(&api, &config).expect("Failed to generate client");

    (types, schema, client)
}

#[test]
fn test_petstore_types() {
    let (types, _, _) = generate_from_spec("tests/fixtures/petstore.yaml");
    insta::assert_snapshot!("petstore_types", types);
}

#[test]
fn test_petstore_schema() {
    let (_, schema, _) = generate_from_spec("tests/fixtures/petstore.yaml");
    insta::assert_snapshot!("petstore_schema", schema);
}

#[test]
fn test_petstore_client() {
    let (_, _, client) = generate_from_spec("tests/fixtures/petstore.yaml");
    insta::assert_snapshot!("petstore_client", client);
}
