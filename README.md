# tomas
Tool transform multi-line comments a single-line comment.

## Usage

```
$tomas
// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt
// ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation
// ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
// reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
// Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit
// anim id est laborum.

"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
```

1. Input `tomas`
2. Input multi-line comments to be rewritten on a single line.
3. Press Enter key

Let's use pbcopy to translate them.

## Supported formats

- `//`, `///`
- `#`
