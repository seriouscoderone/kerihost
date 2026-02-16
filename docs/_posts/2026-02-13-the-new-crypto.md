---
title: "The New Crypto"
date: 2026-02-13
categories: [technical, economy]
tags: [keri, blockchain, cryptography, mychips, credit-clearing, key-management, post-quantum]
description: "KERI isn't a blockchain killer. It's what comes after we stop confusing the ledger with the cryptography."
status: draft
theme_summary: "Blockchain conflated two things that should have stayed separate: cryptographic proof and global consensus. KERI takes the cryptography — the part that actually matters — and drops the chain. What emerges isn't a better blockchain. It's a new breed of crypto entirely, and systems like MyCHIPs show what becomes possible when you build on these primitives instead."
---

## Blockchain Solved the Wrong Problem

Here's a heretical thought: blockchain's greatest contribution to technology wasn't the chain. It was normalizing the idea that ordinary people could use cryptography to establish trust without asking permission from institutions.

That was revolutionary. That mattered.

But then we spent fifteen years obsessing over the chain — the global ordering, the consensus mechanisms, the gas fees, the mining, the proof-of-work, the proof-of-stake — and forgot to ask a basic question:

**Do we actually need all of that?**

---

## Two Things That Got Tangled Together

Blockchain conflated two fundamentally different capabilities:

**1. Cryptographic proof** — The ability to mathematically verify that something happened, that someone said something, that a key controls an identifier. This is pure math. It's elegant. It works.

**2. Global consensus** — The requirement that every participant in the network agrees on the same ordered history. This is where all the problems live: scalability limits, energy consumption, transaction costs, finality delays, governance wars.

Blockchain bundled these together and told us they were inseparable.

They're not.

---

## What If You Kept the Crypto and Dropped the Chain?

That's KERI.

KERI (Key Event Receipt Infrastructure) provides cryptographic proof of identity, key management, and event history — without a blockchain. Without a shared ledger. Without global consensus.

How? By recognizing a simple truth: **you don't need the entire world to agree on your key history.** You just need the people who care about it to be able to verify it.

Your Key Event Log is yours. Your witnesses attest to it. Anyone who needs to verify it can. No one needs to mine a block or pay a gas fee to make that happen.

Same cryptographic guarantees. None of the overhead.

---

## "But That's Not Crypto"

Depends on what you mean by crypto.

If "crypto" means speculative tokens on a shared ledger — then no, KERI isn't that.

If "crypto" means **using cryptography to establish trust, prove identity, and enable secure transactions between parties who don't inherently trust each other** — then KERI is more crypto than most of what calls itself crypto.

KERI uses:
- Ed25519 and ECDSA for signing
- Blake3 and SHA-3 for hashing
- Pre-rotation for post-quantum resistant key management
- CESR for composable cryptographic encoding

This isn't crypto-adjacent. This is cryptography. The real thing. Applied to the problems that actually matter.

---

## What Blockchain Couldn't Do

For all its promise, blockchain hit walls that no amount of Layer 2 solutions or consensus redesigns could fix:

**Scalability.** Every node validates every transaction. This is a feature for global consensus. It's a disaster for everyday use.

**Privacy.** Public ledgers are, well, public. The entire history is visible. "Pseudonymous" turned out to mean "traceable by anyone with a block explorer."

**Key management.** Lose your private key, lose everything. No rotation. No recovery. No delegation. Your entire identity and economic history, tied to a single secret you'd better never lose or have stolen.

**Interoperability.** Ethereum can't talk to Bitcoin can't talk to Solana. Each chain is its own island with its own rules, its own tokens, its own community politics.

**Governance.** Who decides the rules? Miners? Stakers? Developers? The foundation? Hard forks happen because there's no clean mechanism for disagreement.

---

## What KERI Does Instead

| Blockchain | KERI |
|-----------|------|
| Global consensus required | Local verification sufficient |
| One key forever (or complex multisig) | Pre-rotation: commit to your next key before you need it |
| Lose key = lose everything | Key rotation, recovery, and delegation built in |
| Public ledger (privacy problem) | Your log is yours; selective disclosure by default |
| Scalability limited by consensus | No consensus bottleneck; scales with participants |
| Chain-specific identity | One identifier, works everywhere |
| Governance wars | Each ecosystem governs itself |

KERI doesn't compete with blockchain. It operates in a different architectural space entirely. It's not a better chain. It's what you build when you realize you don't need a chain.

---

## MyCHIPs: What Builds on Top

Here's where it gets interesting.

[MyCHIPs](https://github.com/gotchoices/MyCHIPs) is an open-source protocol for digital money based on private credit — not tokens, not coins, but credit agreements between parties, cleared through a distributed network.

Think of it this way: if blockchain-based coins are like crypto-equity (speculative, volatile, traded), a MyCHIPs credit is like a crypto-bond — a promise between two parties, quantified, clearable, and grounded in actual relationships.

The system uses a "lift" protocol — a distributed credit-clearing mechanism where sites discover and settle circular debt pathways across the network with no central authority. No mining. No gas fees. No global ordering.

Sound familiar?

MyCHIPs and KERI share a philosophical foundation: **you don't need global consensus to have trustworthy systems.** You need good cryptography, verifiable identities, and local agreements that compose into larger networks.

KERI provides the identity layer — who are you, and can I verify that?

Systems like MyCHIPs can provide the economic layer — what do we owe each other, and how do we clear it?

And ACDCs provide the contract layer that ties them together.

### ACDCs: The Contract Between People

A MyCHIPs tally is fundamentally a financial contract between two people. An ACDC (Authentic Chained Data Container) is a cryptographically verifiable container for exactly that kind of agreement.

Think about what a credit agreement actually needs:
- **Identifiable parties** — both sides have KERI identifiers
- **Defined terms** — the schema describes what's being agreed to
- **Verifiable issuance** — signed by the issuer, anchored to their key history
- **Selective disclosure** — share the terms with a third party without exposing everything
- **Revocability** — agreements can be completed, cancelled, or updated

ACDCs do all of this natively. A MyCHIPs credit relationship expressed as an ACDC isn't just a record of a transaction — it's a cryptographically bound contract between two identified parties, verifiable by anyone who needs to see it, private from everyone who doesn't.

This is what blockchain tried to do with smart contracts, but couldn't do cleanly because the contracts were tied to a specific chain, visible to everyone, and governed by whoever controlled the consensus mechanism. ACDCs are chain-independent, privacy-preserving, and governed by the parties involved.

**Contracts between people. Not contracts on a platform.**

Neither requires a blockchain. All of it requires cryptography. The real kind.

---

## A New Breed

The blockchain era taught us something important: cryptography can enable trust between strangers. That lesson is permanent. It doesn't go away.

But the specific mechanism — the globally ordered, consensus-driven chain — was one implementation of that idea. The first serious attempt. Not the last one, and not the best one.

What's emerging now is a new generation of cryptographic systems that keep the insight and drop the baggage:

- **Identity without a chain:** KERI provides self-sovereign identifiers with key rotation, delegation, and recovery — no ledger required.
- **Money without a chain:** MyCHIPs provides credit clearing through distributed trust networks — no mining required.
- **Credentials without a chain:** ACDCs (Authentic Chained Data Containers) provide verifiable credentials anchored to KERI identifiers — no token required.
- **Reputation without a chain:** Subjective, community-defined reputation graphs that don't need global agreement — because reputation was never global to begin with.

This isn't anti-blockchain. It's post-blockchain. The same way the web wasn't anti-dial-up — it was what came next.

---

## The Honest Assessment

KERI is real. The protocol is specified. Implementations exist in Python, Rust, and TypeScript. The cryptography is sound. The architecture handles key rotation and post-quantum concerns that blockchain hasn't seriously addressed.

MyCHIPs is real. The protocol is specified. The credit clearing algorithm works.

What's early: the ecosystem. The tooling. The adoption. The network effects that make any of this matter to ordinary people.

We're at the infrastructure stage. The equivalent of building roads before anyone has a car. That's honest. That's where we are.

But the roads are being built with better engineering than last time. And this time, nobody has to pay a toll to a miner just to drive on them.

---

## What This Means for You

If you invested in the idea of blockchain — the idea that cryptography could change how trust works, how money works, how identity works — you weren't wrong.

You were early.

The technology just wasn't there yet. The chain was a necessary first step, the proof of concept that showed the world what cryptography could do. But it came with compromises that were always going to be temporary.

KERI, MyCHIPs, and the systems being built around them represent the next step: the same vision, without the compromises.

Not a better blockchain. Something new.

{% comment %}TODO: Add technical comparison of KERI vs specific blockchain identity solutions (ENS, DID:ethr, etc.), deeper MyCHIPs integration patterns with KERI identifiers, and reference to Ryan Fugger's original Ripple vision that MyCHIPs builds upon{% endcomment %}
