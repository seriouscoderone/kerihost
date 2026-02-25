---
title: "The Last Application"
date: 2026-02-14
categories: [ai, technical]
tags: [keri, acdc, llm, agents, contracts, insurance, claims, platforms, ui]
description: "What if every application was just an AI that knew who you were and what you'd agreed to do?"
status: draft
theme_summary: "We keep building platforms. Login screens. Dashboards. Role-based access. Workflow engines. But there's a simpler formulation hiding underneath all of it: an identity, a set of contracts, and an AI that understands both. AID + ACDC + LLM. That's it. That's every application. The implications are unsettling if you build software for a living — and liberating if you use it."
header:
  teaser: /assets/images/posts/2026-02-14-the-last-satisfying-application.jpg
---

![A folded typewritten document with a deep indigo wax seal on a worn oak desk, natural side light casting long shadows across the paper surface](/assets/images/posts/2026-02-14-the-last-satisfying-application.jpg)

## What Is an Application, Really?

Strip away the CSS. The login screen. The navigation drawer. The role-based access control. The status badges and progress bars and notification bells.

What's actually happening?

A person with a specific identity is fulfilling obligations defined by agreements they've entered into. They take actions. The actions are recorded. Other parties are notified. Conditions are checked. Payments are triggered.

That's it. That's every enterprise application ever built. The rest is decoration.

## The Formula

Here's what came into focus during a recent conversation about insurance claims processing — but it applies to everything:

**AID + ACDC + LLM = any application you need.**

- **AID** — A KERI Autonomic Identifier. Who you are, cryptographically provable.
- **ACDC** — Authentic Chained Data Containers. Your contracts, credentials, and obligations. What you've agreed to do, what you're authorized to do, who you've agreed to do it with.
- **LLM** — A language model that understands both. Your agent. Your interface.

Give an LLM your identity and your contracts, and it has everything it needs to help you fulfill your obligations. It knows what you're supposed to do. It knows what credentials you need to do it. It can present you with exactly the interface you need in the moment, generate the right forms, accept the right inputs, route the right outputs.

No platform required. No centralized workflow engine. No vendor lock-in to someone else's idea of how your job should work.

## The Claims Adjuster Who Doesn't Need a Platform

Here's the concrete version.

An insurance claims adjuster today works inside some company's system. Broadspire. Sedgwick. Crawford. Whatever. Each has its own platform, its own login, its own UI, its own workflow. If the adjuster works for multiple companies — which many do — they juggle multiple systems, multiple credentials, multiple ways of doing fundamentally the same thing.

Now reimagine it.

The adjuster has a KERI identifier. They hold credentials — licensed estimator, certified adjuster, whatever their qualifications are. They have contracts — agreements with Company A to handle claims on certain days, agreements with Company B for certain lines of business.

All of this is expressed as ACDCs. Verifiable. Cryptographically signed by both parties. Machine-readable.

Their AI agent — their "mirror, mirror on the wall" — knows all of it.

So when a claim gets assigned (which is itself a contractual event — the claims assigner fulfilling *their* contractual role), the adjuster's agent says: "You've been assigned a property damage claim. Based on your contract with Company A, you need to: inspect the property, take photos, write an estimate, and submit a report. Your estimator credentials authorize you to sign the estimate. Here's what I need from you."

The adjuster takes photos. The agent submits them — signed with the adjuster's AID, fulfilling a specific clause of a specific contract. The adjuster writes an estimate. Signs it with credentials that prove they're licensed to estimate. The agent routes it to the right place based on the contract terms.

No platform told the adjuster what to do. The contract did. No platform verified their credentials. Cryptography did. No platform routed the paperwork. The contract terms defined where it goes.

The adjuster doesn't care what system Company A uses internally. They don't care what Company B's UI looks like. They have their agent, their identity, their contracts. That's the application.

## What Is a Contract but a Requirements Document?

This is the insight that changes everything.

Every piece of software ever built started with requirements. What should the system do? Who are the users? What are their roles? What actions can they take? What happens when they take them?

Those requirements are, at their core, contractual definitions. Party A agrees to do X. Party B agrees to do Y. When condition Z is met, action W is triggered.

We've been translating contracts into code. Building bespoke platforms to enforce agreements that could be expressed directly as verifiable data.

If the contracts themselves were machine-readable — which is exactly what ACDCs are — then the translation step disappears. The contract IS the application logic. The LLM just renders it into whatever experience the human needs in the moment.

"Build me an app that conforms to these ACDCs" isn't a fantasy. It's a direct consequence of having machine-readable contracts and AI that can interpret them.

## The Estimate That Pays Itself

Follow the insurance example one more step.

The adjuster writes an estimate. It's signed with their credentials. It gets routed to a reviewer — another role defined by contract. The reviewer approves it — another signed contractual action.

Now what? In today's world, someone has to process a payment. Cut a check. Initiate a transfer. More platform, more workflow, more humans doing what the contract already says should happen.

But the policy itself is a contract. An ACDC between the insured and the insurance company. It says: when an approved estimate exists for a covered loss, the insured gets paid.

The condition is met. The estimate exists. It's approved. Both actions are cryptographically signed by parties with the right credentials. The contract terms are unambiguous.

So the payment just happens. Not because some workflow engine was programmed to trigger it. Because the contract says so, the conditions are verifiably met, and the system can see that.

Nobody has to "do" the payment. The contract executes itself — not in the smart-contract-on-a-blockchain sense, but in the sense that all the evidence for fulfillment is verifiable, so the logical next step is automatic.

## Every Role Is a Contract

This pattern isn't unique to claims adjusters. Walk through any organization:

**The claims assigner** has a contract that says: when new claims arrive, match them to available adjusters based on qualifications, geography, and capacity. That's an ACDC describing a role.

**The underwriter** has a contract that says: evaluate risk applications against these criteria, approve or deny, sign your decision with your credentials. Another ACDC.

**The HR department** — or rather, the function that HR performs — is really just a collection of contracts. Employment agreements. Availability commitments. Compensation terms. "I'm contractually signing up to be an adjuster for your company on these days in these hours." That's a verifiable, machine-readable agreement. No HR portal required.

**The manager** has a contract that says: ensure adjusters in your region are meeting their obligations, review escalations, approve exceptions. Another set of obligations, another set of credentials.

Each of these roles, in every organization, can be decomposed into: who you are, what you're qualified to do, what you've agreed to do, and evidence that you did it. Identity. Credentials. Contracts. Attestations.

AID. ACDC. ACDC. ACDC.

The LLM just makes it navigable.

## The End of Platforms

Let's be precise about what "no platform" means. It doesn't mean no software. It doesn't mean no interfaces. It doesn't mean no apps.

It means no *centralized platform* that owns the workflow.

Today, Broadspire owns the claims workflow. If you're an adjuster working Broadspire claims, you use Broadspire's system. Your work history lives in Broadspire's database. Your credentials are Broadspire's to verify. If Broadspire changes their platform, you adapt. If you stop working with Broadspire, your history stays behind.

In the contract-native model, you own your identity. You hold your credentials. Your contracts are bilateral agreements between you and whoever you work with. Your work history is a chain of signed attestations that travel with you. Your AI agent is yours — it works for you, not for the platform.

The "platform" becomes a commodity. A thin layer that routes messages and maybe stores some shared state. Not the center of gravity. Not the gatekeeper. Not the thing that defines how work gets done.

The contracts define how work gets done. The humans (and their agents) do the work. The platform, if it exists at all, is plumbing.

## The Part Where We're Honest

None of this works today.

KERI exists. ACDCs are specified. LLMs are impressive. But the integration — the seamless flow from identity to contract to AI-assisted fulfillment to cryptographic attestation — that's not built yet. Not even close.

We're currently building traditional platforms. Login screens. Dashboards. Role-based access. Because that's what works right now, and people need to see things, click things, and understand what they're looking at.

The vision of "just give me an AI that knows my contracts" is coherent. It's architecturally sound. And it's probably years away from being usable by anyone who isn't deeply technical.

The gap between here and there isn't just technical. It's adoption. Mass adoption of KERI identifiers. Standardized ACDC schemas for common contract types. LLMs that are reliable enough to be trusted with contractual compliance. Credential ecosystems that are rich enough to cover real-world professional qualifications.

That's a lot of gap.

But the direction is clear. Every platform we build today is a bridge. Every workflow we hard-code is temporary. Every role-based access control matrix is a pale shadow of what a contract-native system would provide.

We know where this is going. We just have to get there without pretending we're already there.

## What Gets Built in the Meantime

The pragmatic path looks like this:

**Today:** Build platforms that work. Ship UIs people can use. Solve real problems with current technology. But architect with contracts in mind. Make role definitions explicit. Make obligations machine-readable where possible. Make every action attributable.

**Soon:** Start expressing role definitions and workflows as ACDCs alongside the traditional platform. Run them in parallel. Let the contract layer prove itself while the UI layer keeps people productive.

**Eventually:** The contracts become the source of truth. The UI becomes generated. The platform becomes optional. The agent becomes the interface.

Every step is useful. Every step works. No step requires believing in the end state to deliver value.

## The Quiet Revolution

Here's what's strange about this vision: it's not really about technology. It's about recognizing what applications have always been — clumsy translations of human agreements into software — and asking why we keep translating when the agreements could speak for themselves.

A contract between two parties, expressed as verifiable data, understood by an AI, fulfilled by the parties themselves with cryptographic proof of completion.

That's not a new kind of application. It's the end of applications as a category.

What comes after is simpler. More direct. More human. You have your identity. You have your agreements. You have an agent that helps you fulfill them. Everything else — the platforms, the portals, the dashboards — was always just scaffolding.

We've been building increasingly elaborate scaffolding and calling it the building.

The building is the contracts. It always was.

**Related:** [Elon Is Right About the What](/blog/2026/02/14/elon-is-right-about-the-what/) explores what happens when this vision collides with Big Tech's version of AI-replaces-everything.

{% comment %}TODO: Add technical architecture showing ACDC schema design for role-based contracts, concrete examples of contract-to-UI generation with current LLM capabilities, comparison with smart contracts (and why this is fundamentally different), and exploration of the credential ecosystem needed for professional qualifications{% endcomment %}
