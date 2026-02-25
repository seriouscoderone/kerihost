---
title: "Build Your Own Town Square"
date: 2026-02-15
categories: [community, identity]
tags: [platforms, community, sovereignty, infrastructure, cost, facebook, slack, self-hosting, keri-host]
description: "KERI infrastructure is cheap enough that anyone can own their community. No Facebook required."
status: draft
header:
  teaser: /assets/images/posts/2026-02-15-build-your-own-town-square.jpg
theme_summary: "We're approaching the moment where standing up your own community infrastructure costs about the same as a Slack subscription. Mothers running PTAs, organizers building movements, neighbors helping neighbors — none of them need Facebook's permission anymore. But getting there means letting go of the assumption that communities need platforms at all."
---

![A worn brass key resting on aged kraft paper with a faint neighborhood map, side-lit on a dark walnut table](/assets/images/posts/2026-02-15-build-your-own-town-square.jpg)

## You Don't Need Facebook's Permission to Organize Your Neighborhood

Here's a question nobody asks because the answer seems obvious: Why does your PTA use Facebook Groups?

Because it's free. Because everyone's already there. Because what else would you use?

Those are fine answers. They're also the exact same answers people gave in 2003 about why they used AOL for email.

The thing about "free" platforms is that [you already know the deal](/blog/2026/02/13/accept-these-terms-or-what/). You get the tool, they get the data, the attention, the right to change the rules whenever they want. Your community exists at their pleasure. One algorithm change and your posts stop reaching your own members. One policy update and your group gets flagged. One acquisition and your whole archive disappears.

But until recently, the alternative — running your own infrastructure — was genuinely hard. You needed servers, sysadmins, domain expertise, ongoing maintenance. The cost wasn't just money, it was complexity.

That's changing.

## What "Build Your Own" Actually Means Now

When we say anyone can build their own community platform with KERI infrastructure, we don't mean everyone needs to become a systems administrator. We mean something more like what Squarespace did for websites: take something that used to require specialized knowledge and make it a commodity.

A KERI-based community setup looks like this:

- **A witness node** (like [KERI.host](https://keri.host)) that validates your community's events. Runs on a Lambda function. Costs pennies.
- **Identifiers for your members** that they own and control. Not accounts on your system. [Actual portable identities](/blog/2026/02/13/you-can-leave-whenever-you-want/) that work everywhere.
- **Credentials you issue** — membership, roles, reputation — as real data structures, not rows in someone else's database.
- **Communication channels** where you [follow people, not algorithms](/blog/2026/02/13/follow-people-not-platforms/).

The total infrastructure cost? Roughly what you'd pay for a Slack workspace. Maybe less. A DynamoDB table with a few thousand members costs a few dollars a month. A Lambda function that handles your community's events costs fractions of a cent per request.

You don't need a data center. You don't need a DevOps team. You need a CDK template and an AWS account. {% comment %}TODO: Or eventually, a one-click deploy that handles even that.{% endcomment %}

## The Paradigm Shift Is the Hard Part

The technology is the easy problem. The hard problem is a mental model that's been drilled into us for twenty years: communities live on platforms.

Think about how deeply embedded this is. Someone starts a book club, first thing they do is create a Facebook Group. A neighborhood wants to organize a cleanup, they make a Slack channel. A mother wants to build a parent network, she starts a WhatsApp group.

Every single one of those communities is a tenant on someone else's property. They have no ownership, no portability, no real control. If WhatsApp decides to change their group size limits, your community just shrank. If Slack's free tier removes message history, your institutional memory just vanished.

The paradigm shift isn't technical. It's conceptual: **your community doesn't need to live inside someone else's product.**

This is the same shift that happened when people realized they didn't need AOL to use the internet. Or when businesses realized they didn't need to rent space in someone else's office building to have a professional presence. You can just... have your own thing.

## What This Looks Like in Practice

Imagine a mother organizing a homeschool cooperative. Today, she'd set up a Facebook Group, a shared Google Drive, maybe a Slack or Discord. Three platforms, three sets of terms of service, three companies mining her community's interactions.

With KERI infrastructure, she stands up her own cooperative:

- **She issues membership credentials.** Not a "join this group" button — actual verifiable credentials that members hold in their own wallets. She defines what membership means. She controls who can issue it.
- **Members own their identities.** If someone leaves the co-op, their identity and reputation history go with them. The co-op can revoke the membership credential, but it can't erase the person.
- **Communication is direct.** Messages go between members, verified by their identifiers. No algorithm deciding who sees what. No ads injected into the feed.
- **The data stays with the community.** Curriculum plans, schedules, member directories — all held at the edge, governed by the co-op's own rules. Not sitting in Google's servers under Google's terms.

And the cost of all this? A few dollars a month in cloud infrastructure. Less than the co-op probably spends on printer ink.

## This Isn't a Fantasy. But It's Not Finished Either.

Let's be honest about where we are.

The core infrastructure exists. KERI witnesses run. Credentials can be issued and verified. Identifiers are portable and cryptographically secure. The [community service marketplace](/blog/2026/02/08/community-service-marketplace/) pattern shows how KERI-native communities can actually function.

What doesn't fully exist yet is the last-mile experience. The one-click deploy. The mobile app that makes credential management feel as simple as signing into Facebook. The templates that let a non-technical organizer stand up a community in fifteen minutes.

{% comment %}TODO: Link to KERI.host deployment guide when available.{% endcomment %}

We're in the "you need to know what a CDK stack is" phase. That's roughly where websites were in 1997 — powerful if you could use them, inaccessible if you couldn't. But the trajectory from "write your own HTML" to "drag and drop your website" was fast, and there's no reason this trajectory will be slower.

The underlying infrastructure is already cheap. The protocols are already open. The missing piece is the interface layer — and that's a solvable problem.

## The Real Game-Changer

The revolutionary part isn't the technology. It's what the technology enables for people who have never had this option before.

A mutual aid network that doesn't depend on NextDoor's good graces. A religious community that owns its own membership records. A parents' group that can verify who's actually a parent in the district without handing that data to a tech company. A [neighborhood that builds real value](/blog/2026/02/06/actual-value-economy/) without an intermediary extracting a cut.

Every community organizer, every PTA president, every block captain, every mother building something for her kids — they all deserve to own what they build. Not rent it. Own it.

The cost of that ownership is dropping to the point where "just use Facebook" stops being the rational choice and starts being the lazy one.

We're not there yet. But we're close enough to see it.

**Related:** [Follow People, Not Platforms](/blog/2026/02/13/follow-people-not-platforms/) | [You Can Leave Whenever You Want](/blog/2026/02/13/you-can-leave-whenever-you-want/) | [Accept These Terms... Or What?](/blog/2026/02/13/accept-these-terms-or-what/) | [Lifting One Another's Burdens](/blog/2026/02/08/community-service-marketplace/)
