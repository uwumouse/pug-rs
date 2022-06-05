# Pug.rs
A Rust program that reimplements Pug Template engine

## Roadmap
- [X] CLI params
- [X] List of files (path to directory) instead of single file
- [X] Watch files
- [ ] Better parsing errors / warnings
- [ ] Passing values to the templates
  - [ ] Same data for all rendered files
  - [ ] Allow to pass path to object where values for different files specifed
- [ ] Includes (`--basedir`)
- [ ] `extends` and `block`
- [ ] Some code!
  - [ ] Cariables
  - [ ] Conditionals
  - [ ] Loops
    - [ ] `While`
    - [ ] `Each`
- Mixins
  - [ ] Add attributes
- Rust API to use in real projects
- Ship NPM package to replace original pug (I hope someone will do this...)

## TODO
> Just a short-term list of things I need to do
- [ ] Write tests for existing functions
- [ ] Investigate into how errors are being made
- [ ] Create new system for error messages/warnings

#### Maybe
- [ ] Inline syntax for nested tags: `a: img` => `<a><img /></a>`
- [ ] Exec JS code

- [ ] Pretty print