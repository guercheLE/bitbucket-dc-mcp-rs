# Bitbucket workflow: monitoring and diagnostics

This is a pointer, not a guided flow: `search` for the specific read-only
signal the user wants (search-index status, the audit log, insights
quality-gate reports and annotations on a commit/PR, or instance
application properties), then `get` and `call` the resolved operation
directly — `operationId`s are not stable across this server's supported
API versions, so confirm the resolved operation's path and schema via
`get` before calling it, the same as every other workflow here. If a
listing or report could be large, and your environment supports running
an isolated sub-task, delegate the fetch and bring back only the
distilled answer rather than pulling a large listing into this
conversation.
