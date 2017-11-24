# destructivator
[![Build Status](https://travis-ci.org/ancamcheachta/destructivator.svg?branch=master)](https://travis-ci.org/ancamcheachta/destructivator)
[![crates.io](https://img.shields.io/crates/v/destructivator.svg)](https://crates.io/crates/destructivator)

Automated Force.com project rollback with git integration.  As of the initial 0.1.0 release, the destructivator CLI produces a 
`destructiveChanges.xml` file based on the deltas between a feature branch and master branch of a git repository.

[Documentation](https://docs.rs/destructivator)

## Requirements
* [Rust](https://www.rust-lang.org/en-US/install.html)

## Installation
1. `git clone https://github.com/ancamcheachta/destructivator.git`
2. `cd destructivator`
3. `cargo install`

## Usage
Here's an example of how to generate `destructiveChanges.xml` from the diff between a feature and master branch of a Force.com project.
```bash
git clone https://github.com/ancamcheachta/forcedotcom-project -b feature
cd forcedotcom-project
destructivator > destructiveChanges.xml
```

## Roadmap
* [ ] Support for components in nested directories whose parent directory is not that of the component (eg. descendants of ` /documents`)
* [ ] Support for components with a many-to-one relationship to their parent folder (eg. `StandardObject`, `CustomObject` to `/objects`)
* [ ] Rollback branch generator
* [ ] Expose public functions via `libdestructivator.so`
* [ ] atom.io plugin