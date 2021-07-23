# bo
My (WIP) personal text editor for prose.

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
- [x] First non whitespace character in the line (`^1)
- [x] Support for multi-character commands (e.g. `2j`, `3}`, ...)
- [ ] Search text and highlight matches

### Edition

- [ ] Text edition
- [ ] block (word, paragraph, line, etc) with both `d` and `c`
- [ ] delete a line with `dd`
- [ ] yank/paste a block
- [ ] undo/redo
- [ ] insert newline before/after (`o`, `O`)

### Options

- [x] toggle line numbers
- [x] toggle word count stats

### UX

- [ ] Display help
- [ ] Save session file with last known cursor position
- [ ] Restore unsaved edits by regularly saving to a hidden swap file

## Inspiration

I got the inspiration for `bo` by reading about [antirez](https://github.com/antirez)'s editor [`kilo`](https://github.com/antirez/kilo), and am widly basing my work on the excellent [blogpost series](https://www.philippflenker.com/hecto-chapter-1) by Philipp Flenker.
