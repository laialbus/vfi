# Comments

Not scoped to paths; comments happen everywhere, so this loads every session.

A comment earns its place only by stating what the code cannot: a constraint,
an invariant, a non-obvious why. Everything else is bloat that every future
reader pays for.

Never write a comment that:

- restates what the next line does,
- explains where a value came from when the name could say it,
- defends or narrates the change being made — that belongs in the commit
  message or the PR, not the code,
- marks a conversation ("as discussed", "per review", "NEW in this
  revision").

If a comment restates the code, delete the comment. If the code needs the
comment to be understood, first try making the code say it.

Enforced by: nothing. This is style guidance, not a gate.
