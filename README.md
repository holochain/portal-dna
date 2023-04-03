
# Portal DNA
A DNA for providing zome function access across networks.


[![](https://img.shields.io/github/issues-raw/holochain/portal-dna?style=flat-square)](https://github.com/holochain/portal-dna/issues)
[![](https://img.shields.io/github/issues-closed-raw/holochain/portal-dna?style=flat-square)](https://github.com/holochain/portal-dna/issues?q=is%3Aissue+is%3Aclosed)
[![](https://img.shields.io/github/issues-pr-raw/holochain/portal-dna?style=flat-square)](https://github.com/holochain/portal-dna/pulls)


### Portal Types Crate
See [portal_types/README.md](portal_types/README.md)


### Holochain Version Map
For information on which versions of this package work for each Holochain release, see
[docs/Holochain_Version_Map.md](docs/Holochain_Version_Map.md)


### Build the WASM
Clone the Github repo
[holochain/portal-dna](https://github.com/holochain/portal-dna) and run

```bash
nix-shell
[nix-shell$] make bundled/portal.dna
```
