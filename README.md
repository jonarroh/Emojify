# Emojify

Emojify is a simple command line tool to conventional commit messages with emojis.

## Requirements

- Rust 1.64.0 or higher
- Git

## Installation

1. Clone the repository
2. Ejecuta cargo build --release
3. Copy the binary to a directory in your PATH

```bash
$ cp target/release/emojify /usr/local/bin
```

## Usage

To use emojify, you must be in a git repository. Then, you can run the command:

```bash
$ emojify

Select an option for commit
> ✨ feat
  🎉 new
  🐛 fix
  📗 docs
  🔧 chore
  🚀 perf
  🚧 wip
  🔥 remove
  💄style
  🔒 security

Enter the scope of the commit:
> scope

Enter the commit message:
> commit message

The commit will be:
> ✨ feat(scope): commit message

Do you want to continue? (y/n)
> y
```
