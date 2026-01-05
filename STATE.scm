;; SPDX-License-Identifier: AGPL-3.0-or-later
;; rescript-openapi project state

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2025-01-05")
    (updated "2025-01-05")
    (project "rescript-openapi")
    (repo "github.com/hyperpolymath/rescript-openapi"))

  (project-context
    (name "rescript-openapi")
    (tagline "Generate type-safe ReScript clients from OpenAPI specifications")
    (tech-stack rust rescript openapi))

  (current-position
    (phase "initial-scaffold")
    (overall-completion 20)
    (components
      (parser 80 "OpenAPI 3.x parsing via openapiv3 crate")
      (ir 70 "Intermediate representation for codegen")
      (types-codegen 60 "ReScript type generation")
      (schema-codegen 50 "rescript-schema validator generation")
      (client-codegen 50 "HTTP client with pluggable backend")
      (cli 90 "clap-based CLI"))
    (working-features
      "CLI structure"
      "OpenAPI parsing (JSON/YAML)"
      "Basic type generation"
      "Schema validator generation"
      "HTTP client with functor pattern"))

  (route-to-mvp
    (milestone "m1-compiles"
      (items
        "Fix any compilation errors"
        "Add missing imports"))
    (milestone "m2-basic-generation"
      (items
        "Test with petstore.yaml"
        "Generate valid ReScript code"
        "Verify types compile"))
    (milestone "m3-schema-validation"
      (items
        "Test rescript-schema integration"
        "Handle edge cases (nullable, oneOf)"
        "Add snapshot tests"))
    (milestone "m4-release"
      (items
        "cargo publish"
        "GitHub release workflow"
        "Documentation")))

  (blockers-and-issues
    (critical)
    (high
      "Need to test with real OpenAPI specs"
      "rescript-schema API may need adjustment")
    (medium
      "Fetch bindings may need tweaking for ReScript")
    (low
      "Consider adding openapi 3.1 support"))

  (critical-next-actions
    (immediate
      "cargo build to verify compilation"
      "Test with petstore.yaml")
    (this-week
      "Add snapshot tests with insta"
      "Test generated code compiles in ReScript")
    (this-month
      "Release v0.1.0"
      "Add to rescript-full-stack ecosystem")))
