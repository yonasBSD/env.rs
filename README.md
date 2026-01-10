# env.rs

![Licenses](https://github.com/yonasBSD/env.rs/actions/workflows/licenses.yaml/badge.svg)
![Linting](https://github.com/yonasBSD/env.rs/actions/workflows/lint.yaml/badge.svg)
![Testing](https://github.com/yonasBSD/env.rs/actions/workflows/test-with-coverage.yaml/badge.svg)
![Packaging](https://github.com/yonasBSD/env.rs/actions/workflows/release-packaging.yaml/badge.svg)
![Cross-Build](https://github.com/yonasBSD/env.rs/actions/workflows/cross-build.yaml/badge.svg)

![Security Audit](https://github.com/yonasBSD/env.rs/actions/workflows/security.yaml/badge.svg)
![Scorecard Audit](https://github.com/yonasBSD/env.rs/actions/workflows/scorecard.yaml/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_env.rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=yonasBSD_env.rs)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_env.rs&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=yonasBSD_env.rs)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_env.rs&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=yonasBSD_env.rs)
<!--[![codecov](https://codecov.io/gh/yonasBSD/env.rs/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/yonasBSD/env.rs)-->
<!--[![ghcr.io](https://img.shields.io/badge/ghcr.io-download-blue)](https://github.com/yonasBSD/env.rs/pkgs/container/env.rs)-->
<!--[![Docker Pulls](https://img.shields.io/docker/pulls/env.rs/example.svg)](https://hub.docker.com/r/env.rs/example)-->
<!--[![Quay.io](https://img.shields.io/badge/Quay.io-download-blue)](https://quay.io/repository/env.rs/example)-->

![GitHub last commit](https://img.shields.io/github/last-commit/yonasBSD/env.rs)
[![Dependency Status](https://deps.rs/repo/github/yonasBSD/env.rs/status.svg)](https://deps.rs/repo/github/yonasBSD/env.rs)
![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)
[![GitHub Release](https://img.shields.io/github/release/yonasBSD/env.rs.svg)](https://github.com/yonasBSD/env.rs/releases/latest)
[![License](https://img.shields.io/github/license/yonasBSD/env.rs.svg)](https://github.com/yonasBSD/env.rs/blob/main/LICENSE.txt)
<!--[![Matrix Chat](https://img.shields.io/matrix/vaultwarden:matrix.org.svg?logo=matrix)](https://matrix.to/#/#vaultwarden:matrix.org)-->

Load .env files at runtime. Loads files in the following order:

1. .env
2. .env.[$APP_ENV]
3. .env.local
