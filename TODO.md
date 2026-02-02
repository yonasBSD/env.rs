# TODO

1. Envy (The Most Popular)

envy is the go-to crate for simple environment schemas. It uses serde to map environment variables directly into a Rust struct.

    Best for: Standard applications needing type-safe config.

    How it works: You define a struct with Deserialize and call envy::from_env.

    Validation: Supports basic type validation (e.g., a string won't parse into a u16).

Rust

use serde::Deserialize;

// This is your "Schema"
#[derive(Deserialize, Debug)]
struct Config {
    // Falls back to 'dev' if not set
    #[serde(default = "default_env")]
    env: String,
    port: u16,
}

fn default_env() -> String { "dev".to_string() }

fn main() {
    // Automatically validates and loads
    let config = envy::from_env::<Config>().expect("Invalid Environment");
}
```

2. Figment (The Most Powerful)

Created by the author of the Rocket web framework, figment is a sophisticated configuration library that allows you to "stack" sources (e.g., a .env file, then environment variables, then CLI flags).

    Best for: Complex apps that need to merge multiple config sources.

    Validation: Excellent error reporting that tells you exactly which file or variable caused the schema violation.

3. Envman (Feature Rich)

envman is a newer crate specifically designed for environment management. It includes built-in support for custom validation functions and secret masking.

    Best for: Apps that need custom validation rules (e.g., "port must be > 1024").

    Key Feature: The #[envman(validate = "your_func")] attribute.

4. Dotenv-schema (Strict File Validation)

Unlike the others, dotenv-schema focuses specifically on the .env file itself. It allows you to define a JSON schema to validate your .env file before your application logic even runs.

    Best for: CI/CD pipelines or teams that want to ensure everyone's local .env file matches a shared template.

Comparison Table
Crate	Primary Goal	Validation Method
Envy	Type Safety	Rust Struct (serde)
Figment	Merging Sources	Hierarchical stacking + serde
Envman	Management	Macros + Custom Validators
Dotenv-schema	File Integrity	JSON Schema for .env files

## Deployments

### Deployment Stack
- OpenStack: bare metal provisioning
- Terraform: virtual machine cloud provisioning
- Orchestration: Kubernetes pods
- Services: Docker containers
- Binaries: per OS packages and repositories
- Source code: Gitea / Forgejo
- PKI

### Docker

Add Dockerfile and docker-compose.yaml.

### Helm / Kubernetes

Add helm chart.

### Packages and Binaries
- Flatpack
- AppImage
- Ubuntu Snap
- Nix

### Architecture
- OS
  - Linux
    - Ubuntu / Debian
    - Alpine
    - OpenSUSE
    - Fedora / Rocky
    - Arch
    - Slackware
    - Gentoo
  - BSD
    - FreeBSD
    - OpenBSD
    - NetBSD
  - Mac OS X
  - Windows
- Arch
  - amd64
  - arm v6, v7, v8
  - risc v

### Encryption / PKI
- Sign all assets
  - sigstore
  - minisign [ ed25519 ]
  - GPG [ ed25519 ]
- Checksums
  - BLAKE-3
