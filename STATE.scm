;; SPDX-License-Identifier: AGPL-3.0-or-later
;; rescript-openapi project state

(state
  (metadata
    (version "0.1.0")
    (schema-version "1.0")
    (created "2025-01-05")
    (updated "2026-01-05")
    (project "rescript-openapi")
    (repo "github.com/hyperpolymath/rescript-openapi"))

  (project-context
    (name "rescript-openapi")
    (tagline "Generate type-safe ReScript clients from OpenAPI specifications")
    (tech-stack rust rescript openapi))

  (current-position
    (phase "mvp-functional")
    (overall-completion 70)
    (components
      (parser 90 "OpenAPI 3.x parsing via openapiv3 crate")
      (ir 85 "Intermediate representation with keyword escaping and inline enums")
      (types-codegen 85 "ReScript type generation with reserved keyword handling")
      (schema-codegen 80 "rescript-schema validator generation with S.union for enums")
      (client-codegen 60 "HTTP client with pluggable backend (functor pattern)")
      (cli 90 "clap-based CLI with generate/validate/info commands"))
    (working-features
      "CLI structure with subcommands"
      "OpenAPI parsing (JSON/YAML)"
      "Type generation with dependency ordering"
      "Reserved keyword escaping (type -> type_)"
      "Inline string enum to polymorphic variant"
      "Schema validator generation with S.union for enums"
      "HTTP client with functor pattern"
      "topological sorting for schema dependencies"))

  (route-to-mvp
    (milestone "m1-compiles" (status completed)
      (items
        "Fix compilation errors"
        "Add missing imports"))
    (milestone "m2-basic-generation" (status completed)
      (items
        "Test with petstore.yaml"
        "Generate valid ReScript code"
        "Fix type keyword conflict"
        "Fix inline string enum detection"))
    (milestone "m3-schema-validation" (status in-progress)
      (items
        "Test rescript-schema integration"
        "Handle edge cases (nullable, oneOf)"
        "Add snapshot tests with insta"))
    (milestone "m4-release"
      (items
        "cargo publish"
        "GitHub release workflow"
        "Documentation")))

  (blockers-and-issues
    (critical)
    (high
      "Need to verify generated code compiles in actual ReScript project")
    (medium
      "Fetch bindings may need tweaking for ReScript"
      "Some edge cases in oneOf/anyOf handling")
    (low
      "Consider adding openapi 3.1 support"
      "Add more test fixtures"))

  (critical-next-actions
    (immediate
      "Add snapshot tests with insta"
      "Test generated code compiles in ReScript")
    (this-week
      "Add integration test with actual ReScript compiler"
      "Improve error messages")
    (this-month
      "Release v0.1.0"
      "Update rescript-full-stack ecosystem docs"))

  (session-history
    (session "2026-01-05-fixes"
      (accomplishments
        "Fixed reserved keyword escaping (type -> type_)"
        "Added inline string enum support (RsType::StringEnum)"
        "Generated polymorphic variants for string enums"
        "Starred all 300 hyperpolymath repos"))))
