# Privacy Rules

Hard boundaries between platform code (this repo) and personal/instance data.

## Test Data

- NEVER use real names, locations, medical info, or personal details in test fixtures
- Use: Alice, Bob, Charlie, Springfield, Acme Corp
- If you see existing test data with real PII, flag it for replacement

## Session Data

- NEVER commit agent session transcripts to the repo
- Session data belongs in instance/ (gitignored)
- Test corpus must use synthetic conversations only

## Instance vs Repo Boundary

| Belongs in repo | Belongs in instance/ (local only) |
|----------------|----------------------------------|
| Architecture docs, standards, deployment guides | Working agreements, session memory, lessons learned |
| Config templates (`.example` files) | Config with real values, credentials |
| Platform code, tests, CI | Agent workspace content, personal notes |
| Generic examples in specs | Screenshots, Slack transcripts, meeting notes |

## Credentials

- NEVER hardcode passwords, even for local services
- Environment variables with empty string defaults
- Template files use `__PLACEHOLDER__` syntax

## Identity

- Tracked files use pseudonymous identity (forkwright)
- No full legal names, personal email addresses, or work emails in source
- First names alone in design doc narrative are acceptable
- Full name + email combinations never
