# Dep First Search

> Me: can we have crate?  
> Mom: we have crate at home.

Search your Rust dependency tree for crates.  Don't add new crates for features you already have.

Consider this scenario:

You start a project, add dependencies once in a while, and after a year or two you have a sprawling tree of hundreds of transitive dependencies.  When you reach for a new crate, stop, and check transitive dependencies you already have at home.  You might already be compiling that which you need.

## Installation

```
cargo install depfirstsearch
```

## Usage

From any directory inside your crate or cargo workspace:

```
depfirstsearch REGEX
```

Example:

Q: I need a glob crate and plan to install `wax`, but wait, do I already have a glob crate?

```
$ depfirstsearch glob
glob@0.3.1
	Support for matching file paths against Unix shell style patterns.
```

## How it works

`depfirstsearch` loops through every crate in the `cargo metadata` output, collects stringified name/version/description/keywords, then applies your search term/regex to each, and print the ones that match.
