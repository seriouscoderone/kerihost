---
title: "Unbundling the Monopoly"
date: 2026-02-19
categories: [economy, identity]
tags: [capitalism, monopoly, genealogy, marketplace, decomposition, selective-disclosure, records]
description: "Capitalism's real problem isn't profit — it's bundling. KERI ecosystems decompose monopoly platforms into competitive marketplaces."
status: draft
theme_summary: "Companies like Ancestry don't survive on superior technology. They survive on bundling — search, records, matching, DNA, community — into a single platform you can't leave. KERI ecosystems decompose those bundles into components where anyone can compete. Genealogy is the worked example, but the pattern applies everywhere."
---

## The Bundling Problem

Capitalism doesn't need fixing. It needs unbundling.

The criticism people level at capitalism — monopoly power, rent-seeking, data extraction, lock-in — these aren't features of free markets. They're features of *bundled* markets. When one company owns the records, the search index, the matching algorithms, the community connections, and the DNA analysis, they don't have to be the best at any of them. They just have to be the only place where all of them live together.

That's the actual moat. Not technology. Not innovation. Bundling.

KERI ecosystems attack this directly. Not by regulating monopolies or breaking them up through antitrust (though that has its place). By making bundling *unnecessary*. When credentials, records, and reputation are portable — when they travel with you instead of living inside a platform — every component of the bundle becomes a separate competitive marketplace.

Let's make this concrete.

## Ancestry.com: A Case Study in Bundling

Think about what Ancestry actually provides. It's not one service. It's at least six, welded together:

1. **Record holding.** They've digitized and acquired massive collections of historical records — census data, birth certificates, immigration documents, military records.
2. **Search and indexing.** They've built indexes over those records so you can find your great-grandmother's 1920 census entry.
3. **Matching algorithms.** Their "hints" system uses pattern matching to suggest connections between records — "This marriage certificate might be your ancestor."
4. **Family tree construction.** They provide the workspace where you assemble records into a coherent lineage.
5. **DNA analysis.** They genotype your saliva and match you against their database of other customers.
6. **Community connections.** They surface living relatives — real people you didn't know existed — through shared DNA segments and overlapping trees.

Each of these is genuinely valuable. The problem is they're all locked behind one subscription, one platform, one data silo. Your family tree, your DNA matches, your decades of research — all of it lives inside Ancestry's walls. You can export a GEDCOM file, but that's like printing a screenshot of your bank account. The data moves. The *relationships* don't.

And here's the thing that should bother you: Ancestry doesn't need to be the best at any single component. They just need to be the only place where all six coexist. That's not a competitive advantage. That's a hostage situation.

## What Decomposition Looks Like

Now imagine each of those six components as an independent, competitive marketplace.

**Record holders** are anyone — archives, churches, county clerks, individual researchers, families — who hold original or digitized records. They issue verifiable credentials attesting to what they hold. You don't need Ancestry to be the record holder. You need a credential that says "I hold the original 1847 parish register for County Cork, Ireland" and a way for researchers to find you.

**Search and indexing** becomes a service anyone can provide. Index a collection of records, publish your index as a searchable service, compete on coverage and accuracy. Some indexes include the record itself. Others just tell you it exists and who holds it. Both are valuable. Neither requires owning the underlying data.

**Matching and hints** become algorithm services that operate over your portable data. You bring your family tree — stored as credentials you hold — and a matching service runs its algorithms against available indexes and other trees. Multiple matching services compete on the quality of their suggestions. You're not locked into one algorithm's view of your ancestry.

**Family tree construction** stays with you. Your tree is a collection of credentials — each link an attested relationship between two people, with the supporting records chained as evidence. You hold it. Any tool can render it. No platform required.

**DNA analysis** already works this way in practice — you can upload your raw data to GEDmatch, MyHeritage, and others. The missing piece is a verifiable credential for the analysis itself: "This genotype was processed by Lab X using Kit Y on Date Z." Portable, verifiable, not locked to one company.

**Community connections** happen through credential discovery, not platform membership. When your DNA matches someone, or your trees overlap, the connection exists in the credential graph — not inside a company's database. You find each other the way you find anyone in a KERI ecosystem: through OOBIs and credential exchange.

## The Record Holder Problem

Here's where it gets interesting. In the current model, Ancestry is both the record holder and the platform. They acquired records through partnerships, acquisitions, and digitization projects. They hold them. They index them. They charge you to search them.

In a decomposed model, record holding becomes its own credentialed role. Anyone can be a record holder — but you need attestation. A secondary record attestation validates you as a legitimate holder. Did you digitize the original? Did the archive authorize your copy? Is this a derivative work or a primary source? The credential chain answers these questions.

This creates a real marketplace for record access. Archives can monetize their holdings directly. Independent researchers who've spent years collecting records in a specific region can offer access. Families who hold their own documents can make them discoverable. The value isn't locked up in one company's vault.

And record holders can earn from their work. If you spent twenty years photographing every headstone in rural Iowa, that collection has value. A credential attesting to your holdings, combined with a contract for access, turns your labor into a sustainable service. You don't need Ancestry as the middleman.

## Deduplication Through Consensus

One thing Ancestry does handle (imperfectly) is deduplication. The same person appears in multiple records — a census entry here, a marriage certificate there, an immigration manifest somewhere else. Ancestry tries to link these into a single profile.

In a decomposed model, deduplication works differently. Records attach to identifiers. When a person's identifier is established — whether by a living descendant claiming the lineage or by researcher consensus — records naturally cluster around that identifier. The person the record belongs to *is* the join key.

But what about conflicts? Two researchers might independently create identifiers for the same historical person. This is where community consensus comes in. Research attestations — credentials issued by researchers vouching for record linkages — accumulate. When sufficient attestations agree that two identifiers represent the same person, the community can flag one as redundant. The redundant record isn't deleted. It's marked and hidden by default, still accessible for anyone who wants to verify the merge.

This is [subjective reputation](/blog/2026/02/13/follow-people-not-platforms/) applied to historical records. There's no central authority declaring "these are the same person." There's a community of researchers whose attestations create a transparent, auditable consensus. Disagree with the merge? The attestation chain shows exactly who linked what and why.

## The Privacy Window

Here's a detail that matters enormously and that current platforms handle badly: privacy for the recently deceased.

A reasonable rule: for 25 years after death, only selective disclosure applies. The *fact* of death is public — you can see that a record exists and that the person is deceased. But the full record — cause of death, medical details, financial information, personal correspondence — stays locked until the time window expires and the records are verified for release.

This isn't just privacy protection. It's an incentive structure.

Visibility on *why* something is locked creates a trail. You can see that a record exists, that it's sealed, and what conditions must be met for it to open. This trail becomes a breadcrumb for researchers — you know the record is there, you know when it becomes available, and you can plan your research accordingly.

There's even a gamification angle. Researchers who verify records, link attestations, and contribute to the body of knowledge that will eventually unlock sealed records are building verifiable reputation. When those records do open, the researchers who laid the groundwork get attribution. Their credential chain proves they did the work.

## Being an Ancestry Peer

The endgame isn't replacing Ancestry. It's making Ancestry one participant among many in an open marketplace.

KERI.host — or any community running this infrastructure — can be a record holder. We hold records, we index them, we issue credentials attesting to what we hold. We're a peer in the ecosystem, not a platform above it.

Ancestry can keep doing what they do. They can keep their algorithms, their DNA database, their user interface. What they can't do is hold your data hostage. Because in a decomposed marketplace, your research, your tree, your DNA analysis, your record linkages — all of it is yours. Attested by credentials. Portable across any service.

If Ancestry's matching algorithm is the best, great — use it. If someone builds a better one, switch. If a local genealogical society has better records for your region, go there. The [actual value economy](/blog/2026/02/06/actual-value-economy/) applies: services compete on the value they provide, not the data they've locked away.

## The Pattern Applies Everywhere

Genealogy is the worked example, but the pattern is universal.

Every monopoly platform is a bundle of separable services held together by data lock-in. Social media bundles identity, content, discovery, and community. Ride-sharing bundles driver credentials, matching, payment, and reputation. Healthcare bundles records, billing, scheduling, and provider credentials.

KERI ecosystems decompose all of them the same way: make credentials portable, make reputation travel with the individual, make contracts explicit, and let services compete on merit instead of lock-in. The [community service marketplace](/blog/2026/02/08/community-service-marketplace/) does this for volunteer coordination. The [small trades cooperative](/ecosystems/small-trades-cooperative/) does it for skilled labor. The [genealogy ecosystem](/ecosystems/genealogy/) does it for family history research.

Capitalism works fine when markets are actually competitive. The job isn't to fix capitalism. It's to remove the artificial bundling that prevents competition from happening.

## Where We Are

This is a design, not a running system. Nobody has built a KERI-native genealogy ecosystem yet. The credential schemas, the record holder attestation framework, the deduplication consensus model, the privacy window mechanics — these are all architecturally sound but unimplemented.

What exists: the [infrastructure](/ecosystems/) to build it. KERI provides the identity layer. ACDC provides the credential chain. The ecosystem design pattern provides the governance template. The pieces are here.

What's needed: a community of genealogists who are tired of renting access to their own family history and want to own it instead.

**Related:** [The Actual Value Economy](/blog/2026/02/06/actual-value-economy/) | [Lifting One Another's Burdens](/blog/2026/02/08/community-service-marketplace/) | [Accept These Terms... Or What?](/blog/2026/02/13/accept-these-terms-or-what/)
