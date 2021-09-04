# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [0.3.1]

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


[Unreleased]: https://github.com/brouberol/bo/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/brouberol/bo/compare/v0.3.0...0.3.1
[0.3.0]: https://github.com/brouberol/bo/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/brouberol/bo/compare/v0.1.0...v0.2.0
