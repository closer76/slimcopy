# slimcopy
Recursively copy files in a directory tree, with filters defined with gitignore glob syntax. Designed for backup code projects.

## Usage

```cmd
slimcopy.exe [FLAGS] [OPTIONS] <SRC> <DEST>
```

...where `SRC` and `DEST` are path to directories.

Use `--help` to see the complete supported options.

## Filter Rules

When copying, Slimcopy refers to filter rules to decide whether a file (or a directory) should be copied. For convenience,
the rules are defined in [gitignore glob syntax](https://git-scm.com/docs/gitignore). Negative patterns (prefix '`!`') are supported,
but they should be carefully designed to avoid surprising results.

## Default Rule File

If not specified, Slimcopy will search for `.slimcopy_rules` in the `SRC` directory, and use the filter rules defined in that file.

