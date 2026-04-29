---
title: "The Schema Every Team Has Reinvented"
date: 2026-04-29
categories: [technical, identity]
tags: [acdc, said, schemas, content-addressing, integration, oobi, semantic-web]
description: "Every team reinvents Address. What if the identifier itself came from the schema's content — and convergence happened organically, not by committee?"
status: published
header:
  teaser: /assets/images/posts/2026-04-29-the-schema-every-team-has-reinvented.jpg
theme_summary: "Software has a schema problem. The same concept gets a different shape and a different identifier in every system, and integration is the tax we pay. Content-addressable identifiers — SAIDs in KERI's ACDC system, paired with OOBIs for discovery — change the dynamic. They don't legislate agreement; they make convergence organic. Popular schemas win because they're useful, not because a standards body ratified them. That's the move the Semantic Web missed."
---

![Three parallel dirt tracks worn into a dry late-summer meadow at golden hour, one track noticeably deeper than the others, a small weathered signpost standing off the paths in the middle distance](/assets/images/posts/2026-04-29-the-schema-every-team-has-reinvented.jpg)

## Address, Again

Every developer has shipped Address.

`addr1, addr2, city, state, zip`. Or `street, unit, locality, region, postal_code`. Or a nested object with a country code that one service requires and another service rejects. Whatever shape your team picked the last time you needed it, the *next* team is going to pick a slightly different shape — and the integration tax shows up the moment those two systems try to talk.

We've been doing this for decades. We treat it like an unsolvable people-problem: of course teams disagree, of course schemas drift, of course you're going to write a mapping layer. That's the job.

I'm not sure that's true anymore.

## The Identifier Problem

Here's the thing that's quietly broken: the *name* we use to refer to "Address" is arbitrary. Each system invents its own ID — a table name, a class name, a JSON Schema URL, a database UUID. Two systems modeling the exact same concept have no way to recognize that they're talking about the same thing. There's no shared identifier for *the idea*.

Imagine if there was. Imagine that the identifier for an Address schema was derived from the *content* of the schema itself — so the same schema, defined anywhere by anyone, would always produce the same ID. And imagine anyone could publish their schema, and anyone else could discover and verify it.

That's not science fiction. The pattern has a name: **content-addressable identifiers**. Git uses them for commits. IPFS uses them for files (CIDs). Docker uses them for image digests. The hash of the content *is* the identifier — change the content and you get a new ID, by definition.

What's new is applying this to *meaning*. To the schemas themselves.

## SAIDs and OOBIs

KERI's ACDC system does exactly this. It calls them **SAIDs** — Self-Addressing IDentifiers. A schema's SAID is derived from its content. Two parties who define the same schema produce the same SAID without ever talking to each other. And SAIDs aren't just for schemas — credentials, events, attestations all carry them.

The other half is discovery. **OOBIs** (Out-of-Band Introductions) are how you tell someone where to find a schema given its SAID. No registry. No central directory. Just URLs that resolve to verifiable content. If the content you fetch doesn't hash to the SAID you asked for, you reject it.

Together, SAIDs and OOBIs give you something that's actually unusual in software: a way to refer to a piece of meaning *without depending on a central authority to mint or maintain the reference*.

## "Hasn't This Been Tried?"

This is where any honest version of this argument has to slow down.

Yes. The Semantic Web spent twenty years trying to give universal identity to concepts. RDF, OWL, JSON-LD, schema.org, FHIR in healthcare, ISO standards everywhere. Every one of those efforts aimed at exactly this goal. Most achieved partial adoption at best.

Why didn't they win?

I think it's because they tried to *legislate* meaning. A canonical schema for Address would be ratified by a body, blessed as the standard, and everyone else was supposed to adopt it. That works in narrow domains where the participants have no choice (regulated industries, government data) and falls apart everywhere else, because schema design is path-dependent and committees can't keep up with reality.

SAIDs flip the dynamic. There is no canonical Address. There are *many* Addresses, each with its own SAID, each discoverable, each verifiable. If your schema and mine differ, that's fine — both exist. What changes is that **convergence becomes a market dynamic instead of a committee dynamic**. The popular schema wins because it's useful, not because anyone declared it official. Supporting both during a transition is cheap, because both are addressable and verifiable by content.

Schemas evolve like languages. Not like ISO standards.

## What This Doesn't Fix

Honest limits, because the strong version of this claim is wrong:

Content-addressing doesn't make humans agree. If I model Address with `addr1` and you model it with `street_line_1`, our SAIDs are different even though we mean the same thing. SAIDs make *references verifiable* — they don't make modelers converge on field names by themselves.

What they do is remove a layer of friction. You can point at a schema unambiguously. You can prove that two systems are using the same one. You can publish a mapping between two schemas and have *that* be addressable too. The disagreement is still real, but the substrate underneath the disagreement is finally stable.

## The Robot Version

Step back from enterprise integration for a moment.

Imagine two robots, identical hardware, deployed in two different cities. Both encounter mugs. Both build internal representations of "mug." Today, those representations are private — learned embeddings, model-specific vectors, opaque to anyone but the model that produced them. The robots can't compare notes. They can't agree that they've both seen the same kind of object.

If "mug" had a SAID — if the schema describing what counts as a mug was content-addressable and discoverable — those two robots could refer to the same concept. They could share observations. They could disagree productively. They could be wrong in *commensurable* ways.

I'm not claiming robots are about to do this tomorrow. The dominant approach in robotics is still learned embeddings, and there are good reasons for that. But the symbolic layer above the embeddings — the part where AI agents communicate, share context, hand work to each other — that layer needs identifiers. And we're going to have to pick what kind of identifiers they are.

Centrally minted? Or content-addressable and discoverable?

This is the same question we've been asking about Address. It just gets harder to dismiss when the agents are autonomous.

## Where This Lands

I don't think SAIDs change software overnight. I don't think the Semantic Web's failure modes are fully behind us. Adoption is hard, tooling is early, and most teams will keep writing mapping layers for a long time.

But the schema problem has been treated as fundamental — a permanent tax on integration — for decades. It might not be. It might be that we were missing a primitive. Content-addressable identifiers, discoverable without a registry, verifiable without an authority — that's a real candidate for the missing piece.

I find myself wondering why this isn't a louder conversation. We talked about [the language at the boundary](/blog/2026/03/20/the-language-at-the-boundary/) — about composability replacing integration. SAIDs are one of the things that make composability tractable. Without them, "compose" is still really just "integrate, harder."

Worth paying attention to.

**Related:** [The Language at the Boundary](/blog/2026/03/20/the-language-at-the-boundary/) · [Shopping for Terms](/blog/2026/02/21/shopping-for-terms/)
