---
title: "$15 Million and No Identity Layer"
date: 2026-02-13
categories: [identity, technical]
tags: [cto-challenge, credentials, ler, talent-marketplace, education, workforce, verifiable-credentials, interoperability]
description: "The U.S. Department of Education is spending $15M to connect talent to opportunity. The missing piece is identity."
status: draft
theme_summary: "The U.S. Department of Education's Connecting Talent to Opportunity Challenge is a $15 million effort to build state-level talent marketplaces that connect learners, credentials, and employers. It describes the right problem — fragmented credentials, siloed records, no portability. But it never addresses the foundational question: who is this person, and how do we know? KERI answers that question in a way that makes everything else in the challenge actually work."
---

## The Right Problem

The U.S. Department of Education just launched a [$15 million challenge](https://www.cto-challenge.com/) called Connecting Talent to Opportunity. The goal: build state-level talent marketplaces that connect people's skills and credentials to education and job opportunities.

The problem statement is solid. Credentials are fragmented. Learning records are siloed. A certification earned in one state, one system, one school doesn't travel. Employers can't verify what candidates actually know. Learners can't prove what they've actually done.

The challenge asks states to build integrated marketplaces with credential registries, Learning and Employment Records (LERs), and AI-powered tools to translate skills into job matches.

This is genuinely important work. And it's missing the most important piece.

---

## The Question Nobody Asked

The entire challenge revolves around credentials, records, and matching — connecting what someone has learned to what someone needs done.

But before any of that works, you need to answer a prior question:

**Who is this person?**

Not "what username did they create on the state workforce portal." Not "what email address did they register with." Not "what Social Security Number links these records together."

Who is this person, cryptographically, verifiably, portably — in a way that works across states, across systems, across time?

The CTO Challenge describes credential registries. But a credential is only meaningful if you can verify who it was issued to. LERs are only useful if the person presenting them is provably the person who earned them. AI matching is only trustworthy if the identities on both sides — learner and employer — are real.

Identity isn't a feature of this system. It's the foundation. And it's absent from the conversation.

---

## What Happens Without an Identity Layer

We've seen this before. Systems built on top of fragile identity always produce the same failure modes:

### Credential Fraud

Without a cryptographic binding between a credential and the person it was issued to, credentials can be forged, borrowed, or fabricated. A credential registry that can't verify the holder is just a fancier database.

### Record Fragmentation

LERs stored in state systems are only as portable as the identity that links them. If "portability" means "we'll share records between systems that use the same vendor," that's not portability. That's a bigger silo.

### Verification Bottlenecks

Employer asks: "Did this person really earn this certification?" Without a self-sovereign identity layer, that question goes through institutional channels. Phone calls. Emails. Days or weeks. The whole point of a digital marketplace — speed and trust — evaporates.

### Privacy Violations

When identity is system-controlled rather than person-controlled, the person has no say in what gets shared. A talent marketplace that knows everything about you and shares what it decides is not a marketplace. It's a surveillance system with a jobs tab.

### Platform Lock-In

If your learning records, credentials, and employment history are tied to a state-run platform, what happens when you move states? Change careers? The system that was supposed to free you from silos becomes the new silo.

---

## What KERI Provides

KERI doesn't compete with the CTO Challenge. It provides the layer that makes the CTO Challenge's vision actually achievable.

### A Person Is an Identifier

With KERI, a learner isn't an account on a portal. They're a cryptographic identifier — an AID — that they own, control, and carry with them. Not issued by the state. Not dependent on a platform. Theirs.

That identifier becomes the anchor for everything else:
- Credentials issued to it
- Records linked to it
- Reputation built on it
- Interactions signed by it

### Credentials Bind to People, Not Systems

An ACDC (Authentic Chained Data Container) issued to a KERI AID is cryptographically bound to the person it was issued to. You can verify:

- **Who issued it** — The issuer's AID and key history
- **Who holds it** — The holder's AID
- **What it says** — The schema-defined content
- **Whether it's still valid** — Revocation status

No phone calls. No email verification. No "we'll get back to you." Instant, cryptographic, definitive.

### Records Travel with the Person

Learning and Employment Records anchored to a KERI identifier don't live in a state database. They live with the person. Presented selectively. Verified instantly. Portable across states, across systems, across borders.

A certification earned in Alabama is verifiable in Oregon — not because the two states agreed on a data-sharing protocol, but because the credential is cryptographically bound to an identifier the person controls.

### Selective Disclosure by Default

The talent marketplace asks: "Does this person have a welding certification?"

With KERI and ACDCs, the person can prove they have the certification without revealing:
- Where they earned it
- When they earned it
- What their full learning record contains
- Their name, address, or any other personal information beyond what's needed

Privacy isn't an afterthought. It's the architecture.

### No New Silo

KERI doesn't require a central identity provider. It doesn't require the state to run an identity system. It doesn't require learners to create yet another account on yet another platform.

The identifier is self-sovereign. The credentials are portable. The verification is peer-to-peer. No single system needs to be the bottleneck or the gatekeeper.

---

## The Interoperability Problem, Solved Differently

The CTO Challenge emphasizes interoperability — getting different systems to talk to each other. That's the right goal.

But the usual approach to interoperability is: agree on a standard, build connectors, negotiate data-sharing agreements, and hope everyone implements the same spec the same way.

KERI offers a different path: **interoperability through self-sovereignty.**

If the person owns their identifier and carries their credentials, interoperability isn't a system integration problem. It's a presentation problem. The person presents their credentials to whatever system needs them. The system verifies them cryptographically. Done.

No state-to-state data sharing agreements. No vendor lock-in. No "our system doesn't talk to their system." The person is the integration layer.

---

## What the CTO Challenge Could Look Like with KERI

Imagine a version of this challenge where:

**A learner in community college** earns a credential. It's issued as an ACDC to their KERI identifier. They carry it.

**They move to another state** for work. A new employer asks for proof. They present the credential from their phone. The employer verifies it instantly — not through the original school's system, not through a state database, but by checking the cryptographic chain directly.

**They earn more credentials on the job.** Their employer attests to competencies. Those attestations are ACDCs, added to their portable record.

**An AI agent helps them navigate opportunities.** Running locally, on their device, reviewing job descriptions against their credentials. Suggesting which credentials to disclose for which opportunities. Helping them understand where their skills match and where they might need development.

**Ten years later**, they have a rich, portable, verifiable record of everything they've learned and done. Not trapped in any system. Not controlled by any state. Not dependent on any platform still being in business.

That's what the CTO Challenge is trying to build. KERI is how you actually build it.

---

## Utah Is Already Building This

While the CTO Challenge is still asking for proposals, one state is already legislating the identity layer.

Utah's [SB275](https://le.utah.gov/~2026/bills/static/SB0275.html) — the State-Endorsed Digital Identity Program Amendments — just passed committee unanimously and is heading to the Senate floor. It builds on SB260 from 2025, which established the SEDI (State-Endorsed Digital Identity) program. Together, they represent the most serious state-level commitment to self-sovereign digital identity in the country.

### What SEDI Gets Right

The program starts from a principle that sounds obvious but is radical in practice: **identity belongs to the individual, not the state.**

From that principle, SB275 establishes a digital identity bill of rights:

- **Right to use physical ID** — Digital is optional, never compelled
- **Right to selective disclosure** — Share only the attributes needed, not your entire identity
- **Right to be free from surveillance, tracking, and profiling** — Written into statute, not a privacy setting
- **Right to transparency** — Know how the system works, not just that it exists

The ACLU endorsed the program as asking "the right questions." The state's Chief Privacy Officer, Christopher Bramwell, frames it bluntly: whoever controls the key controls the identity. Under SEDI, that's you — not the state.

### What Makes This Different

Most government digital ID programs are surveillance systems with a consent checkbox. SEDI is architecturally different:

- **The state endorses identity; it doesn't own it.** The government's role is attestation, not control.
- **Decentralized storage** prevents the state from tracking usage — even in a breach scenario, there's no central log of who presented what to whom.
- **No mass reissuance** after security incidents — because the architecture doesn't create the single point of failure that requires it.
- **State-controlled data centers in Utah** with federal government access explicitly prohibited.

Utah isn't just talking about privacy-respecting digital identity. They're writing it into law, with enforcement provisions and audit requirements.

### The Connection to CTO Challenge

Here's where it gets interesting: imagine a CTO Challenge proposal built on SEDI's identity layer.

A learner in Utah has a SEDI-endorsed digital identity. Their community college issues a credential — bound to their identity, verifiable by anyone, portable across state lines. They move to Colorado for work. Their credential travels with them. The employer verifies it instantly. No portal. No state-to-state data sharing agreement. No "we'll get back to you."

That's not hypothetical architecture. Utah is building the identity layer right now. SB275's effective date is May 6, 2026 — six days after the CTO Challenge submission deadline.

The timing isn't coincidental. It's convergent. The federal government is asking states to build talent marketplaces. Utah is building the identity infrastructure that makes talent marketplaces actually work.

### Other States Are Watching

Sen. Kirk Cullimore, who sponsored SB275, is actively recruiting other states to adopt similar frameworks. A multistate consortium is forming. If even a handful of states adopt SEDI-style legislation, the interoperability problem the CTO Challenge is trying to solve becomes dramatically simpler — because the identity layer is consistent, privacy-preserving, and person-controlled across state lines.

---

## KERI and SEDI: The Technical Alignment

SEDI's principles map directly onto KERI's architecture:

| SEDI Principle | KERI Implementation |
|---------------|-------------------|
| Identity belongs to the individual | Self-sovereign AIDs — you generate and control your identifier |
| Selective disclosure of attributes | ACDCs with graduated disclosure — share only what's needed |
| Freedom from surveillance and tracking | No central log; verification is peer-to-peer |
| Right to transparency | Open protocol; verifiable key event logs anyone can audit |
| State endorses, doesn't own | State as credential issuer, not identity provider |

SEDI doesn't name KERI in the legislation — and it shouldn't. Good legislation is technology-agnostic. But the architecture SEDI describes — individual key control, selective disclosure, decentralized verification, endorsement without ownership — is what KERI already implements.

The policy framework is being written. The technical implementation exists. What remains is connecting them.

---

## The Honest Assessment

The CTO Challenge is real. The $15 million is real. The deadline is April 30, 2026. States are forming teams right now.

Utah's SEDI program is real. SB275 passed committee unanimously. The effective date is May 6, 2026. Other states are forming a consortium.

KERI is real. The protocol is specified. Implementations exist in Python, Rust, and TypeScript. ACDCs work.

For the first time, the policy layer (SEDI), the federal incentive ($15M CTO Challenge), and the technical infrastructure (KERI) are all converging at the same moment. The question isn't whether these pieces fit together — they obviously do. The question is whether anyone in the room will put them together.

Because $15 million can build a very nice talent marketplace. But without an identity layer, it's just another portal with better AI and the same old problems underneath.

Utah is building the identity layer. KERI is the architecture that makes it work. The CTO Challenge is the funding that could bring them together.

The pieces are on the table.

{% comment %}TODO: Add specific ACDC schema examples for educational credentials and LERs, comparison with W3C Verifiable Credentials and Open Badges 3.0 approaches, technical architecture for a KERI-native talent marketplace, analysis of which CTO Challenge requirements KERI addresses directly, and detailed mapping of SB275 provisions to KERI capabilities{% endcomment %}
