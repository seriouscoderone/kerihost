---
title: "Services"
permalink: /services/
layout: single
author_profile: true
toc: true
toc_sticky: true
description: "KERI consulting that makes you independent, not dependent. Ecosystem design, custom applications, infrastructure, and training."
---

## The Goal Is Your Independence

KERI.host offers consulting services with one guiding principle: every engagement ends with you owning everything we built together.

Your infrastructure. Your credentials. Your governance. Your keys. We help you stand it up, then we step away.

This isn't a managed service you subscribe to. It's not a platform you log into. We don't hold your data, control your identifiers, or insert ourselves as a permanent dependency. If you never need us again after the engagement, that's the best possible outcome.

## Ecosystem Design

You have an industry. You see the bundling, the lock-in, the intermediaries that extract value without adding it. You know KERI can restructure the trust relationships — but you don't know where to start.

We do this work with you, not for you. Through structured conversations, we map your industry's trust relationships, identify which intermediaries add real value, design credential schemas, define roles and delegation trees, and produce a complete governance framework.

**What you get:**
- A machine-readable ecosystem specification (roles, credentials, delegation hierarchies)
- A credential catalog with schemas, disclosure modes, and chaining relationships
- A trust framework covering governance, privacy, dispute resolution, and liability
- Diagrams: role interactions, credential flows, delegation trees, dependency graphs

**What you own:** Everything. The YAML files, the governance documents, the credential schemas. Fork them. Modify them. Use them without us.

We've published our ecosystem designs openly — [genealogy](/ecosystems/genealogy/), [humanitarian services](/ecosystems/humanitarian-service-marketplace/), [small trades](/ecosystems/small-trades-cooperative/) — so you can see exactly what this work looks like before you engage.

## Custom KERI Applications

You need software that issues credentials, verifies identities, manages key rotation, or coordinates multi-party workflows — and it needs to work with your existing systems.

We build custom applications on the KERI stack: keriox (Rust), keripy (Python), signify-ts (TypeScript). Serverless on AWS Lambda, containerized on ECS, or browser-based with the Signify protocol. Whatever fits your architecture.

**What you get:**
- Production application code, tested and deployed
- Integration with your existing APIs, databases, and authentication systems
- Documentation sufficient for your team to maintain and extend the application

**What you own:** The source code, the deployment, the infrastructure. No license keys, no proprietary dependencies, no callbacks to our servers.

## Custom KERI Infrastructure

You need witness pools, watcher networks, KERI agents, or ACDC registries — deployed, monitored, and running in your own cloud account.

We design and deploy KERI infrastructure tailored to your requirements: environment sizing, security posture, geographic distribution, high availability. Multi-AZ production deployments or minimal dev environments. Your AWS account, your control.

**What you get:**
- Infrastructure-as-code (CDK/CloudFormation) deployed to your AWS account
- Witness pools, watcher nodes, agent services, ACDC registries — whatever your ecosystem requires
- Monitoring, alerting, and operational runbooks
- Knowledge transfer so your team can operate and scale the infrastructure independently

**What you own:** The AWS account, the CDK stacks, the running infrastructure, the operational knowledge. We hand over the keys and walk away.

## Credential Schema Design

You know what information needs to flow between parties, but you need the credential architecture: ACDC schemas, disclosure modes (full, selective, partial), chaining relationships, revocation policies, and privacy constraints.

This is the detailed work that sits between ecosystem governance and running software. Getting schemas right means your credentials are interoperable, privacy-preserving, and legally sound.

**What you get:**
- ACDC credential schemas for your use case
- Disclosure mode recommendations (what to reveal, what to withhold, under what conditions)
- Chaining architecture (which credentials depend on which)
- Revocation and lifecycle policies

**What you own:** The schemas, the documentation, the design rationale. Publish them, modify them, share them with your ecosystem partners.

## KERI Integration

You have existing applications — web apps, mobile apps, backend services — and you want to add KERI-based identity, authentication, or credential verification without rewriting everything.

We integrate KERI into your stack: signify-ts for browser-based signing, the Signify browser extension for passwordless authentication, KERI agent APIs for your backend, credential verification endpoints for your existing workflows.

**What you get:**
- KERI identity and credential capabilities integrated into your existing applications
- Authentication flows that use cryptographic proof instead of passwords
- Credential verification endpoints your existing systems can call
- Migration path documentation for transitioning users from legacy auth

**What you own:** The integrated code, the deployment, the auth flows. No external service dependency.

## Training

Your engineering team needs to understand KERI, CESR, ACDC, and the Signify protocol well enough to build and maintain systems independently.

We offer workshops tailored to your team's existing knowledge and your specific use case. Not abstract protocol theory — grounded in the systems you're actually building.

**Topics we cover:**
- KERI fundamentals: identifiers, key events, witnesses, watchers, delegation, pre-rotation
- CESR encoding: code tables, qualified primitives, stream parsing, SAID derivation
- ACDC credentials: schemas, graduated disclosure, IPEX exchange, TEL lifecycle
- Signify protocol: edge signing, agent communication, browser integration
- Infrastructure operations: witness pool management, watcher networks, key rotation procedures

**What you get:** A team that can build, operate, and extend KERI systems without us.

## Hosted KERI Infrastructure

Not everyone wants to operate their own witness pools, watcher nodes, and KERI agents. That's fine. We'll run them for you.

KERI identity requires infrastructure that stays online: witnesses that receipt your key events, agents that host your identifier state and credential wallet, watchers that monitor for duplicity, and OOBI endpoints that let others discover and verify your identifiers. This infrastructure needs to be available, monitored, geographically distributed, and correctly implementing the protocol. Running it well is operational work that most organizations would rather not take on.

Here's what makes this different from every other hosted identity service: **your private keys never touch our servers.**

KERI's architecture separates the "cloud half" from the "edge half" through the Signify protocol. We run the cloud half — the agents, witnesses, and watchers. You keep the edge half — your keys, on your device. We can host your identifier state, but we cannot sign as you, impersonate you, or lock you out. The cryptographic design makes this impossible, not just a policy we promise to follow.

And because your Key Event Log is portable, you can migrate to another host or to your own infrastructure at any time. No export process, no data request, no negotiation. Your KEL is yours. Take it and go whenever you want.

**What we host:**
- **Witness pools** — geographically distributed witnesses that receipt your key events, with configurable threshold and rotation
- **KERI agents** — cloud agents (KERIA) that manage your identifiers, credential wallet, and witness coordination
- **Watcher nodes** — duplicity detection and first-seen monitoring for your identifiers
- **OOBI endpoints** — discovery services so others can find and verify your identifiers and credentials

**What you keep:** Your private keys. Your signing authority. Your ability to leave.

## Protocol Compliance Review

You've built a KERI implementation — or you're evaluating one — and you need confidence that it correctly implements the specification.

We review implementations against the KERI, CESR, and ACDC specifications. KEL validation rules, witness agreement (KAACE), delegation chains, key rotation, pre-rotation commitments, first-seen semantics, ACDC disclosure modes, TEL lifecycle.

**What you get:**
- Detailed findings report: spec compliance, edge cases, potential vulnerabilities
- Specific code-level recommendations
- Verification against the reference implementations (keripy, keriox)

## How We Work

Every engagement starts with a conversation. We understand your problem before proposing solutions. We scope work to deliver specific, ownable outcomes — not open-ended retainers.

We're a small operation. We work directly with the people building the thing. No account managers, no layers of abstraction. You talk to the engineers who do the work.

Reach out: [consulting@keri.host](mailto:consulting@keri.host)

---

*KERI.host is infrastructure for the rest of us. We help you build your own — then we get out of the way.*
