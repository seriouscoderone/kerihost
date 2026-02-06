# Cross-Org Workflow Without Orchestration — Technical Pattern

**Type:** Architecture pattern document

**Priority:** 🟢 Phase 1 (Months 3–6)

---

## Purpose

Deep dive into the "no central workflow engine" pattern with concrete flow diagrams.

## Core Principles

### Each Org Runs Its Own Software
- No shared infrastructure required
- Independence preserved
- Upgrade at your own pace

### No Shared Database or Workflow Engine
- State is local
- Coordination through messages
- No single source of truth

### No Super-Admin
- No privileged observer
- No central control point
- No master key

### Only Cryptographically Provable State Transitions
- Every change is signed
- Every claim is verifiable
- Audit trail is built-in

## Applied Patterns

### Procurement
- Vendor submits bid with credentials
- Purchasing agent evaluates and attests
- Approval chain signs off
- Delivery confirmed by receiver
- Payment authorized by finance

### Permitting
- Applicant submits with qualifications
- Reviewer evaluates and attests
- Inspector verifies and signs
- Certificate issued as ACDC

### Hiring
- Candidate presents credentials
- Employer verifies qualifications
- Offer extended and accepted
- Employment attested

### Lending
- Borrower presents identity and income
- Lender evaluates and offers terms
- Agreement signed by both parties
- Payments attested as completed

### Licensing
- Applicant demonstrates qualifications
- Authority evaluates and attests
- License issued as ACDC
- Renewals update the credential

### Membership Onboarding
- Applicant requests membership
- Existing members vouch/sponsor
- Organization attests membership
- Member receives credential

---

*TODO: Add flow diagrams and detailed examples*
