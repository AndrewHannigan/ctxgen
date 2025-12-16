# ctxgen

A Rust CLI tool that generates `AGENTS.md` and `CLAUDE.md` files from a `.context` folder.

## Overview

`ctxgen` reads text files from a `.context` directory (typically at the root of a code repository) and compiles them into markdown files suitable for AI agents. The tool:

- Flattens nested folder structures into a single output file
- Wraps each file's contents in `<file>` tags with path information
- Supports fold tags to hide content that agents should read from the source

## Installation

Build from crates.io

```bash
cargo install ctxgen
```

Or build from source:

```bash
git clone https://github.com/andrewhannigan/ctxgen
cd ctxgen
cargo install --path .
```

## Usage

```bash
# Basic usage (uses .context in current directory)
ctxgen

# Specify a custom context directory
ctxgen --context-dir /path/to/.context

# Specify output directory
ctxgen --output-dir /path/to/output

# Full options
ctxgen -c .context -o .
```

## Context Folder Structure

The `.context` folder can contain text files organized in any folder structure:

```
.context/
├── guidelines.txt
├── architecture/
│   ├── overview.txt
│   └── patterns.txt
└── api/
    └── conventions.txt
```

## Output Format

Each file is wrapped in a `<file>` tag:

```xml
<file path="guidelines.txt">
Content of the file...
</file>

<file path="architecture/overview.txt">
Content of nested file...
</file>
```

## Fold Tags

Use `<ctxgen:fold>` tags to hide content in the generated markdown. This is useful for controlling context bloat. Agent can read the folded content using filesystem tools:

```xml
This content is always visible.

<ctxgen:fold>
This detailed content will be replaced with a placeholder.
The agent will be instructed to read the original file to see it.
</ctxgen:fold>

More visible content here.
```

When compiled, the fold is replaced with a placeholder:

```
This content is always visible.

[Folded content: 2 lines (lines 3-4). Read 'filename.txt' for full content.]

More visible content here.
```

Files containing folds will have `has_folds="true"` added to their `<file>` tag:

```xml
<file path="detailed-guide.txt" has_folds="true">
...
</file>
```

## Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--context-dir` | `-c` | `.context` | Path to the context folder |
| `--output-dir` | `-o` | `.` | Output directory for generated files |

## License

MIT

