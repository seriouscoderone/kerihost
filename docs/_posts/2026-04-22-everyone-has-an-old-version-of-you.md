---
title: "Everyone Has an Old Version of You"
date: 2026-04-22
categories: [identity, philosophy]
tags: [contacts, rolodex, data-sovereignty, access-control, privacy, revocation]
description: "Your contacts aren't contacts. They're stale copies of people. The rolodex never died — it just got smaller and multiplied."
status: published
theme_summary: "The entire history of personal contacts is the history of copying. Rolodex cards, address books, iCloud, Google Contacts, LinkedIn, Slack, Signal, WhatsApp — every single system makes its own copy of you. Every copy is immediately out of date. The fix isn't a better copy. It's no copy at all. Grant access to the current you. Revoke it when you want. Update once, everywhere."
header:
  teaser: /assets/images/posts/2026-04-22-everyone-has-an-old-version-of-you.jpg
---

![A vintage Bakelite rolodex on a worn oak desk, its fanned index cards filled with faded handwritten names and numbers, while a single crisp card sits apart on the desk showing a fresh current entry, morning light from a north-facing window casting long soft shadows across the wood grain](/assets/images/posts/2026-04-22-everyone-has-an-old-version-of-you.jpg)

## The Rolodex Never Died

It just got smaller and multiplied.

Your parents had one on a desk. Now you have seventeen of them — iCloud, Google Contacts, your work directory, LinkedIn, WhatsApp, Signal, Slack, Telegram, the CRM at your job, the CRM at every company you've ever done business with, your dentist's scheduling system, your kid's school portal, the gym you quit three years ago that still emails you.

Every one of them has a copy of you.

Every one of them is wrong in some way.

## The Problem No One Solved

Think about this for a second. For a hundred years of modern communication, we've had the same unsolved problem: **personal contact information goes out of date the moment it's written down.**

You move. You change jobs. You get a new number. You kill an email address because it got too much spam. You switch from Twitter to the other thing. You get married and change your name. You get divorced and change it back.

And every rolodex, every address book, every CRM, every phone, every app that ever copied you — is now wrong.

The rolodex had this problem. So we built digital contacts. Digital contacts had this problem. So every app built its own contacts. Every app's contacts have this problem. So we sync them. Syncing has this problem.

We keep making the copy easier to update. We never question whether copying was the right idea in the first place.

## The Absurd Workaround

Here's what we actually do today, in 2026, with all our technology:

You get a new phone number. You send a group text to maybe fifty people. "Hey, new number, save me." Some save it. Some don't. Some forget. Six months later you message an old friend from the new number and they think it's a stranger. A year later someone calls your old number trying to reach you and gets someone else's life.

Multiply that by every change you've ever made. Multiply it by every person in your life who has ever changed anything about themselves. That's the ambient data staleness you live inside.

We accept this because we can't imagine anything else.

## What If Nobody Had a Copy?

Here's the inversion.

What if your contact information lived in one place — with you — and when you "shared your contact" with someone, you weren't handing them a copy? You were granting them **access** to the current you.

Access, not copies.

When you change your number, you change it once. Everyone you've granted access to immediately sees the new one. No group text. No "update my contact." No stale entries in fifty databases.

When you don't want someone to have your information anymore, you revoke the access. That ex-coworker who won't stop texting? Revoke. The company you did business with six years ago who keeps emailing you? Revoke. The app you deleted but whose CRM still has your cell? Revoke.

You don't have to ask them to delete the copy. There is no copy.

## This Is Not Science Fiction

This is just a different data model. One where the authoritative version of a person's contact information lives with that person, signed by them, and other people hold **grants of access** instead of copies of data.

The technology exists. It's what [KERI](/blog/2026/02/11/why-digital-identity/) does. You control a cryptographic identifier. You publish signed statements about yourself — name, phone, email, address, whatever you want to share — and you control who can read them. Revocation is a signed statement. Updates are signed statements. Your "contact card" is an endpoint, not a file.

The people you've granted access to don't store your phone number. They store a pointer that, when they need it, resolves to whatever your current phone number is. If you've revoked them, the pointer returns nothing.

This is what [data staying at the edge](/blog/2026/02/06/data-at-the-edge/) actually means in practice. Not "my data is private" — lots of services say that. It means **the only authoritative copy of me is me.** Everything else is a live read against my permission.

## What Changes

| Old world | New world |
|-----------|-----------|
| Every app stores a copy of your contact | Apps hold a grant of access to you |
| You update your info in 17 places | You update once |
| Ex-friends keep your number forever | You revoke and they have nothing |
| Your info gets sold with the CRM | Your info isn't *in* the CRM |
| "Update your contacts" group texts | Never again |
| LinkedIn knows more about you than you | You know exactly who knows what |

The small things matter most. You move to a new apartment and — nothing. The people who should know, know. The people who shouldn't, don't. You get a new phone number and it just works, everywhere, for everyone who should have it.

No migration. No announcement. No loss.

## The Part Where I'm Honest

This doesn't exist as a consumer product yet. You can't download "KERI Contacts" from the App Store today. The infrastructure exists — the identifiers, the signing, the access control, the revocation model — but someone has to build the experience that makes it feel as simple as adding a contact to your phone.

That's the gap. Not the protocol. Not the cryptography. The product.

And it's a small gap, because the hardest part is already solved. The hard part was never "how do we store a phone number." The hard part was always "how do we let a person control who sees their information and update it once." That's a key management problem, and KERI was built for key management problems.

A "contacts" app that treats your identifier as the source of truth, and every relationship as a revocable grant, is — honestly — one of the most obvious first products that could be built on this. Simpler than credentials. Simpler than community governance. Simpler than delegated AI.

It's just contacts. Done right. For the first time.

## Conclusion

Every rolodex was wrong the moment the ink dried. Every digital contacts list is wrong the moment it syncs. We've spent a century making better copies of something that should never have been copied in the first place.

The fix isn't another app. It isn't a better sync protocol. It isn't AI that keeps your contacts fresh.

The fix is that you are the contact. Everyone else just has permission — for now.

**Related:** [Beyond "Data Companies": Partners, Not Products](/blog/2026/02/06/data-at-the-edge/) — the same principle, applied to the services you use every day.

{% comment %}TODO: Consider a follow-up post on the mechanics — how access grants work, how revocation propagates, what happens when an app is offline, and what a "resolver" endpoint looks like in practice.{% endcomment %}
