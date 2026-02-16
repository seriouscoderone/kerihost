---
title: "You Can Leave Whenever You Want"
date: 2026-02-13
categories: [identity, technical]
tags: [key-rotation, pre-rotation, apple, google, transition, sovereignty, migration, aids]
description: "KERI's key rotation means you can start with Apple or Google and take your identity back whenever you're ready."
status: draft
theme_summary: "The biggest objection to self-sovereign identity is the transition: how do we get from here to there? KERI's answer is elegant — anyone can create an AID for you, including Apple and Google. And because of pre-rotation, you can take control of that AID from them whenever you want, without their permission. This isn't a revolution. It's an exit door that's always unlocked."
---

## The Transition Problem

Every new technology faces the same question: how do we get from here to there?

Self-sovereign identity has a particularly sharp version of this problem. You can't ask seven billion people to stop using Apple ID and Google accounts on a Tuesday and start managing their own cryptographic keys on a Wednesday. That's not a transition plan. That's a fantasy.

The platforms are entrenched. People depend on them. Entire ecosystems — apps, services, payments, communications — are built on platform-issued identity. Ripping that out isn't realistic. Asking people to abandon it isn't fair.

So how do you transition?

You don't. You build an exit door and leave it unlocked.

---

## Anyone Can Create an AID

Here's the first thing to understand about KERI: **anyone can create an Autonomic Identifier (AID).**

There's no central authority that issues AIDs. No registry you have to apply to. No company that has to approve you. The math doesn't care who runs it.

Apple can create an AID for you. Google can create an AID for you. Your bank can. Your employer can. Your phone can. You can do it yourself on a laptop with no internet connection.

The identifier is just a cryptographic key pair with a self-certifying prefix. Whoever generates the keys generates the AID. It's as natural as creating an email address — except no one owns the server.

This means the transition can start exactly where people already are.

---

## Start Where People Are

Imagine Apple decides to support KERI AIDs. (They don't yet. This is hypothetical. But the architecture allows it.)

**Day one:** Apple generates a KERI AID for you, just like they generate an Apple ID today. It lives in your iCloud Keychain. You don't think about it. You don't manage cryptographic keys. You just use your phone the way you already do.

From the outside, nothing changes. Apple manages your identity. Your apps work. Your credentials are stored in Apple's ecosystem. You're a happy customer.

But underneath, something is fundamentally different. Your identity isn't an Apple ID — a proprietary identifier that exists only within Apple's system. It's a KERI AID — a self-certifying identifier with a Key Event Log, pre-rotated keys, and a cryptographic structure that doesn't depend on Apple to function.

**You don't notice the difference. That's the point.**

---

## The Exit Door

Here's where KERI does something no other identity system can do.

Your AID has a **pre-rotation commitment**. When Apple created your AID, the protocol required committing to a next key — the key that will take over if the current key is rotated. That next key can be held by you, stored separately, waiting.

At any point — next week, next year, a decade from now — you can **rotate your keys.** You take the pre-committed next key, execute a rotation event, and now you control the AID. Not Apple. You.

And here's the critical part: **Apple can't stop you.**

They don't need to approve the rotation. They don't need to cooperate. They don't even need to know it's happening. The rotation is a cryptographic event recorded in your Key Event Log. It's self-certifying. It's valid because the math says it's valid, not because Apple says it's valid.

One moment, Apple manages your identity. The next moment, you do. The transition is a single key rotation event. No migration. No export. No "request your data." No permission.

---

## Why This Changes Everything About Adoption

Every previous attempt at self-sovereign identity had the same fatal flaw: it required people to abandon what they're using before they could start using the new thing. That's a massive barrier. People don't switch unless the pain of staying exceeds the pain of leaving.

KERI inverts this entirely.

**Phase 1: Platforms create AIDs for their users.** The user experience doesn't change. Apple manages your AID like they manage your Apple ID. Google manages your AID like they manage your Google account. People don't need to understand cryptography. They don't need to manage keys. They just use their phones.

**Phase 2: The ecosystem grows.** Credentials, services, and communities start accepting KERI AIDs. Not instead of platform logins — alongside them. Your AID accumulates credentials, reputation, and history while you go about your life normally.

**Phase 3: People who want control take it.** Some people will never rotate their keys. They're happy with Apple managing things. Fine. That's their choice. But the people who want sovereignty — who want to own their identity, carry their credentials, and not depend on any platform — can take control at any time. No permission needed. No migration hassle. Just a rotation.

**Phase 4: The platforms become optional.** As more people hold their own keys and more services accept self-presented AIDs, the platforms shift from identity owners to service providers. They can still offer key management as a service. They just can't hold your identity hostage.

This isn't a revolution. It's an evolution with an exit door.

---

## What Pre-Rotation Actually Means

For the technically curious, here's what makes this work:

When an AID is created, the inception event includes a **commitment to the next key** — specifically, a hash of the next public key. This is the pre-rotation commitment.

The current key can sign events, authorize actions, and manage the AID. But the current key **cannot prevent rotation** to the pre-committed next key. The next key is already baked into the event log from the beginning.

When you rotate:
1. You reveal the next public key (which matches the hash committed at inception)
2. You sign the rotation event with the next key
3. The rotation event is witnessed and recorded
4. The old key is no longer authoritative
5. A new pre-rotation commitment is made for the key after that

The old key holder — Apple, Google, whoever — loses authority over the AID. Permanently. Cryptographically. Without their participation.

This is not a theoretical capability. It's how KERI works at the protocol level. Every AID created with pre-rotation has this exit built in from the moment of inception.

---

## The Apple and Google Incentive

"Why would Apple or Google support this? They'd be giving up control."

Two reasons:

**1. They're already being forced.** Regulators worldwide are pushing for data portability, identity interoperability, and user control. The EU's eIDAS 2.0, Utah's SEDI legislation, and similar efforts are making platform-locked identity a liability, not an asset. Supporting KERI-compatible AIDs positions platforms as cooperative rather than adversarial.

**2. They still make money.** Apple and Google don't actually need to own your identity to profit from their services. They need you to use their devices, their app stores, their cloud services. Managing your AID is a value-add service, not a control mechanism. If you rotate away, they lose a key management customer — not a captive user. Their hardware, their ecosystem, their services still stand on their own merit.

The platforms that embrace this transition will be the ones that survive it. The ones that fight it will discover that users with exit doors eventually use them.

---

## What Needs to Happen

This transition path is architecturally sound. The protocol supports it. The math works. But making it real requires:

**Platform adoption.** Apple, Google, or any major platform needs to support KERI AID creation. This could start with a single use case — a developer tool, an enterprise feature, a pilot program.

**Wallet infrastructure.** People need somewhere to hold their pre-rotation keys that isn't the platform. Hardware security modules in phones help. Dedicated KERI wallets help. Even a written-down key in a safe deposit box helps.

**Community education.** People need to understand — at a high level, not a technical level — that their identity is portable. That they have an exit. That the door is unlocked.

**Ecosystem growth.** Credentials, services, and communities need to accept KERI AIDs. Every service that accepts an AID makes the exit door more valuable.

None of this requires anyone to stop using what they're using today. That's the whole point.

---

## The Unlocked Door

The transition to self-sovereign identity doesn't require a revolution. It doesn't require people to abandon platforms. It doesn't require a flag day where everyone switches at once.

It requires one thing: an identity architecture where the exit door is always unlocked.

KERI's pre-rotation means that anyone who creates an AID for you — Apple, Google, your employer, your government — cannot prevent you from taking it back. The commitment to your next key is in the protocol from inception. The rotation is yours to execute, on your schedule, for your reasons.

You can stay with Apple forever if you want. That's fine. But the moment you want to leave — the moment you want to carry your own identity, manage your own keys, and stop depending on a platform for something as fundamental as who you are — you can.

No permission. No migration. No asking nicely.

You just rotate.

{% comment %}TODO: Add technical diagram of pre-rotation flow, concrete example of AID creation by platform followed by user-initiated rotation, comparison with current platform account portability (Google Takeout, Apple data export) to show how KERI rotation is fundamentally different, and timeline estimate for realistic platform adoption scenarios{% endcomment %}
