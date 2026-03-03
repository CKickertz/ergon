// Shared preamble injected into all sub-agent role prompts
export const PRIVACY_PREAMBLE = `
## Privacy Rules

This repo is platform code. Personal/instance data never belongs in tracked files.

- Test data uses fictional names only: Alice, Bob, Springfield, Acme Corp
- Never write real names, medical info, credentials, or personal details to files
- Config templates use \`__PLACEHOLDER__\` syntax, never real values
- Session data and operator-specific content belong in instance/ (gitignored)
`;
