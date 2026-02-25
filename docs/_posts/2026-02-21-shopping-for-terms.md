---
title: "Shopping for Terms"
date: 2026-02-21
categories: [economy, identity]
tags: [acdc, contracts, terms-of-service, selective-disclosure, ai-agents, negotiation, competition, consent, marketplace]
description: "What if you could shop for terms the way you shop for price? ACDC schemas and AI agents make contracts a competitive marketplace."
status: draft
theme_summary: "Today, terms and conditions flow one direction: from the service provider to you. Accept or leave. You can't negotiate, you can't counter-offer, and you can't even meaningfully compare. But when contracts are structured ACDC schemas — machine-readable, selectively disclosable, and bidirectional — something new becomes possible. Both parties publish acceptable ranges. AI agents find the overlap. You shop for terms the way you shop for price. That's not just better contracts. That's a new competitive landscape."
header:
  teaser: /assets/images/posts/2026-02-21-shopping-for-terms.jpg
---

![Aged linen paper with two mirrored columns of handwritten text resting on a worn oak workbench, a rubber stamp out-of-focus in the foreground — both parties filling in the same form](/assets/images/posts/2026-02-21-shopping-for-terms.jpg)

## One-Way Paper

We wrote about [the man at the eye doctor](/blog/2026/02/13/accept-these-terms-or-what/) who actually read the fine print and discovered his data was being sold. He refused to sign. Nobody knew what to do.

That post was about visibility — making contracts readable and comparable. But there's a deeper problem we didn't fully address: **the paper only flows one direction.**

The eye doctor writes the terms. You sign them. The employer writes the non-compete. You sign it. The app publishes its privacy policy. You click accept. Spotify decides what happens to your listening data. Your bank decides who sees your transaction history. Your gym decides whether they can sell your workout patterns.

In every case, one party writes and the other party complies. There is no mechanism for you to write back. No way to say "here are *my* terms for this interaction." No counter-offer. No negotiation.

It's not that you chose bad terms. It's that you were never a party to the drafting.

## Why It Stays One-Way

This isn't a conspiracy. It's an infrastructure problem.

There's no standard format for "consumer terms." If you walked into your eye doctor and handed them a sheet of paper that said "here are my conditions for you holding my medical data," they'd have no idea what to do with it. Their software can't process it. Their workflow doesn't account for it. Their lawyer didn't write a response template for it.

The asymmetry isn't malicious. It's structural. Businesses have legal departments and contract templates. You have a signature line.

For negotiation to work, both sides need:
1. A shared format for expressing terms
2. A way to compare their terms against the other party's
3. A mechanism to find overlap
4. A way to formalize the agreement when overlap exists

None of that infrastructure exists today. So terms flow one way. Not because anyone decided it should. Because nobody built the alternative.

## Both Sides of the Contract

ACDC schemas give both sides the same expressive power.

The [previous post](/blog/2026/02/13/accept-these-terms-or-what/) showed what a service provider's terms look like as structured data:

```
Provider Terms ACDC:
  data_sold_to: []
  data_retention: "7 years"
  data_shared_with: ["insurance_provider"]
  patient_controls: ["deletion_request", "export"]
```

Here's the part we didn't explore: **you can write the same structure back.**

```
My Terms ACDC:
  data_sold_to: "none"
  data_retention: "maximum 3 years"
  data_shared_with: ["insurance_provider", "my_primary_care"]
  patient_controls: ["deletion_request", "export", "restrict_processing", "audit_log"]
  required: ["annual_data_audit_report"]
```

Same schema. Same fields. Different values. Now both parties have expressed their position in the same language.

This isn't hypothetical complexity. The ACDC schema defines the *structure* of the agreement — what fields exist, what values are valid, what constraints apply. Both parties fill in their acceptable values. The schema is the shared vocabulary that makes negotiation possible without lawyers in the room.

## The Range

Here's where it gets interesting. You don't need exact matches. You need *overlapping ranges*.

Most terms aren't binary. They exist on a spectrum:

| Term | Your Range | Provider A | Provider B |
|------|-----------|------------|------------|
| Data retention | 1-3 years | 7 years | 2 years |
| Third-party sharing | Insurance only | Insurance + 3 ad networks | Insurance only |
| Deletion rights | On request, within 30 days | Within 90 days | Within 7 days |
| Audit access | Annual report | Not available | Real-time dashboard |
| Price | $150-250 | $150 | $200 |

Provider A fails on retention (7 years vs. your 3-year max) and third-party sharing (ad networks vs. your insurance-only). Provider B fits within every range.

You didn't negotiate with either provider. You published your ranges. They published theirs. The overlap — or lack of overlap — is visible immediately.

This is the shift: from "accept these terms or leave" to "here are the terms that work for me; who's offering them?"

## Your AI Reads Both Sides

Now add [your AI agent](/blog/2026/02/13/buy-your-ai-dont-rent-it/) to the picture.

Your agent knows your preferences — not because it profiled you, but because you told it. You set your ranges once:

*"I don't want my data sold. Retention under five years. I want the right to delete. I want to know who sees my information. I'm willing to pay more for better terms."*

Your agent translates that into structured ranges across relevant ACDC schemas. When you need a service — eye doctor, bank, insurance, gym, streaming, anything — your agent compares your ranges against every provider's published terms.

Not one at a time. All of them. Instantly.

> "Three eye doctors within 10 miles. Two are within your terms. Dr. Park has a 2-year retention policy, no data selling, and offers real-time audit access for $190. Dr. Chen has 3-year retention, no data selling, deletion within 30 days, for $175. Valley Eye Group sells data to four advertising networks — outside your acceptable range."

You're shopping for terms the same way you'd shop for price or location or reviews. Because the terms are finally in the same structured, comparable format as everything else.

## Selective Disclosure in Negotiation

Not every term needs to be visible to find a match.

ACDC selective disclosure means your agent can check whether a provider's terms fall within your ranges *without revealing your exact ranges to the provider*. And the provider can check whether your requirements are within their capabilities without revealing their full terms to you.

This matters. If a provider knows your exact maximum willingness-to-pay for privacy, they'll price to it. If you know the provider's minimum data-sharing requirement, you'll push to the floor. Selective disclosure lets both parties find overlap without exposing their negotiating position.

The disclosure works like this: your agent asks the provider's system a structured query — "Is your data retention period under 5 years?" — and gets a yes or no. Not the actual number. Just whether overlap exists on that dimension. Run that across every term, and you get a compatibility score without either side revealing their full hand.

When both parties confirm overlap across all required terms, *then* the full terms are disclosed and the agreement is formalized as a chained ACDC — signed by both parties, anchored to both identifiers, verifiable by either side at any time.

## The New Competitive Landscape

Today, businesses compete on:
- Price
- Quality
- Convenience
- Brand

They don't compete on terms because terms are invisible. Nobody comparison-shops privacy policies.

Make terms structured, comparable, and machine-readable, and suddenly a new competitive dimension opens up:

**The terms themselves become a marketplace.**

A gym that doesn't sell your workout data to health insurers can differentiate itself — not with a vague "we respect your privacy" badge, but with a verifiable ACDC that proves it. A bank that offers better data rights can attract customers who care about those rights. A streaming service that doesn't track your viewing habits beyond what's needed for recommendations can compete against one that builds advertising profiles.

Some businesses will compete on the most permissive terms (cheapest, but they sell everything). Some will compete on the most restrictive terms (premium, but your data is locked down). Most will land somewhere in between.

The point isn't that everyone picks the same terms. The point is that *terms become a market*. Supply and demand. Differentiation and competition. The same forces that drive prices down and quality up can now drive terms toward what people actually want.

Honest businesses — the ones already doing the right thing — finally get rewarded for it. As we noted in the [previous post](/blog/2026/02/13/accept-these-terms-or-what/): the honest businesses are subsidizing the dishonest ones right now because consumers can't tell the difference. Structured, comparable contracts fix that.

## Service Providers Shop Too

This works in both directions.

A doctor's office might have their own ranges:

*"We need at least 2 years of data retention for continuity of care. We require the ability to share with insurance for billing. We won't accept patients who refuse all data collection — we need basic records to treat you safely."*

Fair enough. Those are reasonable terms from their side. They publish them as structured ACDC ranges just like you do.

A freelance consultant might publish terms like:

*"I need 50% upfront for projects over $5,000. I retain the right to use anonymized work in my portfolio. I require a 30-day payment window."*

A homeowner hiring a tradesperson from the [small trades cooperative](/ecosystems/small-trades-cooperative/) might publish:

*"I need proof of insurance, a licensed journeyman or above, and a warranty on the work. I'll pay on completion with a 5% holdback for 30 days."*

The tradesperson's agent checks: insurance credential? Yes. License level? Journeyman. Warranty terms? Matches. Payment terms? Within range. Both agents confirm compatibility. The contract formalizes automatically.

No general contractor in the middle taking 40-60%. No platform extracting a matching fee. Just two parties whose agents confirmed compatible terms, formalized as a verifiable agreement.

## What Has to Exist

This is a design, not a running system. For it to work:

**Community-defined schemas.** Industries need to agree on what fields matter for their contracts. Healthcare data terms look different from employment terms look different from financial terms. Each ecosystem — using the kind of governance design we build with [C0 ecosystem design](/blog/2026/02/19/designing-keri-ecosystems-with-ai/) — defines its contract schemas.

**AI agents that understand the schemas.** Your agent needs to parse structured ACDC terms, compare ranges, run compatibility checks, and explain the results in plain language. This is well-suited to AI — the schemas are structured data, not ambiguous prose. It's exactly the kind of problem that AI solves well: pattern matching over structured fields with clear rules.

**Provider adoption.** Businesses need to publish their terms as structured ACDCs, not PDFs. This starts where the incentive is clearest: industries where trust differentiation matters most, and where regulation is already pushing toward transparency (healthcare, finance, education).

**Cultural expectation.** People need to expect choices. That takes time. But a generation that grows up with agent-mediated, comparable contract terms will find today's "accept or leave" model as quaint as we find the pre-internet model of calling three stores to compare prices.

None of this exists yet. All of it is buildable with infrastructure that already exists: ACDC schemas for the contract format, selective disclosure for the matching, KERI identifiers for the signing, and AI agents for the intelligence layer. The [pairing](/blog/2026/02/21/the-railroad-moment/) of KERI and AI is what makes bidirectional contracts practical — not just theoretically possible.

## The Level Playing Field

Today, a Fortune 500 company and a single consumer sit on opposite sides of a contract, and one side has a legal department and the other has a checkbox.

Structured ACDC contracts don't eliminate the power asymmetry overnight. But they change the playing field in a fundamental way: both parties express their positions in the same format, with the same tools, using the same schemas.

Your AI agent reads contracts as well as their legal department writes them. Your ranges are as formally specified as their terms. Your identity is as cryptographically solid as their corporate identifier. And if their terms don't work for you, your agent already found three alternatives that do.

That's not utopia. It's infrastructure. The same way that standardized pricing and public markets made it possible to comparison-shop for goods, standardized contract schemas make it possible to comparison-shop for terms.

The eye doctor who sells your data? They can keep doing that. But now they're competing against one who doesn't — and you can see the difference before you walk in the door.

The terms are finally a marketplace. And in a marketplace, you get to shop.

**Related:** [Accept These Terms... Or What?](/blog/2026/02/13/accept-these-terms-or-what/) | [Buy Your AI. Don't Rent It.](/blog/2026/02/13/buy-your-ai-dont-rent-it/) | [The Actual Value Economy](/blog/2026/02/06/actual-value-economy/) | [The Railroad Moment](/blog/2026/02/21/the-railroad-moment/) | [Small Trades Cooperative](/ecosystems/small-trades-cooperative/)
