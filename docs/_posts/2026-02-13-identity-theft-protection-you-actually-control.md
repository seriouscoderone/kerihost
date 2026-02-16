---
title: "Identity Theft Protection You Actually Control"
date: 2026-02-13
categories: [identity, technical]
tags: [key-rotation, pre-rotation, multi-sig, recovery, witnesses, delegation, identity-theft, security]
description: "KERI gives you layers of identity theft protection — from a written-down key to a trusted circle of people who can help you recover."
status: draft
theme_summary: "Current identity theft protection is a service you subscribe to that monitors databases you don't control for breaches you can't prevent. KERI flips this entirely: you choose your level of protection, from a private key written on paper in a safe, to a circle of trusted people who can collectively help you recover your identity. Multiple layers, all under your control, all based on math rather than monitoring."
---

## The Identity Theft Protection Racket

You've seen the ads. Pay $19.99 a month and we'll watch the databases where your identity already leaked and tell you when someone uses it.

Think about that for a second.

The "protection" is: we'll tell you after it happens. After someone opens a credit card in your name. After someone files a tax return with your Social Security Number. After the damage is done.

This isn't protection. It's notification. You're paying for a subscription to bad news.

And the reason this is the best we can do is that your identity — as it exists today — is a collection of static facts (name, birthdate, SSN, address) stored in databases you don't control. Once those facts leak, they can be reused forever. There's no way to "rotate" your Social Security Number. There's no way to invalidate a stolen birthdate.

Your identity is a skeleton key that can't be changed after it's copied.

---

## What If Your Identity Could Rotate?

This is the fundamental shift. KERI identifiers aren't static facts. They're cryptographic key pairs — and the keys can be changed.

If someone compromises your current key, you rotate to a new one. The old key becomes worthless. The attacker's copy is dead. Your identity continues, stronger than before, because the rotation itself is proof that you're the legitimate controller.

But rotation is just the beginning. KERI gives you multiple layers of protection, and you choose how deep you want to go.

---

## Level 1: A Key on Paper

The simplest form of KERI identity protection is almost absurdly low-tech.

When your AID is created, the protocol generates a pre-rotation commitment — a hash of the next public key that will take over if you rotate. That next private key can be generated, written down, and stored somewhere physical.

A piece of paper in a safe deposit box. A metal plate in a fireproof safe. A handwritten note in a sealed envelope with your attorney.

That's it. That written-down key is your recovery mechanism. If your phone is stolen, your laptop is compromised, or your cloud account is hacked — none of it matters. You take the key from the safe, execute a rotation event, and the attacker's access is permanently revoked.

Why this works:

- **The key was never online.** It was generated and immediately stored physically. No digital exposure. No server breach can touch it.
- **The commitment is already in your Key Event Log.** When you created your AID, you committed to this next key's hash. The rotation is pre-authorized from inception.
- **The attacker can't rotate first.** They'd need the pre-rotated private key — the one sitting in your safe — to authorize a rotation event. Having your current signing keys isn't enough.

The current system asks you to memorize a Social Security Number and hope no database leaks it. KERI lets you write down a key, put it in a safe, and know that no digital attack can permanently compromise your identity.

---

## Level 2: Separate Keys for Separate Purposes

KERI maintains a strict separation between two types of keys:

**Signing keys** — Used for everyday operations. Signing messages, authorizing transactions, presenting credentials. These are the keys your phone or laptop uses day-to-day.

**Rotation keys** — Used exclusively to authorize key changes. These only appear when you need to rotate. They never sign everyday operations.

Why this matters: if an attacker compromises your signing keys, they can impersonate you for everyday interactions — but they **cannot change your keys.** They can't lock you out. They can't take over your identity. They can create false interactions, but the moment you notice and rotate, their access ends and yours continues.

It's like someone stealing your house key versus stealing the deed to your house. The stolen house key is a problem. But you can change the locks, and the house is still yours.

In every other identity system, compromising the password is game over. In KERI, compromising signing keys is a recoverable event. The real authority — the rotation keys — was never exposed.

---

## Level 3: Multi-Signature Thresholds

Here's where it gets powerful.

Your AID doesn't have to depend on a single key. KERI supports multi-signature thresholds with fractional weights. In plain language: you can require that multiple keys agree before anything important happens.

### Simple Threshold: 2-of-3

You create three keys. Store them in three different places — your phone, your laptop, and a hardware security device. Any two of the three must sign to authorize a rotation.

An attacker would need to compromise two of your three devices simultaneously. If they get one, it's not enough. You rotate using the other two.

### Weighted Threshold

KERI goes further than simple m-of-n. Each key can carry a fractional weight:

```
Key A (phone):    weight 1/2
Key B (laptop):   weight 1/2
Key C (hardware): weight 1/2
```

A rotation requires weights summing to 1. So any two of the three keys are sufficient. But you could also structure it differently:

```
Key A (phone):    weight 1/3
Key B (laptop):   weight 1/3
Key C (hardware): weight 1/2
```

Now the hardware key plus either other key is sufficient, but the phone and laptop alone aren't enough. You've given more authority to the key stored on the most secure device.

### Multi-Clause Thresholds

For even more control, KERI supports multiple independent clauses:

```
Clause 1: [1/2, 1/2, 1/2]  — any 2 of 3 personal keys
Clause 2: [1, 1]            — any 1 of 2 recovery keys
```

Either clause being satisfied authorizes the rotation. This means your personal keys work for routine rotations, but if all three personal keys are somehow compromised, you still have an independent recovery path.

This is identity protection that you design. Not a one-size-fits-all service. A security architecture tailored to your life, your risk tolerance, and your trust relationships.

---

## Level 4: Your Trusted Circle

This is the most human layer — and the most powerful.

You choose a group of people you trust. Your spouse. Your sister. Your best friend. Your attorney. Your financial advisor. You give each of them a key shard. You set a threshold: 3 of 5 must come together to recover your identity.

No single person can do it alone. Your sister can't unilaterally recover your identity without your spouse and your attorney also agreeing. But if you're incapacitated, if you lose everything, if disaster strikes — your circle can collectively reconstitute your identity for you.

### How This Works Technically

Your AID's next rotation key set is structured as a multi-sig group with a weighted threshold across your trusted circle. Each person holds one key. The threshold requires enough of them to come together to authorize a recovery rotation.

```
Recovery group (next rotation keys):
  Spouse:     weight 1/3
  Sister:     weight 1/3
  Best friend: weight 1/3
  Attorney:   weight 1/3
  Advisor:    weight 1/3

  Threshold: sum >= 1 (any 3 of 5)
```

When your trusted circle executes the recovery, they're performing a cryptographically valid rotation event. The result is a new key state under your control (or under a new set of keys you designate). Your identity survives. Your credentials, reputation, and history continue uninterrupted.

### Why This Is Better Than Anything That Exists

**Better than a password manager:** A password manager is a single point of failure. Compromised master password = everything gone. A trusted circle requires multiple independent people to collude — people who know each other and have reasons not to.

**Better than "security questions":** Your mother's maiden name is in seventeen breached databases. A cryptographic key shard held by your actual mother is not.

**Better than account recovery:** Platform account recovery asks you to prove who you are to the platform's satisfaction, using the platform's criteria, on the platform's timeline. KERI recovery is executed by people who actually know you, using keys you actually distributed, on a timeline you control.

**Better than identity theft monitoring:** Monitoring tells you after the damage. A trusted circle prevents the damage — or recovers from it — without waiting for a company to notice.

---

## Level 5: Witnesses as Independent Verification

Every level so far is about your keys. Witnesses add an independent layer.

Your AID designates witnesses — independent infrastructure that attests to the order and integrity of your key events. When you rotate keys, your witnesses must also sign receipts confirming the rotation.

An attacker who somehow obtained your rotation keys would still need to get your witnesses to sign off. The witness threshold — say, 2 of 3 witnesses — means an attacker needs to compromise both your keys and a majority of your witnesses simultaneously.

Witnesses don't have your keys. They don't control your identity. They simply attest: "Yes, I saw this rotation event, and it was properly authorized." They're notaries, not custodians.

You can choose your own witnesses. You can change them. You can set the threshold higher for more security or lower for more convenience. Another parameter in your security architecture that you design.

---

## Level 6: Delegation Hierarchies

For organizations or families who want even deeper protection, KERI supports delegated identifiers — AIDs that derive their authority from a parent AID.

Your personal AID delegates to a daily-use AID. Your daily-use AID is what your phone uses for routine interactions. If your phone is compromised and the daily-use AID is stolen, your personal AID — the delegator — can revoke it and issue a new one.

The attacker got a branch. You own the trunk. Cut the branch, grow a new one.

This creates hierarchical protection:

```
Personal AID (cold storage, rarely used)
  └── Daily AID (phone, routine use)
  └── Work AID (laptop, professional contexts)
  └── AI Agent AID (delegated, scoped, revocable)
```

Compromise at any leaf level doesn't touch the root. The root can always recover.

---

## Choose Your Level

| Level | Mechanism | Protects Against | Effort |
|-------|-----------|-----------------|--------|
| **1** | Key on paper in a safe | Device theft, cloud compromise | Minimal |
| **2** | Separate signing/rotation keys | Signing key compromise | Low |
| **3** | Multi-sig weighted thresholds | Multi-device compromise | Moderate |
| **4** | Trusted circle (m-of-n people) | Total loss, incapacitation | Moderate |
| **5** | Witness thresholds | Coordinated attack on keys | Moderate |
| **6** | Delegation hierarchies | Leaf-level compromise | Higher (organizations) |

You don't have to use all six. A piece of paper in a safe (Level 1) is already better protection than anything the current identity system offers. Adding a trusted circle (Level 4) makes your identity recoverable from essentially any scenario short of everyone you trust conspiring against you.

The point isn't that everyone needs maximum security. The point is that **you choose.** You design your protection architecture based on your needs, your relationships, and your risk tolerance. Not a corporation's checkbox. Not a service's subscription tier. Your security. Your design.

---

## The Comparison

| | Current Identity | KERI Identity |
|-|-----------------|---------------|
| **If credentials stolen** | Used forever; can't be invalidated | Rotate keys; stolen credentials become useless |
| **If password compromised** | Attacker has full access | Attacker has signing keys only; can't rotate |
| **If everything is lost** | Start over; rebuild from scratch | Trusted circle recovers your identity |
| **Protection model** | Monitor databases for breaches | Prevent compromise through architecture |
| **Who controls protection** | A service you subscribe to | You |
| **What it costs** | $19.99/month forever | Time to set up; then it's math |

---

## The Real Protection

Identity theft protection isn't a service. It's an architecture.

The current model is surveillance-based: watch for breaches, alert after damage, help with cleanup. It exists because the underlying identity system — static facts in centralized databases — is fundamentally indefensible.

KERI's model is prevention-based: keys rotate, compromised keys become worthless, multiple layers of protection stack, and trusted people can recover what technology can't.

A piece of paper in a safe. A circle of people who care about you. Math that doesn't depend on any company staying in business.

That's identity theft protection you actually control.

{% comment %}TODO: Add step-by-step setup guides for each level, specific hardware recommendations for key storage, walkthrough of a recovery scenario using a trusted circle, comparison with existing recovery mechanisms (Apple Account Recovery Contacts, Google Inactive Account Manager), and technical diagrams of multi-sig threshold structures{% endcomment %}
