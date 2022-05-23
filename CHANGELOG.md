# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

### Features
- Implement autocompletion based on user provided commands (ex: `:deb<TAB>` autocompletes to `:debug`). Multiple completion suggestions can be cycled through, accepted or ignored.
- Add support for Undo last operation, via the `u` command

### Improvements
- Move the cursor on the first character of the currently selected autocompletion suggestion, to highlight it
- Allow multiple search occurences to be found in the same line

### Fixes
- Move to the last available character when deleting a line that is longer that the previous one, with `d`
- Reset the history when opening a new file

## [0.3.3] - 2022/05/01

### Features
- Introduce the `o` short command to open a file
- Display cursor in the message bar while typing a command

### Improvements
- Add strong types around row index VS line number to avoid conversion errors
- Move the cursor to the top left position when opening a new file
- Format the help sections titles in bold and automatically generate the help text

### Fixes
- Fix erratic behavior when navigating into the current view, caused by using the terminal size and not the text area size
- Prevent crashes by using saturating arithmetic operations
- Prevent wrong jumps when inserting/deleting a line after the first half view
- Fix a bug preventing `bo` from being used as the `git` editor

## [0.3.2] - 2022/04/24

### Improvements
- When moving from a line to a shorter one, we now make sure the cursor don't overflow it by moving it to the last character when necessary.
- Display backtrackes on failing tests.

### Fixes
- The hash of the last saved document is always updated when the save is successful, saving us from having to run `:q!` on a freshly saved doc.

## [0.3.1] - 2021/09/05

### Features
- Trailing spaces are now removed on save. Thanks @ilmanzo !
- New `NORMAL` mode command: `J`, joins the current line with the previoue one
- New `:debug` command, that dumps the `Editor` state as JSON to `bo.log`
- `:w <filename>` allows us to save as / rename the file. Thanks @jim4067!

### Improvements
- `bo --version` now outputs both the tag and git hash in debug mode
- The welcome message is automatically hidden as soon as we start writing anything in an empty buffer
- Overal refactoring of the Position/Offset/x_offset typing system into a better design

### Fixes
- Only save documents with a filename to a swap file, saving `bo` from crashing when attempting to generate the swap file name for an empty buffer
- Pasting content with `Ctrl-v` no longer only paste half the content
- A file that was modified, then put back to its original state will now be marked as "dirty" anymore.

## [0.3.0] - 2021/08/15

### Features
- New `NORMAL` mode commands:
    * `A`: go to end of line and enter `INSERT` mode
- Add multiline support to `m` (go to matching symbol)
- Change the style of the cursor depending on the mode
- New command to save and quit: `:wq`
- Expand `~/` in the filename
- `bo <filename>` now opens that file

### Improvements
- Display a marker in in the status bar if the file has unsaved changes
- The terminal size is re-computed at each event, so we can re-draw the editor if the terminal size changed

### Fixes
- Prevent `d` from deleting the last line in the document
- Insertion and deletion now work even when the viewport has a `x` or `y` offset
- Fix many offset-related bugs
- Fix a crash happening when we clicked outside of the document bounds

## [0.2.0] - 2021/08/03

### Features
- New `NORMAL` mode commands:
  *
- New `:help` command, displays the user manual
- `:open <filename>` opens a file
- `:w` saves the file
- New supported mode: `INSERT`, to insert characters in the file

### Improvements
- Unicode characters are now supported
- New `--version` flag

### Fixes
- Many offset/insertion related bugs were fixed


[Unreleased]: https://github.com/brouberol/bo/compare/v0.3.3...HEAD
[0.3.3]: https://github.com/brouberol/bo/compare/v0.3.2...0.3.3
[0.3.2]: https://github.com/brouberol/bo/compare/v0.3.1...0.3.2
[0.3.1]: https://github.com/brouberol/bo/compare/v0.3.0...0.3.1
[0.3.0]: https://github.com/brouberol/bo/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brouberol/bo/compare/v0.1.0...v0.2.0
