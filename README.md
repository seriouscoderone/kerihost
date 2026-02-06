# KERI.host

**Reference-grade infrastructure for KERI-native economies.**

KERI.host is an open, forkable reference implementation for building communities and governance structures on cryptographic accountability. It is not a SaaS provider, wallet company, or workflow engine. It is a pattern library — infrastructure that gets out of the way.

## What This Is

- A **reference implementation** of KERI witness services on serverless infrastructure (AWS Lambda + DynamoDB)
- **Forkable and self-hostable** — if KERI.host disappears, nothing breaks
- **Optional hosting** — every component can be run independently without KERI.host infrastructure
- Infrastructure for **community builders**, not just developers

## What This Is Not

- Not an identity provider
- Not a platform monopoly
- Not a global reputation system
- Not a gatekeeper to participation
- Cannot revoke or invalidate anyone's AID
- Cannot be a root authority for credentials

## Architecture

The infrastructure is organized into three AWS CDK stacks:

```
┌─────────────────────────────────────────────────────────────────┐
│                         CDK App                                 │
└─────────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│  DataStack    │    │   ApiStack    │    │ WitnessStack  │
│               │    │               │    │               │
│ • DynamoDB    │◄───│ • API Gateway │◄───│ • Lambdas     │
│ • Secrets     │    │ • Certificate │    │ • Routes      │
│               │    │ • DNS         │    │ • EventBridge │
└───────────────┘    └───────────────┘    └───────────────┘
     (Layer 1)           (Layer 2)           (Layer 3)
```

**DataStack** — Persistent storage (DynamoDB tables for KEL, states, receipts, escrows)

**ApiStack** — Shared API infrastructure (API Gateway, ACM certificate, Route53)

**WitnessStack** — KERI witness service (Rust Lambda functions, API routes)

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/witness` | GET | Witness OOBI |
| `/witness/introduce` | GET | Get witness OOBI |
| `/witness/process` | POST | Submit KERI events |
| `/witness/query` | POST | Query KEL, state, receipts |
| `/witness/oobi/{id}` | GET | Resolve OOBI for identifier |

## Development

### Prerequisites

- Rust 1.88+
- Node.js (LTS)
- AWS CLI configured
- [cargo-lambda](https://www.cargo-lambda.info/)

### Build

```bash
# Build Rust lambdas
cargo build --release

# Build CDK infrastructure
cd infrastructure
npm install
npm run build
```

### Deploy

```bash
cd infrastructure

# Deploy all stacks
npx cdk deploy --all -c hostedZoneId=YOUR_HOSTED_ZONE_ID

# Or deploy individually
npx cdk deploy KerihostDataStack
npx cdk deploy KerihostApiStack -c hostedZoneId=YOUR_HOSTED_ZONE_ID
npx cdk deploy KerihostWitnessStack
```

### Test

```bash
# Run integration tests
cd tests/integration
npm install
npm test
```

## Vision

See [docs/keri-host-artifact-plan.md](docs/keri-host-artifact-plan.md) for the full KERI.host vision and roadmap.

### Core Principles

1. **Reference > Platform** — Forkable, self-hostable, documented as patterns
2. **Optional Hosting, Never Required** — Every demo runnable without KERI.host
3. **Simulated but Real** — Flight simulators, not PowerPoints
4. **Subjective, Plural Reputation** — No universal score
5. **Regulator-Friendly** — Audit trails without surveillance
6. **Cooperation, Not Competition** — Non-profit, no ecosystem wars

## License

Apache License 2.0 — See [LICENSE](LICENSE)

## Contributing

Contributions welcome. This is reference infrastructure meant to be forked, adapted, and improved by the community.
