# ostree-rs
[![Crates.io](https://img.shields.io/crates/v/ostree.svg)](https://crates.io/crates/ostree)
[![main-docs](https://img.shields.io/badge/docs-main-brightgreen.svg)](https://docs.rs/ostree)

**Rust** bindings for [libostree](https://ostreedev.github.io/ostree/introduction/).

libostree is both a shared library and a suite of command line tools that combines
a "git-like" model for committing and downloading bootable filesystem trees,
along with a layer for deploying them and managing the bootloader configuration.

> **Note**: this crate was renamed from the `libostree` crate.

## Status
Most bindings that can be auto-generated are being auto-generated.
Anything that is not yet supported by the crate probably requires handwritten
bindings. These will most likely be added on an as-needed basis.

## Using

### Requirements
The `ostree` crate requires libostree and the libostree development headers.

On Debian and Ubuntu:
```ShellSession
$ sudo apt-get install libostree-1 libostree-dev
```

On Fedora and CentOS:
```ShellSession
$ sudo dnf install ostree-libs ostree-devel
```

### Installing
To use the crate, add it to your `Cargo.toml`:

```toml
[dependencies]
ostree = "0.20"
```

To use features from later libostree versions, you need to specify the release
version as well:

```toml
[dependencies.ostree]
version = "0.20"
features = ["v2025_2"]
```

## Developing
The `ostree` and `ostree-sys` crates can be built and tested using regular
Cargo commands.

### Generated code
Most code is generated based on the gir files using the
[gir](https://github.com/gtk-rs/gir) tool.

You can update `OSTree-1.0.gir` by directly copying it from a local ostree build.

Or, these parts can be regenerated using
the included Makefile:

```ShellSession
$ make gir
```

Run the following command to update the bundled gir files:

```ShellSession
$ make update-gir-files
```

### Documentation
The libostree API documentation is not included in the code by default because
of its LGPL license. This means normal `cargo doc` runs don't include API docs
for the generated code. Run the `merge-lgpl-docs` Makefile target to include
the API docs in the source so they can be consumed by `cargo doc`:

```ShellSession
$ make merge-lgpl-docs
```

Keep in mind that if you build the crate with the API docs included, it's
effectively LGPL-licensed and you need to comply with the LGPL requirements
(specifically, allowing users of your end product to swap out the LGPL'd
parts).

CI includes the LGPL docs in the documentation build.

### Updating glib-rs
* update `GIR_VERSION` in `Makefile` to the latest gir commit (matching the target glib-rs version)
* `make gir` to regenerate the generated code
* inspect differences in generated code
* update glib-rs dependencies in `Cargo.toml` and `sys/Cargo.toml`

### Updating ostree
* update `OSTREE_VERSION` in `Makefile`
* `make update-gir-files` to update all gir files
* inspect differences in `OSTree-1.0.gir`
* `make gir` to regenerate the generated code
* add any new feature levels to `Cargo.toml`
* update the example feature level in `README.md` in case of a new feature level

### Releases
Releases can be done using the publish_* jobs in the pipeline. There's no
versioning helper so version bumps need to be done manually.

The version needs to be changed in the following places (if applicable):
* in `sys/Cargo.toml` for the -sys crate version
* in the `ostree-sys =` dependency in `Cargo.toml`
* in `Cargo.toml` for the main crate version
* in `README.md` in the *Installing* section in case of major version bumps

Then tag the commit as `ostree/x.y.z` and/or `ostree-sys/x.y.z`. This will run
the crates.io deployment jobs. Main and -sys crate don't have to be released in
lockstep.

## License
The `ostree` crate is licensed under the MIT license. See the LICENSE file for
details.

libostree itself is licensed under the LGPL2+. See its
[docs](https://ostreedev.github.io/ostree/) for more
information.

The libostree GIR file (`gir-files/OSTree-1.0.gir`) is derived from the
libostree source code and is also licensed under the LGPL2+. A copy of the
LGPL version 2 is included in the LICENSE.LGPL2 file.

The remaining GIR files (`gir-files/*.gir`) are from the glib project and
are licensed under the LGPL2.1+. A copy of the LGPL version 2.1 is included
in the LICENSE.LGPL2.1 file.
