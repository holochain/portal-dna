[back to README.md](README.md)

# Contributing

## Overview
The purpose of this project is enable cross-DHT calls in Holochain apps


## Development

### Environment

- Developed using rustc rustc `1.71.1 (eb26296b5 2023-08-03)`
- Enter `nix develop` for development environment dependencies.

### Building

Make targets
```
nix develop
[nix-shell$] make portal.dna
```

#### Crate Documentation

```
make build-docs
```


### Release Process
Each release involves

1. (if changed) Publish types
2. (if changed) Use new types crate version and publish sdk
3. (if changed) Publish zomelets
4. Update Holochain Version Map
5. Commit with new version tag and version tags for each subpackage
   - `types-x.x.x`
   - `sdk-x.x.x`
   - `zomelets-x.x.x`
6. Creating a Github release.


#### Publishing Types Crate

https://crates.io/crates/hc_portal_types

```
make preview-types-crate
make publish-types-crate
```


#### Publishing SDK Crate

https://crates.io/crates/hc_portal_sdk

```
make preview-sdk-crate
make publish-sdk-crate
```

#### Publishing Zomelets NPM Package

https://www.npmjs.com/package/@holochain/portal-zomelets

```
make preview-zomelets-package
make publish-zomelets-package
```

#### Github Release
https://github.com/holochain/portal-dna/releases


#### Update Holochain Version Map

Update versions in [docs/Holochain_Version_Map.md](docs/Holochain_Version_Map.md)


### Testing

To run all tests with logging
```
make test-debug
```

- `make test-unit-debug` - **Rust tests only**
- `make test-integration-debug` - **Integration tests only**

> **NOTE:** remove `-debug` to run tests without logging
