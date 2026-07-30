---
paths:
  - "shell/**/*"
---

# Python

## Guide

- https://google.github.io/styleguide/pyguide.html

Consult it when making a decision it covers. Do not read it end to end, and do
not copy it into this repo — a local summary of an upstream guide is a second
source of truth and it will drift.

## Style is guidance, not a gate

No build fails over formatting. Match the guide unless there is a stated reason
not to, recorded in an ADR.

## The shell is presentation

Python here is the shell, and the shell only displays what the engine produces.
Before adding logic to a Python file, check Anchor 1. If the code computes
something rather than showing something, it is in the wrong language and the
wrong crate.

Long work belongs in the engine, handed back through a handle the shell polls.
The interface must never block waiting on it.
