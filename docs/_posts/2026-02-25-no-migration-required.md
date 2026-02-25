---
title: "No Migration Required"
date: 2026-02-25
categories: [technical, ai]
tags: [application-design, migration, data-sovereignty, aids, credentials, innovation, ai-agents, developer-experience]
description: "When data is controlled by the AID holder, not the application, you can ship a new app without migrating anything."
status: draft
header:
  teaser: /assets/images/posts/2026-02-25-no-migration-required.jpg
theme_summary: "Software teams spend enormous time and risk on data migrations — not because migration is inherently necessary, but because they built applications that own their users' data. When data is controlled by the AID holder instead, a new application with the right credentials can access the same data immediately. The migration problem is actually the data-ownership problem in disguise. KERI solves the ownership problem, and migration largely dissolves with it."
---

![Work-worn hands transferring a manila folder between two open steel filing cabinet drawers in a concrete workshop](/assets/images/posts/2026-02-25-no-migration-required.jpg)

## Every Developer Knows This Feeling

You have a good idea. You know how to build it. You know it would be better than what you have.

But your current system has three years of user data in a schema that doesn't fit the new design. Before you can ship the better thing, you have to write migrations. Test migrations. Run migrations on production data that you can't afford to lose. Roll back plans for when the migrations go wrong. Freeze features while the migration runs. Pray.

If the migration fails halfway through, you have a split state. Some users on the new schema, some on the old. Your application logic has to handle both — which means the "new" application carries the weight of the old one indefinitely.

This is not a niche problem. It is the default experience of every software team that has ever shipped something real and tried to improve it.

And here's what nobody says out loud: the migration problem is not a technical problem. It is an ownership problem.

## Why Migration Exists

Migration exists because the application owns the data.

In the standard architecture — monolith, microservices, doesn't matter — user data lives in databases controlled by the application team. Your health records are in the clinic's database. Your professional credentials are in LinkedIn's database. Your work history is in Broadspire's claims system. Your identity is in Google's.

When you build a new application, or rebuild the old one, you have to bring the data with you. Because the data is yours to move. It lives in your tables, your schema, your infrastructure. If you want to ship something different, you have to migrate your data into the shape the new thing expects.

This is also why leaving a platform is hard. "Export your data" isn't really portability — it's a dump of whatever the platform decided to give you, in whatever format they chose, that you have to now figure out how to import somewhere else.

The data is theirs. Migration exists because of that fact.

## What Changes When the AID Holds the Data

KERI inverts the ownership model. Data that matters — credentials, authorizations, attestations, contractual relationships — is held by the AID owner, not the application.

Your professional certification isn't a row in a database. It's an ACDC — a signed, verifiable credential issued to your AID. Your employment agreement isn't a record in someone's HR system. It's a bilateral attestation between your AID and your employer's AID, signed by both parties. Your claims history isn't locked in Broadspire's tables. It's a chain of signed actions attributed to identifiers, accessible to anyone with the right authorization.

The application doesn't own this data. It reads from it.

So when you build a new application — one with better AI, a different workflow, a cleaner design, a more efficient architecture — it doesn't need to migrate anything. It presents the right credentials, gets authorized access to the same KERI layer, and reads the same data the old application read.

The old application and the new one can coexist. They can run in parallel while you test. You can switch users over gradually. And when you're ready to shut down the old one, you do — without leaving anyone's data behind.

## The Same Role, The Same Credentials, The Same Data

Here's the concrete version.

Imagine a claims adjuster using Broadspire's current platform. Three years of work history. Hundreds of claims. Verified credentials. All of it is in Broadspire's database, associated with an internal user ID.

Now imagine Broadspire builds a new AI-assisted claims platform. Better UX. Smarter routing. Faster. But in the current architecture, they have to migrate every user's history from the old schema to the new one before anyone can use the new system. That migration is expensive, risky, and blocks the launch.

In a KERI-native model, the picture is different.

The adjuster's identity is their AID. Their professional certifications are ACDCs. Their assignments are contractual events, signed and recorded. Their work history is a chain of signed attestations.

The new platform doesn't need to migrate any of this. It checks: does this AID hold an active adjuster certification? Yes. Does it hold an assignment contract from Broadspire? Yes. Are the historical claims records accessible via that contract? Yes.

The new application inherits full context from day one. Not because anyone migrated it — because the adjuster always owned it.

As [The Last Application](/blog/2026/02/14/the-last-satisfying-application/) puts it: give a system your identity and your contracts, and it has everything it needs. The application logic changes; the identity layer doesn't.

## This Is Especially Important for AI

Software teams are adding AI capabilities at a pace that strains traditional development patterns. An AI assistant gets integrated into an application. The model improves, so it needs to be swapped. An agentic workflow gets added. The agent needs context about who the user is, what they're authorized to do, what they've done before.

In a traditional architecture, all of that context lives in the application's database. Swap the AI layer, and the new model doesn't know anything until you build the data pipeline from the old tables to the new context window. Change the agentic architecture, and you need to migrate the agent's memory and authorization records.

In a KERI-native architecture, the AI agent gets context from the KERI layer directly. The user's AID. Their credentials. Their contracts. Their history of signed actions. That context doesn't live in a database tied to the current application version. It lives with the identity.

Swap the AI model, rebuild the agent, change the underlying framework — the agent wakes up with the same context it would have had yesterday. Not because anyone migrated anything. Because the data follows the person.

This matters more and more as AI makes applications iterate faster. Rapid AI development cycles collide with slow, painful migration cycles. AID-controlled data breaks that collision.

## What Still Needs Migration

Let's be honest about the limits.

Migration doesn't go to zero. It goes down — significantly. But some data is inherently application-specific, and that data still moves the old-fashioned way.

UI state. Cache layers. Application-internal workflow steps. Anything that was never about the user's identity or contractual relationships — that's still the application's data, and it still needs migration if you change the application.

Schema changes to ACDCs themselves also require coordination. If a credential schema evolves — new fields, changed semantics — issuers and verifiers have to agree on the new schema. That's not trivial. It's lighter than a database migration, but it's real work.

And applications that were never built with AID-controlled data in mind — which is most applications today — can't immediately skip migration by declaring KERI support. The shift requires rethinking where data lives, from the beginning.

None of that is hand-waving. It's real. The claim isn't that KERI eliminates migration. The claim is that KERI eliminates migration for the data that matters most: who you are, what you're qualified to do, what you've agreed to, and what you've done.

That's a lot of migration that simply stops being necessary.

## A New Design Posture

The deeper point isn't about migration as a technical problem. It's about what becomes possible when you can innovate without paying the migration tax.

Today, teams hold back. They avoid architectural changes because the migration cost is prohibitive. They ship incremental improvements instead of ground-up rethinks because they can't afford to move the data. They keep systems alive long after they should be retired because the migration to replace them is too risky.

When data follows the person instead of the application, that calculation shifts. You can try a completely different approach, in parallel with the old one, and users can move when they're ready. You can build the AI-native replacement for your legacy system without freezing the legacy system while you migrate. You can iterate aggressively without accumulating migration debt.

The applications get to compete on merit. The better one wins. And the person's data travels with them, unchanged, from the old application to the new one — because it was always theirs.

## Conclusion

The migration problem is familiar to every developer because it's universal. But it's not inherent to software. It's a consequence of a design pattern — applications that own their users' data — that we've accepted as normal because we didn't have an alternative.

KERI offers the alternative. Not as a distant vision, but as a direct consequence of building applications on AID-controlled credentials and attestations instead of application-owned databases.

Build the new thing. Present the right credentials. The data is already there.

**Related:** [The Last Application](/blog/2026/02/14/the-last-satisfying-application/) explores how AID + ACDC + LLM collapses the entire application development model into identity, contracts, and an agent that understands both.

{% comment %}TODO: Add a concrete schema comparison showing what application data looks like "owned by the app" vs. "owned by the AID." Add a diagram illustrating parallel old/new application running against the same KERI credential layer. Expand the AI section with a worked example of agent context continuity across model swaps.{% endcomment %}
