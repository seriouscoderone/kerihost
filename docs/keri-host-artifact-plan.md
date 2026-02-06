# KERI.host — Master Artifact Plan

## Vision Summary

**KERI.host** is a non-profit, reference-grade "ecosystem foundry" for KERI-native economies. It is not a SaaS provider, wallet company, or workflow engine. It is an open, forkable reference implementation — a pattern library for building economies, communities, and governance structures on cryptographic accountability instead of centralized control.

The primary audience is **community organizers and entrepreneurs** (economy builders). Developers are the supporting cast. The initial phase is **narrative-first, no code** — building shared understanding before shared infrastructure.

> "You are not selling technology. You are teaching people how to organize. KERI simply becomes the inevitable tool once they try."

---

## Core Design Constraints (Non-Negotiable)

These constraints govern every artifact produced:

| # | Constraint | Meaning |
|---|-----------|---------|
| 1 | **Reference > Platform** | Forkable, self-hostable, documented as patterns. If KERI.host disappears, nothing breaks. |
| 2 | **Optional Hosting, Never Required** | Every demo must be runnable without KERI.host infrastructure. |
| 3 | **Simulated but Real** | Demos are flight simulators — legally plausible, structurally identical to production, swappable for real endpoints. |
| 4 | **Subjective, Plural Reputation** | No universal score. Reputation is contextual, community-defined, algorithmically diverse. |
| 5 | **Regulator-Friendly Framing** | Show audit trails, evidence, accountability, reduced compliance cost — without surveillance. |
| 6 | **Cooperation, Not Competition** | Non-profit, cost-covering, no ecosystem wars, no official endorsements. |

---

## Phase 0 — Foundations & Narrative (Months 0–3)

### Artifact 1: The KERI.host Manifesto

**Type:** Essay (1–2 pages)
**Working Title:** *"Building Communities Without Gatekeepers"*
**Purpose:** The north star document. Sets tone, philosophy, and intent for everything that follows.

**Contents:**
- Why communities struggle at scale
- Why platforms aren't neutral
- Why we need infrastructure that gets out of the way
- Why this is not about replacing humans with tech
- Cooperation over competition
- Economics without extraction
- What KERI.host refuses to become

**Voice:** Human-first, anti-hype, grounded, quietly radical. No crypto vocabulary. No jargon unless it earns its keep.

---

### Artifact 2: "What We Don't Do" Page

**Type:** Single page / reference document
**Purpose:** Prevent confusion and build trust fast by explicitly stating what KERI.host is *not*.

**Contents:**
- No identity provider
- No platform monopoly
- No global reputation system
- No forced wallets
- No endorsements or "official" ecosystems
- Cannot revoke or invalidate anyone's AID
- Cannot be a root authority for credentials
- Cannot decide "who is legitimate"
- Cannot act as gatekeeper to participation

---

### Artifact 3: Threat Model Document

**Type:** Reference document
**Purpose:** Identify and design against the failure modes that would corrupt the mission.

**Sections:**
1. **Capture** — One entity quietly becomes the center. *Defenses:* easy exit, multiple witnesses, forkable infra, no exclusive credentials.
2. **Centralization** — Convenience causes everyone to rely on one place. *Defenses:* always show alternatives, actively promote self-hosting, celebrate others doing it better.
3. **Wallet Hell** — Incompatible wallets and tools. *Defenses:* interoperability standards, minimal UX assumptions, don't brand wallets as identity.
4. **Foundation Dominance** — The non-profit becomes "the voice of truth." *Defenses:* curate don't decide, no endorsements, public disagreement is okay.

---

### Artifact 4: Pattern Vocabulary Document

**Type:** Glossary / concept reference
**Purpose:** Define the recurring patterns in plain language so all subsequent artifacts use consistent terminology.

**Key Patterns to Define:**
- "No Central Workflow" — Coordination without orchestration
- "Delegated AI" — AI agents with explicit, scoped, revocable authority
- "Subjective Reputation" — Community-defined, algorithmically diverse reputation
- "Ecosystem Autonomy" — Each ecosystem defines its own witnesses, trust anchors, credential issuers, reputation logic, and governance norms
- "Offer → Accept → Disclose → Attest" — The canonical KERI flow pattern
- "Negative Capability" — Power you intentionally do not build

---

## Phase 0 — Content Architecture (Sections A–E)

These five sections form the **narrative backbone** of KERI.host's public content. Every subsequent artifact slots into one of these sections.

### Artifact 5: Section A — "The Problem You Already Have"

**Type:** Essay series (3–4 pieces)
**Purpose:** Describe problems the audience already feels — before mentioning KERI at all.

**Candidate Essays:**
- Why every growing community eventually centralizes power
- Why platforms start helpful and end extractive
- Why trust doesn't scale, but accountability can
- Why compliance hurts small organizations the most

**Rule:** No KERI mentioned yet. These should feel uncomfortably familiar to the reader.

---

### Artifact 6: Section B — "A Different Way to Organize"

**Type:** Essay series (3–4 pieces)
**Purpose:** Introduce *patterns*, not technology. Social architecture concepts.

**Concepts to Introduce:**
- Authority that is explicit and revocable
- Coordination without a central owner
- Proof instead of promises
- Local rules, global interoperability
- Independence without isolation

**Rule:** Still no deep KERI terms. This is about how humans can organize differently.

---

### Artifact 7: Section C — Ecosystem Stories

**Type:** Narrative walkthroughs (9 total, start with 3 flagship)
**Purpose:** The heart of the content. Each story follows a consistent template so readers see themselves in the narrative.

**Story Template:**
1. **The People** — A community organizer, small business owner, city clerk, volunteer coordinator
2. **The Old Way** — Spreadsheets, email chains, platforms, manual audits, trust by reputation
3. **The Breaking Point** — Growth, conflict, fraud, burnout, regulation
4. **The New Pattern** — Everyone keeps their own records, agreements are explicit, authority is delegated not assumed, proof travels but control doesn't
5. **What Changed** — Less drama, less admin, more autonomy, fewer middle people

**Only after the story lands:** "This pattern is enabled by KERI."

**Priority Flagship Stories (Write First):**

| # | Story | Proves |
|---|-------|--------|
| 7a | Homeschool network run by parents | Personal / small community |
| 7b | Small trades cooperative (concrete, welding) | Economic / blue collar |
| 7c | City permit office | Civic / institutional |

**Remaining Ecosystem Stories (Write Later):**

| # | Vertical | Key Themes |
|---|----------|------------|
| 7d | Universities | Credentials, delegated authority, cross-institution verification, revocation |
| 7e | Municipalities (full) | Permits, licenses, procurement, contractor reputation, anti-bribery |
| 7f | Auto Loans / Lending | Borrower AID, vehicle AID, loan-as-ACDC, AI payment agent, delegated repossession |
| 7g | KERI Commerce | Merchant AIDs, inventory credentials, proof-of-delivery, dispute resolution |
| 7h | Service Orgs (Red Cross, United Way) | Volunteer credentials, donation traceability, delegated field authority |
| 7i | Female-Led Community Orgs | Child safety credentials, event permissions, selective disclosure by default |
| 7j | Male-Led Orgs (Sports → National) | Team rosters, referee authority, anti-cheating, cross-league interop |
| 7k | Reputation & Credit | Legacy credit ingestion, native reputation graphs, bring-your-own-score |

---

### Artifact 8: Section D — "What This Enables for Builders"

**Type:** Essay / guide
**Purpose:** Developers enter as *enablers*, not heroes. Position developers as "infrastructure gardeners, not platform owners."

**Framing:**
- "If you want to build tools for communities like this…"
- "If you want to offer services without becoming a gatekeeper…"
- "If you want to help communities coordinate without owning them…"

---

### Artifact 9: Section E — "What KERI.host Actually Is"

**Type:** Positioning page
**Purpose:** Very explicit, very humble description of what KERI.host provides.

**Contents:**
- A reference library
- A storytelling hub
- A place to learn patterns
- Optional infrastructure
- Never a required dependency

---

## Phase 0 — Specialized Documents

### Artifact 10: "What If KERI Wins?" Essay Series

**Type:** Speculative narrative essays
**Purpose:** Imagination scaffolding — not technical docs.

**Candidate Essays:**
- What if cities issued permits this way?
- What if AI could legally act, but only by delegation?
- What if reputation wasn't global?
- What if nobody owned the process?

---

### Artifact 11: "No Central Workflow" — Explained for Non-Technical People

**Type:** Explainer essay
**Working Title:** *"What if nobody owned the process?"*
**Purpose:** Make the strongest technical differentiator accessible to community builders.

**Contents:**
- Why centralized workflows fail across organizations
- How agreements can still move forward without a shared system
- Why independence actually improves cooperation
- The canonical flow: Offer → Accept → Disclose → Attest (explained in human terms)

---

### Artifact 12: Regulator & Auditor Narrative

**Type:** Soft persuasion piece
**Working Title:** *"How Accountability Can Be Stronger Without Surveillance"*
**Purpose:** Quietly reassure cities, NGOs, auditors, and risk-averse partners.

**Key Narrative:**
*"Here's how a municipal auditor could reconstruct an entire procurement process without asking permission, without special access, and without anyone knowing they looked."*

---

## Phase 1 — Architecture Documentation (Months 3–6)

### Artifact 13: KERI.host Architecture Document ("The Spine")

**Type:** Technical reference / architecture overview
**Purpose:** Document the three-layer infrastructure that everything hangs on.

**Layer 1 — KERI Primitives:**
- AIDs (individuals, orgs, agents, devices)
- Delegated AIDs (AI agents, service bots)
- Multi-sig AIDs (boards, councils)
- Witnessing (own + external)
- Watchers / duplicity detection

**Layer 2 — Interaction Primitives:**
- OOBIs
- Event streaming
- Receipt flows
- Escrowed interactions (offers, accepts, disclosures)

**Layer 3 — Meaning:**
- ACDCs (credentials, roles, permissions, attestations)
- Schema registries
- SAIDs as stable semantic anchors
- Contractual disclosure patterns

**Key Insight:** This quietly replaces OAuth, SAML, IDPs, workflow engines, and most integration platforms.

---

### Artifact 14: Cross-Org Workflow Without Orchestration — Technical Pattern

**Type:** Architecture pattern document
**Purpose:** Deep dive into the "no central workflow engine" pattern with concrete flow diagrams.

**Demonstrates:**
- Each org runs its own software
- No shared database or workflow engine
- No super-admin
- Only cryptographically provable state transitions
- Applied to: procurement, permitting, hiring, lending, licensing, membership onboarding

---

### Artifact 15: AI as First-Class KERI Actor — Specification

**Type:** Design specification
**Purpose:** Define how AI agents operate as delegated KERI participants.

**Contents:**
- AI agents with delegated AIDs — explicit, scoped, revocable authority
- Non-repudiable AI actions
- AI capabilities: execute contracts, negotiate offers, perform disclosures, integrate services via ACDC schemas
- AI Integration-on-the-fly: read schema → compose disclosure → establish OOBI trust → execute without pre-integration
- Key positioning: "AI that can legally and cryptographically act, not just suggest."

---

### Artifact 16: Anti-Corruption & Duplicity Case Studies

**Type:** Case study collection
**Purpose:** Show where KERI becomes unavoidable — corruption detection.

**Corruption Scenarios:**
- Permit bribery
- Procurement kickbacks
- Vendor favoritism
- Credential forgery
- Identity impersonation

**KERI-Native Detection Mechanisms:**
- Duplicity proofs
- Inconsistent logs
- Missing receipts
- Illegitimate delegation chains

**For each scenario, show:** How corruption becomes detectable, how honest actors are protected, how blame is precise (not collective).

---

## Phase 2 — Ecosystem Demonstrators (Months 6–12)

### Artifact 17: Five Deep Demo Specifications

**Type:** Detailed specification per demo vertical
**Purpose:** Structurally identical to production — flight simulators, not PowerPoints.

| Demo | Focus |
|------|-------|
| Municipality | Permits + procurement |
| Trades / Blue Collar | Certs + jobs |
| University | Credentials |
| Service Org | Volunteers |
| KERI Commerce | Merchant flows |

**Each demo spec includes:**
- Repository structure
- Architecture diagram
- Trust boundaries
- Failure modes
- Regulator/auditor view
- "What breaks if centralized" comparison

---

## Phase 3 — AI & Reputation Layer (Months 12–18)

### Artifact 18: Reputation Marketplace Design

**Type:** Design document
**Purpose:** Specify how subjective, plural reputation works in practice.

**Contents:**
- Algorithm marketplace concept
- Bring-your-own-reputation flows
- Legacy credit ingestion as attestations
- Native reputation graphs
- Explicit "no universal score" stance
- Community-defined reputation logic

---

### Artifact 19: AI Agent Delegation Framework

**Type:** Framework specification
**Purpose:** Define patterns for AI agents with delegated AIDs operating across ecosystems.

**Contents:**
- Contract execution demos
- Delegation scope and revocation patterns
- Cross-ecosystem AI agent interoperability
- Audit trails for AI actions

---

## Phase 4 — Community Multiplication (Months 18–24)

### Artifact 20: "Start Your Own KERI.host" Guide

**Type:** Comprehensive guide
**Purpose:** Enable others to replicate without permission.

**Contents:**
- Step-by-step self-hosting guide
- Federation norms (not rules)
- Governance templates
- Case studies of independent deployments
- How to run without KERI.host infrastructure

---

### Artifact 21: KERI.host Homepage Structure

**Type:** Information architecture / wireframe document
**Purpose:** Design the public face of KERI.host as a narrative hub.

**Suggested Structure:**
- Hero: "Building Communities Without Gatekeepers"
- Section A teasers (the problems)
- Section C story previews (the ecosystem stories)
- "What We Don't Do" callout
- Builder entry point (Section D)
- "What KERI.host Actually Is" (Section E)

---

## Artifact Summary & Priority Matrix

| Priority | Artifact | Phase | Type |
|----------|----------|-------|------|
| 🔴 1 | Manifesto | 0 | Essay |
| 🔴 2 | "What We Don't Do" Page | 0 | Reference |
| 🔴 3 | 3 Flagship Ecosystem Stories | 0 | Narratives |
| 🔴 4 | "No Central Workflow" Explainer | 0 | Essay |
| 🟡 5 | Regulator Narrative | 0 | Essay |
| 🟡 6 | Threat Model Document | 0 | Reference |
| 🟡 7 | Pattern Vocabulary | 0 | Glossary |
| 🟡 8 | Section A Problem Essays | 0 | Essay series |
| 🟡 9 | Section B Pattern Essays | 0 | Essay series |
| 🟡 10 | "What If KERI Wins?" Series | 0 | Essays |
| 🟢 11 | Architecture Document | 1 | Technical |
| 🟢 12 | Cross-Org Workflow Pattern | 1 | Technical |
| 🟢 13 | AI Actor Specification | 1 | Technical |
| 🟢 14 | Anti-Corruption Case Studies | 1 | Case studies |
| 🔵 15 | 5 Deep Demo Specs | 2 | Specifications |
| 🔵 16 | Remaining 8 Ecosystem Stories | 2 | Narratives |
| 🟣 17 | Reputation Marketplace Design | 3 | Design doc |
| 🟣 18 | AI Delegation Framework | 3 | Framework |
| ⚪ 19 | "Start Your Own" Guide | 4 | Guide |
| ⚪ 20 | Homepage Structure | 0–1 | IA / Wireframe |
| ⚪ 21 | Builder Narrative (Section D) | 1 | Essay |
| ⚪ 22 | Positioning Page (Section E) | 0–1 | Reference |

**Legend:** 🔴 Write First (30–60 days) · 🟡 Write Next (60–90 days) · 🟢 Phase 1 · 🔵 Phase 2 · 🟣 Phase 3 · ⚪ Ongoing/Flexible

---

## Legal Structure Path

**Phase 0–1:** Option A — Operate informally as a personal project with a public mission. Publish a manifesto, transparency statement, and "future non-profit intent" note. Cost: $0.

**Phase 2:** Option B — Incorporate as a 501(c)(3) non-profit. State filing ($50–200) + IRS Form 1023-EZ (~$275). Total: ~$300–1,200. Done 100% online in 2–6 weeks.

---

## Success Metrics (2-Year Horizon)

- Number of independent KERI deployments
- Number of active org AIDs
- Number of cross-org workflows
- Number of delegated AI agents
- Number of third-party auditors relying on KERI data
- Number of municipalities piloting real processes
- Number of communities that say "We should build this" after reading the stories
