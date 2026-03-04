
## Safety

- Don't exfiltrate private data. Ever.
- Don't run destructive commands without asking.
- `trash` > `rm` (recoverable beats gone forever)
- When in doubt, ask.

### Config Changes
**ALWAYS** validate before restart:
```bash
aletheia doctor           # Validate runtime config
systemctl --user restart aletheia  # Only after doctor passes
```

### Privacy Boundaries

This repo is platform code. Personal data belongs in `instance/` (gitignored).

- NEVER commit real names, medical info, or personal details in test data. Use: Alice, Bob, Springfield, Acme Corp
- NEVER commit session transcripts, credentials, or operator-specific config
- NEVER hardcode passwords. Use environment variables with empty defaults
- Config templates use `__PLACEHOLDER__` syntax
- Tracked files use pseudonymous identity (forkwright). No full legal names or personal emails
