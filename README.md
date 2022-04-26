# bo

[![Tests](https://github.com/brouberol/bo/actions/workflows/tests.yml/badge.svg)](https://github.com/brouberol/bo/actions/workflows/tests.yml) ![Coverage](https://github-brouberol-coverage.s3.eu-west-3.amazonaws.com/bo/flat.svg)

My (WIP) personal text editor for prose.

https://user-images.githubusercontent.com/480131/131999617-61acc5a2-4055-4cd1-9da1-134ee9e075b4.mp4

## Why?
The goals for `bo` are the following:

- write a non trivial application in Rust for the first time
- create a text editor extremely tailored to my personal tastes, only implementing the Vim navigation features I like and use
- make it _non configurable_

Having a good text editor is paramount for a software engineer, but the defnition of _good_ really varies depending on the context.
I do enjoy Visual Studio for its rich ecosystem, I enjoy Sublime Text for its extreme snappiness, and I enjoy vim for its ubiquitousness.
I tend to favour an editor with plugins/linters/autocompletion when I'm developing code, but when I'm writing prose (blogposts, book chapters...), I would like to use an editor that is as simple as possible and "works" for me, without giving me the opportunity of getting lost in configuration options.

So, something like [Left](https://hundredrabbits.itch.io/left), but with vim-like navigation commands.

## Roadmap

### Navigation

- [x] Navigation with `h`, `j`, `k`, `l`
- [x] Next/previous paragraph (`}`, `{`)
- [x] Next/previous word (`w`, `b`)
- [x] First/last line in document (`g`, `G`)
- [x] First/last character in the line (`0`, `$`)
- [x] Screen navigation (`H`, `M`, `L`)
- [x] First non whitespace character in the line (`^`)
- [x] Support for multi-character commands (e.g. `2j`, `3}`, ...)
- [x] Go to n% in the file (`%`)
- [x] Search text and navigate through matches
- [x] Move cursor by left clicking
- [x] Go to matching symbol, bracket, quote, etc
- [x] Support multiline goto-matching-symbol

### Editing

- [x] Create a new file
- [x] Open an existing file
- [x] Save file `w`
- [x] Rename file `w` `file name`
- [x] Insert character under the cursor
- [ ] block (word, paragraph, line, etc) with both `d` and `c`
- [x] delete a line with `dd`
- [ ] yank/paste a block
- [x] insert newline before/after (`o`, `O`)
- [ ] Replace current character (`r`)
- [ ] Replace search matches
- [x] Remove trailing space at save
- [x] Remove current character

### Options

- [x] toggle line numbers
- [x] toggle word count stats
- [ ] toggle line wrapping

### UX

- [x] Display help
- [ ] Save session file with last known cursor position
- [x] Restore unsaved edits by regularly saving to a hidden swap file
- [ ] Command history, browsable with arrows
- [x] Support Unicode characters
- [x] Redraw rows when the terminal size changes

### Long shot
- [ ] Multiline edition support
- [ ] undo/redo
- [ ] Tab navigation
- [ ] Fuzzy file finder

## Inspiration

I got the inspiration for `bo` by reading about [antirez](https://github.com/antirez)'s editor [`kilo`](https://github.com/antirez/kilo), and am widly basing my work on the excellent [blogpost series](https://www.philippflenker.com/hecto-chapter-1) by Philipp Flenker.

It is called `bo` because I've recently [turned 30](https://www.youtube.com/watch?v=XrOa5hDzXIY).
