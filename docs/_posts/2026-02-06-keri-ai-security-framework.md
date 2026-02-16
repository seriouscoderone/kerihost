---
title: "How KERI + AI Frameworks Will Transform Security"
date: 2026-02-06
categories: [ai, technical]
tags: [keri, ai-agents, delegation, security, threat-model, revocation]
description: "KERI's cryptographic accountability combined with AI agent frameworks eliminates most traditional threat vectors."
status: draft
theme_summary: "KERI's cryptographic accountability combined with AI agent frameworks creates a security model that eliminates most traditional threat vectors."
---

## The Problem with AI Security Today

- AI agents act on behalf of users but lack verifiable identity
- API keys and tokens are static, stealable, and overprivileged
- No audit trail of what AI actually did vs. what it was authorized to do
- Revocation is slow, incomplete, or impossible
- AI actions are repudiable — "the AI did it" with no proof

## Traditional Threat Landscape

### Identity Threats
- Credential theft
- Session hijacking
- Impersonation attacks
- Privilege escalation

### Authorization Threats
- Over-permissioned access
- Lateral movement
- Scope creep
- Stale permissions

### Audit Threats
- Log tampering
- Missing evidence
- Deniability
- Incomplete trails

### AI-Specific Threats
- Prompt injection
- Agent hijacking
- Unauthorized tool use
- Scope violation

## How KERI Changes the Game

### Cryptographic Identity for AI Agents

Every AI agent gets a **delegated AID** (Autonomic Identifier):
- Derived from the principal's identity
- Explicitly scoped authority
- Instantly revocable
- Non-transferable

**Threat eliminated:** Credential theft becomes useless — keys are bound to the delegation chain.

### Non-Repudiable Actions

Every action the AI takes is **signed**:
- Verifiable by anyone
- Attributable to specific agent
- Traceable to delegating principal
- Timestamped and sequenced

**Threat eliminated:** "The AI did it" now has cryptographic proof of exactly what, when, and under whose authority.

### Scoped Delegation

Delegation credentials define **exactly** what the AI can do:
- Action types (read, write, sign, transfer)
- Value limits (max $1000 per transaction)
- Time bounds (expires in 24 hours)
- Counterparty restrictions (only interact with verified parties)

**Threat eliminated:** Over-permissioned access — AI literally cannot exceed its scope.

### Instant Revocation

Revocation is **cryptographically enforced**:
- Immediate effect (no propagation delay)
- Verifiable by any relying party
- No central authority needed
- Audit trail of revocation itself

**Threat eliminated:** Stale permissions — revoked agents are immediately rejected everywhere.

### Tamper-Evident Audit

Every interaction creates **verifiable evidence**:
- Key Event Logs (KEL) are append-only
- Witness receipts provide independent verification
- Duplicity detection catches inconsistencies
- Reconstruction possible without special access

**Threat eliminated:** Log tampering — evidence is cryptographically protected.

## The KERI + AI Security Model

### Before: Traditional AI Agent Security

```
User → API Key → AI Agent → Services
         ↓
   (stealable, static, overprivileged)
```

**Vulnerabilities:**
- Key theft = full access
- No scope enforcement
- Actions are repudiable
- Revocation is incomplete

### After: KERI-Delegated AI Agent

```
User (AID) → Delegation Credential → AI Agent (Delegated AID) → Services
                    ↓
        (scoped, revocable, auditable, non-repudiable)
```

**Security properties:**
- Delegation chain is verifiable
- Scope is cryptographically enforced
- Every action is signed
- Revocation is instant and complete

## Threats Reduced or Eliminated

| Threat | Traditional | With KERI |
|--------|-------------|-----------|
| Credential theft | High risk | Eliminated (keys bound to delegation) |
| Session hijacking | High risk | Eliminated (no sessions, per-action signing) |
| Impersonation | High risk | Eliminated (cryptographic identity) |
| Privilege escalation | Medium risk | Eliminated (scope enforced cryptographically) |
| Over-permissioned access | High risk | Eliminated (explicit delegation scopes) |
| Lateral movement | High risk | Severely reduced (scoped to specific actions) |
| Stale permissions | High risk | Eliminated (instant revocation) |
| Log tampering | Medium risk | Eliminated (tamper-evident logs) |
| Deniability | High risk | Eliminated (non-repudiable signatures) |
| Prompt injection | High risk | Reduced (actions still scoped) |
| Agent hijacking | High risk | Reduced (compromised agent still scoped) |
| Unauthorized tool use | Medium risk | Eliminated (delegation defines allowed tools) |

## Practical Implementation

### AI Agent Onboarding

1. **Principal creates delegation** — Defines scope, duration, constraints
2. **Agent receives delegated AID** — Cryptographically derived from principal
3. **Agent begins operating** — Signs all actions with delegated key
4. **Services verify delegation** — Check scope before allowing action

### Runtime Security

1. **Every request is signed** — Agent's delegated AID signs each action
2. **Services verify scope** — Is this action within delegation bounds?
3. **Witnesses receipt actions** — Independent verification
4. **Logs are append-only** — Tamper-evident audit trail

### Incident Response

1. **Revoke delegation** — Instant, cryptographic, universal
2. **Audit actions** — Complete, verifiable history
3. **Attribute responsibility** — Clear chain from action to principal
4. **Prove scope violation** — If agent exceeded authority, evidence exists

## Why This Matters

### For Organizations

- **Reduced liability** — Clear attribution of AI actions
- **Compliance ready** — Audit trails that satisfy regulators
- **Incident response** — Know exactly what happened
- **Granular control** — AI does only what you authorize

### For Users

- **Trust with verification** — Don't trust AI, verify its actions
- **Instant revocation** — Change your mind, revoke immediately
- **Privacy preserved** — Selective disclosure of what AI can access
- **Accountability** — AI actions are your actions (verifiably)

### For AI Developers

- **Security by design** — Not bolted on, built in
- **Reduced attack surface** — Scoped access limits damage
- **Verifiable behavior** — Prove your AI did what it claimed
- **Interoperability** — Standard delegation works across ecosystems

## The Future: AI That Can Legally Act

KERI + AI frameworks enable something new: **AI that can legally and cryptographically act, not just suggest.**

- Sign contracts (within delegated authority)
- Make payments (within limits)
- Access services (with verifiable credentials)
- Represent users (with explicit, revocable delegation)

All while maintaining:
- **Accountability** — Every action attributable
- **Auditability** — Complete, tamper-evident history
- **Revocability** — Instant, universal, cryptographic
- **Scoping** — AI does exactly what authorized, nothing more

## Conclusion

The combination of KERI's cryptographic accountability with AI agent frameworks doesn't just improve security — it fundamentally changes the threat model. Most traditional attacks become irrelevant when:

- Identity is cryptographic, not credential-based
- Authorization is scoped and enforced, not assumed
- Actions are signed and non-repudiable, not logged and forgettable
- Revocation is instant and universal, not slow and incomplete

This isn't incremental improvement. It's a paradigm shift in how we think about AI security.

{% comment %}TODO: Add diagrams, code examples, and real-world scenarios{% endcomment %}
