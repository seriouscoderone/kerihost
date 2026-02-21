---
title: "Accept These Terms... Or What?"
date: 2026-02-13
categories: [identity, economy]
tags: [acdc, contracts, privacy, terms-of-service, selective-disclosure, competition, consent, data-sovereignty]
description: "You can't negotiate with your eye doctor's privacy policy. ACDCs change that by making contracts a primary primitive."
status: draft
theme_summary: "A man at the eye doctor read the fine print and discovered the office sells patient data to Apple, Google, and Facebook. He refused to sign. The office didn't know what to do — and neither did he, because there's no alternative. In today's economy, 'consent' means 'accept or leave.' In a KERI economy, contracts are primary primitives via ACDCs — customizable, transparent, comparable, and competitive. When contracts are real data structures instead of buried legalese, service providers compete on terms, not just price. That gives people something they've never had: options."
---

## The Man Who Read the Fine Print

Someone told me about a visit to the eye doctor. Not remarkable — until they read the paperwork.

Buried in the privacy disclosure, in the fine print that everyone signs and nobody reads, it said the practice sells patient data to Apple, Google, Facebook, and a list of other companies.

The man refused to sign.

The office didn't know what to do.

They'd probably never had someone actually read it. The entire system is designed around the assumption that you won't. The form exists so they can say you "consented." The signature exists so they have legal cover. The fine print exists so they can do whatever they want while technically having told you.

He refused. And then what?

Then nothing. There's nothing to do. He needs an eye doctor. The next eye doctor has the same form — or worse, doesn't even bother telling you. His options are: sign and accept the data selling, or don't get your eyes checked.

That's not consent. That's compliance with a script.

---

## The "Or What?" Problem

This isn't about one eye doctor. This is the entire economy.

Every app. Every service. Every doctor's office, gym membership, streaming platform, airline, and grocery store loyalty program. They all present the same proposition:

**Accept these terms... or don't use our service.**

There is no door number three. There is no "I'd like the version where you don't sell my data." There is no negotiation. The terms are set by the company, presented as a wall of text, and your only role is to click "I Accept."

Why? Three reasons:

**1. Contracts are afterthoughts.** In the current system, the service is the product. The contract is legal boilerplate stapled to the side. Nobody designs the contract. A lawyer writes it once to maximize the company's flexibility and minimize its liability. It's not a negotiated agreement. It's a liability shield.

**2. Contracts are opaque.** Have you ever compared the privacy policies of two eye doctors? Two airlines? Two banks? Of course not. They're 47-page documents written in legal language designed to be technically precise and humanly unreadable. You can't compare what you can't understand.

**3. There's no infrastructure for alternatives.** Even if a doctor wanted to offer you a "we don't sell your data" option, how would they implement it? Their office software doesn't support per-patient data-sharing agreements. Their billing system doesn't have a "privacy-respecting" tier. The entire technology stack assumes one set of terms for everyone.

So the man at the eye doctor has no options. Not because options are impossible. But because the system was never built to offer them.

---

## What If Contracts Were a Primary Primitive?

This is the shift that most people miss about KERI and ACDCs.

When people hear "verifiable credentials," they think about proving your age at a bar or showing your degree to an employer. That's real, but it's the small version of the idea.

The bigger version: **ACDCs make contracts a primary data structure in the economy.**

An ACDC (Authentic Chained Data Container) isn't just a credential. It's a cryptographically bound, schema-defined, selectively disclosable container for any structured agreement between identified parties. Including the terms under which you receive a service.

That changes everything about the eye doctor scenario.

---

## Contracts You Can Actually Read

An ACDC-based service agreement isn't a 47-page PDF. It's a structured data object with a defined schema.

What does the eye doctor's data-sharing agreement look like as an ACDC?

```
Service Agreement ACDC:
  Issuer: [Eye Doctor's AID]
  Holder: [Patient's AID]
  Schema: healthcare-service-agreement/v1

  Terms:
    service_type: "comprehensive eye exam"
    data_collected: ["visual acuity", "refraction", "eye pressure", "retinal scan"]
    data_shared_with: ["insurance_provider"]
    data_sold_to: []
    data_retention: "7 years (regulatory minimum)"
    patient_controls: ["deletion_request", "export", "restrict_processing"]
```

That's not legalese. That's a data structure. A machine can read it. A person can read it. An AI agent running on your phone can read it and tell you what it means before you walk into the office.

`data_sold_to: []` — empty array. Nobody. That's the term. It's unambiguous.

And if the other eye doctor's agreement says:

```
    data_sold_to: ["Apple", "Google", "Meta", "data_broker_network"]
```

Now you can see the difference. Instantly. Before you sit down in the chair.

---

## Competition on Terms

Here's where the economics change.

Right now, eye doctors compete on location, wait times, insurance acceptance, maybe bedside manner. They don't compete on privacy terms because privacy terms are invisible. Nobody compares them. Nobody can compare them.

Make contracts a visible, structured, comparable primitive and suddenly privacy terms become a competitive dimension.

**Doctor A:** Sells your data. Exam costs $150.
**Doctor B:** Doesn't sell your data. Exam costs $175.
**Doctor C:** Doesn't sell your data, lets you control retention, and deletes everything after treatment is complete. Exam costs $200.

That's a real choice. You can see what you're getting. You can decide what your privacy is worth. Some people will take the $150 option. Some will pay $200. Both are legitimate choices.

The point isn't that everyone chooses maximum privacy. The point is that **there's a choice at all.**

Right now, there's no choice because there's no visibility. Make the contract a primary primitive — structured, comparable, machine-readable — and the market can finally function.

---

## Your Agent Reads the Fine Print

Remember the management layer from the "Buy Your AI" model? This is where it earns its keep.

Your AI agent — running locally, on your device, with your KERI identifier — doesn't just manage your keys and credentials. It reads every contract before you do.

You walk into the eye doctor. Their system presents a service agreement ACDC. Your agent reviews it against your preferences:

> "This practice shares data with three advertising networks and retains records indefinitely. Based on your preferences, I'd flag the data sharing as outside your comfort zone. There are two other practices within 5 miles that don't share data with advertisers. Want me to show you the comparison?"

You didn't read the fine print. Your agent did. Not on a remote server that's also profiling you — on your phone, with your rules, answering to you.

The man at the eye doctor shouldn't have had to be the one who reads the paperwork. That's what agents are for. But agents need structured data to work with, and that's what ACDCs provide.

---

## Beyond Healthcare

This pattern applies everywhere the "accept or leave" problem exists:

### Employment Contracts

Your employer presents a non-compete agreement as an ACDC. Your agent compares it against the industry standard schema, flags the unusual clauses, and shows you how it compares to offers from other companies. Non-competes become a competitive dimension in hiring — companies that impose unreasonable restrictions lose talent to companies that don't.

### Financial Services

Your bank's terms of service as a structured ACDC. What do they do with your transaction data? Who do they share it with? What are the actual fee structures, not buried in footnotes but in a machine-readable schema that your agent can compare across every bank in your area?

### Software and Apps

App permissions become contract terms, not checkboxes you tap through. `camera_access: "during_active_use_only"` vs `camera_access: "background_permitted"` — visible, comparable, competitive.

### Education

The CTO Challenge problem from another post, but applied to the student side. What are the terms under which your university holds your learning records? Who do they share them with? Can you get them back when you leave? Structured ACDCs make these terms visible and comparable across institutions.

---

## The Issuer Also Benefits

This isn't just good for patients and consumers. It's good for the honest providers.

The eye doctor who doesn't sell your data? Right now, they have no way to differentiate themselves on that basis. Their privacy practices are invisible, just like everyone else's. They're eating the cost of being ethical with no market reward.

Make contracts visible and comparable, and suddenly ethical practices become a competitive advantage. The doctor who respects your data can prove it — cryptographically, not just with a "we value your privacy" banner on their website.

**The honest businesses are subsidizing the dishonest ones right now** — because consumers can't tell the difference. Structured, verifiable contracts fix that asymmetry.

---

## What Needs to Be True

For this to work, several things need to come together:

**Standard schemas.** Healthcare service agreements, employment contracts, financial terms — these need community-defined schemas that make comparison meaningful. This is where ACDC schemas and ecosystem governance matter. Not one schema to rule them all, but enough standardization that comparison is possible.

**Agent infrastructure.** People need AI agents that can read, compare, and advise on contract terms. These need to run locally (not cloud services that become new intermediaries) and need to understand the relevant schemas.

**Provider adoption.** Businesses need to present their terms as structured ACDCs, not just PDFs. This is a chicken-and-egg problem, but it starts with industries where trust matters most — healthcare, finance, education — and where regulatory pressure is already pushing toward transparency.

**Cultural shift.** People need to expect choices. Right now, we've been trained to click "Accept" without reading. A generation that grows up with agent-reviewed, comparable contract terms will find the current model as absurd as we find the pre-internet model of signing whatever the car dealer puts in front of you.

None of this is instant. All of it is buildable.

---

## The Real Consent

The man at the eye doctor did something radical: he read the terms and said no.

The system broke. Nobody knew what to do. Because the system was never designed for someone to actually exercise judgment about the terms of their own interactions.

KERI and ACDCs don't just give that man better tools. They rebuild the system around the assumption that people will exercise judgment — or have agents that exercise it for them.

Contracts as primary primitives. Structured, readable, comparable, competitive. Not legal boilerplate designed to be ignored. Not "consent" that means "compliance." Actual agreements between identified parties with visible terms.

Accept these terms... or choose different ones. From a provider who competes for your trust, not just your copay.

That's the "or what" that doesn't exist today. ACDCs build it.

**Related:** [Shopping for Terms](/blog/2026/02/21/shopping-for-terms/) | [Buy Your AI. Don't Rent It.](/blog/2026/02/13/buy-your-ai-dont-rent-it/) | [The Actual Value Economy](/blog/2026/02/06/actual-value-economy/)

{% comment %}TODO: Add specific ACDC schema examples for healthcare service agreements, comparison of current privacy regulation approaches (HIPAA, GDPR consent mechanisms) vs ACDC-based contract transparency, economic analysis of how contract visibility changes market dynamics, technical walkthrough of agent-mediated contract review flow, and real examples of industries where terms competition already exists in nascent form{% endcomment %}
