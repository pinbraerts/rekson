# ⚡ rekson
rectify your json. [Try it in the browser!](https://pinbraerts.github.io/rekson/)

## 🍎 Motivation
I frequently edit [json](https://json.org) files.
It is a completely unusable configuration language with poorly written grammar.
So I came up with a simple program to fix manually edited json.
This is not a formatter, rather it is intended to be used in conjunction with one,
for example [`jq`](https://github.com/jqlang/jq).

## ✨ Features
 - [x] trailing comma (`{,"a":3,} -> {"a":3}`)
 - [x] comma between values (`[1 2] -> [1, 2]`)
 - [x] correct quotes (```{`key`: 'value'} -> {"key": "value"}```)
 - [x] multiline strings
 - [x] correct booleans (`True -> true`)
 - [x] correct nulls (`None -> null`)
 - [x] quote unknown values (`{a:3} -> {"a":3}`)
 - [x] correct colon (`{"a"=3} -> {"a":3}`)
 - [x] insert colon (`{b 4} -> {"b":4}`)
 - [x] replace parenthesis with brackets (`((),()) -> [[],[]]`)
 - [x] sequential io
 - [x] strip single-line comments

## 💡 Plans
 - [ ] [convert numbers](https://github.com/pinbraerts/rekson/issues/10) (`0xff -> 256`)
 - [ ] [COW](https://github.com/pinbraerts/rekson/issues/7) (reduce memory copying)
 - [ ] [autoclose quotes](https://github.com/pinbraerts/rekson/issues/12)
 - [ ] strip multiline comments

## ✅ Pros
 - written in Rust => instant format on write
 - no dependencies => faster to build than to install an npm package
 - can be used as standalone executable
 - can be used as standalone library

## ❌ Cons
 - barebone simplicity => adding new features requires rethinking the architecture
 - single-threaded (for now)
 - no configuration (for now)
 - small testsuite (for now)

## 🚀 Installation

### [crates.io](https://crates.io/crates/rekson)
```bash
cargo install rekson
```

### From [source](https://github.com/pinbraerts/rekson)
```bash
cargo install --git https://github.com/pinbraerts/rekson
```

### Binary
Prebuilt binaries can be found at the [releases](https://github.com/pinbraerts/rekson/releases) page.

## 🛠️ Usage

Pass dirty json to stdin, receive fixed json in stdout.

### [conform.nvim](https://github.com/stevearc/conform.nvim)

```lua
require("conform").setup({
  formatters = {
    rekson = {
      command = "rekson",
    },
  },
  formatters_by_ft = {
    json = { "rekson" },
  })
})
```

## 👀 Alternatives
 - https://github.com/adhocore/php-json-fixer
 - https://github.com/rhysd/fixjson
 - https://github.com/Berkmann18/json-fixer
 - https://github.com/messense/dirty-json-rs
