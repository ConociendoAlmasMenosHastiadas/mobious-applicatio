this is a lean library that should avoid gui depdencies as its meant to be just the math behind the mobius transform and some associated utilities

the examples can use egui or other libraries but these should be clearly separated by the dev-depedencies tag in Cargo.toml

if you're an agent you should try to track changes made in the corresponding plan file that will be used to track the release.  use a checkbox style to track things that should be done and things that have been done.  add context as you see fit.

this is a mathematical library so exactness and rigor wins over “just make it work”

## Dealing with infinity
This library should gracefully deal with infinity whenever possible.  The definition of the complex plane will be the extended complex plane wich contains R^2 numbers plus infinity.  This means that signed infinities are the same as infinity and if either part of a complex number is it should be treated as infinity.  

## Dealing with invalid values
Avoid panicking.  Whenever possible handle with errors.  MobiousTransofrms should return an error when they fail to create.  the mathematical operations allowed on the transform dont' need the Ok() pattern as they are valid mathematically.  

## final checkout checklist
- update Cargo.toml to propper version
- update Readme.md based on changes made in plan_v*.md.  use a summarized version.
- move current plan into old_plans
- git commits:
  ```powershell
  git add .
  git commit -m "Release v0.1.x - Brief description
  
  Major changes:
  - Key feature 1
  - Key feature 2
  
  Details of significant changes...
  
  Tests: X unit tests + Y doctests passing"
  ```
- push to github:
  ```powershell
  git push origin main
  ```
- push to crates.io (possibly need to get a new key):
  ```powershell
  cargo login
  cargo publish
  ```