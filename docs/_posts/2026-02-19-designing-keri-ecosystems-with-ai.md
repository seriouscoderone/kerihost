---
title: "Designing KERI Ecosystems with AI"
date: 2026-02-19
categories: [technical, ai]
tags: [ecosystem-design, claude-code, skills, architecture, credentials, governance, tooling, workflow]
description: "Four conversational AI skills walk you from industry analysis to domain implementation. Here's how they work."
status: draft
theme_summary: "Building a KERI ecosystem from scratch is daunting. You need roles, credentials, delegation trees, governance, infrastructure, and domain components — and they all have to fit together. We built four conversational AI design skills that guide you through the entire process, one level at a time. They are opinionated, conversational, and grounded in real KERI architecture. This is how you use them."
---

## The Problem: Where Do You Even Start?

You want to restructure an industry around verifiable credentials. You've read about KERI, ACDC, and the [actual value economy](/blog/2026/02/06/actual-value-economy/). You see the potential. But when you sit down to design the thing — the actual architecture — you hit a wall.

What are the roles? What credentials do they issue? Who delegates authority to whom? What does the governance look like? What infrastructure do you need? What KERI protocol components run inside that infrastructure?

These questions are interdependent. The answers at one level constrain the options at the next. Getting them wrong early means rework later. Getting them right requires holding a lot of context simultaneously — industry knowledge, KERI protocol specifics, infrastructure patterns, and domain implementation details.

We built four AI design skills to hold that context for you.

## The C4 Architecture

The skills follow a layered architecture we call C4 — four levels of design, each producing artifacts that feed the next:

| Level | Skill | What You Design | What It Produces |
|-------|-------|----------------|-----------------|
| **C0** | `/keri:design0-ecosystem` | Industry governance | `ecosystem.yaml`, credential catalog, trust framework |
| **C1** | `/keri:design1-service` | Human-facing service | `system.yaml`, service design narrative |
| **C2** | `/keri:design2-infrastructure` | AWS infrastructure | `stack.yaml`, resource documentation |
| **C3** | `/keri:design3-domain` | KERI protocol components | `domain.yaml`, component specs, data structures |

Each level reads the artifacts from the level above. C1 reads C0's ecosystem.yaml. C2 reads C1's system.yaml. C3 reads C2's stack.yaml. Context flows down. You don't have to re-explain your industry at every level.

The artifacts are YAML specifications — machine-readable, diffable, versionable. The narrative documents are Markdown for humans. Both are stored in `docs/` alongside your code.

## C0: Ecosystem Design

This is where you start. C0 maps an entire industry into KERI-native governance.

Run `/keri:design0-ecosystem` and the skill guides you through eight design questions, one at a time:

1. **Trust intermediaries.** What trust relationships require middlemen today? Which intermediaries add real value vs. which exist only because there was no alternative?
2. **Data duplication.** What data is copied across organizations? Who is the source of truth?
3. **Reconciliation.** What reconciliation processes exist between parties? How long do they take?
4. **Liability boundaries.** Who gets blamed when things go wrong? Are those boundaries clear?
5. **Friction-reducing credentials.** What credentials would eliminate the most daily friction?
6. **Authorized issuers.** Who should issue credentials? What makes an issuer trustworthy?
7. **Privacy requirements.** What data must never be revealed to certain parties?
8. **Regulatory compliance.** What regulations apply? What audits happen today?

The skill doesn't rush you. It asks one question, listens, asks follow-ups, then summarizes what it learned and which artifact fields the answer populates. If you're unsure about something, it suggests patterns from similar industries.

From your answers, it proposes roles, credentials, delegation trees, and governance. You iterate until it's right. Then it generates three files:

- `ecosystem.yaml` — the machine-readable specification (roles, credentials, delegation trees, interoperability, privacy requirements)
- `credential-catalog.md` — every credential with its issuer, holder, verifiers, schema fields, and chaining relationships
- `trust-framework.md` — narrative governance document covering regulatory frameworks, dispute resolution, liability, and privacy

The [genealogy ecosystem](/ecosystems/genealogy/), the [humanitarian service marketplace](/ecosystems/humanitarian-service-marketplace/), and the [small trades cooperative](/ecosystems/small-trades-cooperative/) were all designed this way.

## C1: Service Design

C0 defines the industry. C1 defines what you build for humans inside that industry.

Run `/keri:design1-service` and it reads your ecosystem.yaml, then asks a different kind of question: not "how does the industry work?" but "what pain goes away for a real person?"

There's a jargon rule. If you describe your service using KERI terminology — AIDs, KELs, ACDCs — the skill pushes back:

> "That describes KERI infrastructure, not a human problem. Imagine you are pitching this to someone who has never heard of KERI. What pain goes away for them?"

It keeps pushing until you speak in human terms. "Instant background checks," not "KEL validation." "Identity protection," not "witness pools." This matters because it forces you to articulate the value proposition before you touch any infrastructure.

The skill identifies your actors (2-4 user types), walks you through user journeys for each, then pattern-matches against four canonical service types:

1. **Identity Lifecycle** — secure digital identity management
2. **Credential Verification** — instant background checks
3. **Marketplace Trust** — portable reputation
4. **Compliance-as-a-Service** — automated regulatory compliance

From the user journeys, it derives KERI infrastructure requirements. If a journey involves creating an identity, you need an agent service. If it involves checking for fraud, you need a watcher network. The requirements are justified by the journeys, not assumed.

It also captures your business model, SLA targets, and integration points. The output is `system.yaml` (machine-readable) and `service-design.md` (human-readable narrative).

## C2: Infrastructure

C2 is intentionally thin. Most decisions are already made at C0 and C1.

Run `/keri:design2-infrastructure` and it reads your system.yaml, performs a gap analysis (what stacks are needed vs. what exists), and asks exactly four questions:

1. **Environment** — prod, staging, or dev?
2. **Region** — which AWS region?
3. **Custom domain** — do you need one?
4. **Security requirements** — HIPAA, SOC2, FedRAMP, or standard?

That's it. Everything else auto-fills from your ecosystem and service definitions. Six stack types are available:

| Stack Type | Compute | Database | Purpose |
|-----------|---------|----------|---------|
| witness-pool | ECS Fargate | DynamoDB | Receipt generation |
| watcher-node | Lambda | Aurora Serverless | Duplicity detection |
| agent-service | ECS Fargate | RDS PostgreSQL | Full KERI agent |
| acdc-registry | Lambda | DynamoDB + S3 | Credential management |
| judge-jury | Step Functions + Lambda | DocumentDB | Consensus on duplicity |
| frontend | — | — | Static web app |

Resource sizing adjusts automatically by environment. Dev gets minimal single-AZ. Prod gets multi-AZ, deletion protection, and 30-day backup retention.

## C3: Domain Components

C3 is where KERI protocol logic lives. This is the most technically dense level.

Run `/keri:design3-domain` and it reads all three parent artifacts (ecosystem.yaml, system.yaml, stack.yaml), then guides you through component selection for each stack.

For a witness pool stack, required components are an Event Log Engine and a Witness Service. For an agent service, you need those plus a KERI Agent and OOBI Resolver. The skill knows which components each stack type requires and which are optional.

For each component, the skill asks targeted protocol questions. For a Witness Service: How many witnesses? What KAACE threshold? Public or restricted? For an ACDC Registry: Backed or backerless TEL? Revocation policy? Graduated disclosure?

Then it maps your components to the AWS resources defined at C2, reviews KERI protocol invariants (sequence numbers must increment by exactly one, first-seen rule, etc.), and recommends a runtime:

- **Serverless (Lambda):** Rust with keriox + cesride. Python's keripy depends on LMDB and long-running processes — incompatible with Lambda.
- **Container (Fargate):** Python with keripy/KERIA. The reference implementation with the most complete feature set.
- **Frontend:** TypeScript with signify-ts. Signing at the edge, keys never leave the browser.

C3 enforces one critical boundary: domain components implement *protocol rules*, not *governance rules*. If you propose a rule about who can issue which credentials, the skill redirects you to C0. If you propose a business workflow rule, it redirects you to C1. C3 is deterministic and spec-aligned.

## The Conversation Matters

These skills are not code generators. They're design conversations.

At C0, you're an industry expert and the AI is interviewing you. It draws out your knowledge of trust relationships, liability boundaries, and privacy requirements — then maps them to KERI architecture. At C1, you're a product designer and the AI won't let you hide behind jargon. At C2, the AI handles the infrastructure plumbing so you don't have to. At C3, the AI enforces protocol correctness so your domain components actually work.

The value isn't in the YAML files the skills generate (though those are useful). The value is in the conversation that produces them. Every design question surfaces assumptions. Every follow-up probe reveals edge cases. Every iteration tightens the architecture.

You can always go back. If C1 reveals that your ecosystem needs a role you didn't define at C0, go back and add it. If C3 reveals that your infrastructure can't support a required component, go back to C2. The artifacts are files in your repo — diffable, editable, version-controlled.

## What You Get

After running all four levels for a single service, your `docs/` directory looks like this:

```
docs/
  {ecosystem}/
    ecosystem.yaml
    credential-catalog.md
    trust-framework.md
    {service}/
      system.yaml
      service-design.md
      {stack-1}/
        stack.yaml
        resources.md
        domain/
          domain.yaml
          components.md
          data-structures.md
      {stack-2}/
        ...
```

Every design decision is documented. Every credential has a schema, an issuer, a holder, and a verifier. Every delegation tree has explicit depth limits. Every infrastructure stack has sized resources. Every domain component has mapped state and protocol invariants.

This is not a whitepaper. It's a buildable specification.

## Where We Are

The skills exist and work. The [three ecosystem designs](/ecosystems/) on this site were all produced by `/keri:design0-ecosystem`. You can install the skills from the [keri-skills plugin](https://github.com/seriouscoderone/keri-skills) and start designing your own ecosystem today.

C0 through C3 are conversational design. What comes next — C4, actual implementation — is where the YAML becomes code. That's a different post. But the design skills give you a foundation that's architecturally sound, protocol-correct, and grounded in your industry's actual trust relationships.

If you've been thinking about restructuring your industry around verifiable credentials but didn't know where to start: start with `/keri:design0-ecosystem`. Tell it about your industry. Answer the questions honestly. See what comes out.

**Related:** [Unbundling the Monopoly](/blog/2026/02/19/unbundling-the-monopoly/) | [The Actual Value Economy](/blog/2026/02/06/actual-value-economy/) | ["Why Do I Need the Internet?" — 1990](/blog/2026/02/11/1990-internet-moment/) | [The Map Has a KERI-Shaped Hole](/blog/2026/02/18/the-map-has-a-keri-shaped-hole/)
