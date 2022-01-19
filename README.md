crate-r (Crate Resources)
==========

Crate-r finds all Rust dependencies for a given project allowing a flexible way to build a tags file.

This is intended for use with Vim or Emacs in combination with [universal-ctags](https://docs.ctags.io/en/latest/index.html).

Usage
=====

Pipe all Rust dependencies to ctags

```
crate-r | exctags -L -
```
