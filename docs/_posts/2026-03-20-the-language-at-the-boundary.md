---
title: "The Language at the Boundary"
date: 2026-03-20
categories: [ai, technical]
tags: [programming-languages, abstraction, contract-first, personal-software, composability, ai-agents]
description: "Every abstraction layer in software history served the same purpose: eliminate what doesn't matter. The next one is different."
status: published
header:
  teaser: /assets/images/posts/2026-03-20-the-language-at-the-boundary.jpg
theme_summary: "Programming languages have always been about raising the abstraction level — hiding what doesn't matter so you can focus on what does. The next level isn't about hiding implementation details. It's about operating at the boundaries where software meets the physical world: human intent, hardware state, and legal obligation. At those boundaries, the language isn't code — it's contracts. And composability replaces integration."
---

![A worn wooden ladder leaning against a raw concrete wall, lower rungs sharp and warm in amber side light, upper rungs dissolving into bright overcast light from above, deep indigo shadow pooling at the base](/assets/images/posts/2026-03-20-the-language-at-the-boundary.jpg)

## The Ladder

Every generation of programmers has lived through the same transition: the abstraction level rises, and what used to be the whole job becomes invisible infrastructure.

Machine code gave way to assembly. Assembly gave way to high-level languages. Each rung on the ladder didn't eliminate the layers below — it hid them. The CPU still executes instructions. The registers still hold values. You just don't think about them anymore.

The next rung is forming now.

## What's Different This Time

Previous abstraction jumps were about eliminating boilerplate. You stopped writing memory management routines so you could focus on algorithms. You stopped writing SQL connection logic so you could focus on domain models. The gains were real, but the domain stayed the same: you were still writing programs that ran on computers.

The next level operates at a different kind of boundary altogether.

Not the boundary between your code and the CPU. The boundary between your software and the physical world — specifically, three of them:

**The human-software boundary.** Intent. What a person actually wants, expressed in natural language, translated into action. This is where the current AI wave is crashing.

**The hardware-software boundary.** Physical state. A door that is open or closed. A sensor that fired. A shipment that moved. The real world, pushing events into the system.

**The contract-legal boundary.** Obligation. What was agreed to, by whom, when, and under what conditions. Signed records that carry legal or social weight.

These three boundaries have something in common: they are not about computation. They are about **meaning in the world** — and software has always been terrible at them. Not because programmers weren't clever enough. Because the languages didn't operate there.

## Goodbye to Integration

Here is the thing that has always been quietly embarrassing about enterprise software: integration.

Teams spend months and years just getting systems to talk to each other. Data lives in seventeen places. Every connection is a negotiation. Every migration is a crisis. The industry built an entire category — middleware, ETL, iPaaS — to patch over the gap between systems that were never designed to compose.

Integration work exists because data is trapped. It lives inside the application that created it, accessible only through that application's API, on that application's terms.

The next abstraction level doesn't improve integration. It makes integration unnecessary.

When data is owned by the person it belongs to — when a work history, a health record, an agreement travels with the individual as a signed document rather than sitting in a vendor's database — two systems don't need to negotiate a connection. They each ask the same person for the same record. That's it. No ETL. No sync. No migration.

[No Migration Required](/blog/2026/02/25/no-migration-required/) explored this from the database side. The same logic applies at the language level. A "higher-level language" that operates at the contract-legal boundary doesn't compose systems. It composes agreements. The agreements are the interface.

## Highly Personal Software

The mass-market SaaS model exists because software was expensive to build and operate. You built one thing for a million people because building a million things for a million people was impossible.

That assumption is dissolving.

When a teenager can describe an app to his phone and have it running an hour later — [as we wrote about recently](/blog/2026/03/10/the-app-he-built-on-his-walk-home/) — the economics of "one product for many people" stop making sense. The question stops being "which app do I use?" and starts being "what do I want, and what building blocks already exist to assemble it?"

The building blocks are the layer below the next abstraction. Not the applications — the infrastructure those applications used to lock up: identity, attestations, agreements, permissions. When those primitives are portable and composable, someone can assemble a purpose-built tool from them in an afternoon, use it for a week, and discard it. No vendor relationship required.

This is what "highly personal software" actually means. Not apps customized with your name in the header. Software that was built for exactly your situation, from blocks that belong to you, running on your terms.

## What the Language Looks Like

We don't have a clean name for it yet. "Higher-level language" works as a placeholder — it puts it in the right slot on the ladder.

What it won't look like: a programming language in the traditional sense. You won't write loops and conditionals. You'll express intent, constraints, and relationships. The runtime will figure out how to satisfy them.

What it will share with programming languages: precision. The history of software is littered with "no-code" promises that collapsed into chaos when the problems got real. The reason structured programming mattered — and still matters — is that you need to be able to reason about what a program does. The next abstraction level will still require rigor. Just rigor at a higher level of description.

And like every previous abstraction, there won't be one. There will be many. Each one optimized for a different boundary, a different domain, a different kind of problem. The same way Python and Rust and SQL all exist as high-level languages for different purposes, the boundary languages will specialize. Some will be better at human intent. Some at physical-world events. Some at legal obligation.

The interesting question — the one the industry hasn't fully confronted yet — is who owns the runtime. Who decides what "valid" means when someone expresses intent in one of these languages? Who controls the interpreter?

That question is more important than the language design. But it's a post for another day.

## Where We Are

The abstraction is forming. The primitives are being assembled — in pieces, by different teams, without central coordination, the way these things always happen.

The contract-legal boundary is the one closest to production-ready, because it maps directly onto things that already have legal meaning: signatures, attestations, agreements, audit trails. The building blocks exist. What's missing is the higher-level language that makes them composable by people who aren't cryptographers.

[That's the map with a hole in it.](/blog/2026/02/18/the-map-has-a-keri-shaped-hole/)

The abstraction ladder has one more rung. It operates at the boundary between software and obligation. And whoever builds that rung well will look, in retrospect, like the people who built the first compiler.

{% comment %}TODO: Consider expanding the "who controls the runtime" question into a follow-up post — this is the governance question at the heart of the higher-level language debate.{% endcomment %}
