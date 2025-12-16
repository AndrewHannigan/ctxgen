# ctxgen

A CLI tool that generates `AGENTS.md` and `CLAUDE.md` files from a `.context` folder.

## Installation

Install with Homebrew:
```
brew install andrewhannigan/tap/ctxgen
```

See [latest release](https://github.com/AndrewHannigan/ctxgen/releases) for additional installation methods.


## Overview

`ctxgen` reads text files from a `.context` directory (typically at the root of a code repository) and compiles them into a flat markdown files suitable for AI agents. The tool also supports progressive disclosure of context via the `<ctxgen:fold>` tag.

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

The `.context` folder can contain text files organized in any folder structure. File format, file naming, and directory structure do not matter. Organize the `.context` folder however you see fit for your project. Here's an example:

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

When compiled to the AGENTS.md and CLAUDE.md, each file is wrapped in a `<file>` tag:

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

## License

MIT

