# cargo-goose GitHub Action

A GitHub Action that installs and runs [`cargo-goose`](https://github.com/mozilla-ai/cargo-goose), a semver-aware release helper for Rust workspaces.

This action is a thin wrapper around the `cargo-goose` CLI. It installs the tool and optionally runs it with provided arguments.

## Usage

### Install `cargo-goose` only

If you just want `cargo-goose` available in your workflow:

```yaml
- uses: mozilla-ai/cargo-goose/actions/cargo-goose@v1
```

After this step, `cargo goose` is available for the rest of the job.

### Install and run `cargo-goose`

You can also pass arguments to run `cargo-goose` directly:

```yaml
- uses: mozilla-ai/cargo-goose/actions/cargo-goose@v1
  with:
    args: bump patch
```

This is equivalent to running:

```bash
cargo goose bump patch
```

## Inputs

| Name   | Description                       | Required | Default |
| ------ | --------------------------------- | -------- | ------- |
| `args` | Arguments passed to `cargo goose` | No       | `""`    |

If `args` is not provided, the action will only install `cargo-goose` and do nothing else.

## Examples

### Bump the patch version in CI

```yaml
- uses: actions/checkout@v4

- uses: mozilla-ai/cargo-goose/actions/cargo-goose@v1
  with:
    args: bump patch
```

### Install once, run multiple commands

```yaml
- uses: mozilla-ai/cargo-goose/actions/cargo-goose@v1

- run: cargo goose current-version
- run: cargo goose bump minor
```

## Notes

* This action installs Rust using the stable toolchain.
* `cargo-goose` is installed via `cargo install`.
* The installation is scoped to the current job.
* Calling this action multiple times will reinstall `cargo-goose` unless cached by the workflow.

## Versioning

This action should be referenced using a major version tag:

```yaml
uses: mozilla-ai/cargo-goose/actions/cargo-goose@v1
```

This allows bugfixes and improvements without breaking workflows.

## License

This action is licensed under the same terms as the `cargo-goose` project.
