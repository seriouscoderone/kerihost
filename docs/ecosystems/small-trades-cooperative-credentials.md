---
title: "Small Trades Cooperative — Credentials"
permalink: /ecosystems/small-trades-cooperative/credentials/
layout: single
toc: true
toc_sticky: true
description: "Complete credential catalog for the Small Trades Cooperative ecosystem."
---

This is the complete credential catalog for the Small Trades Cooperative ecosystem. Each credential below includes its issuer, holder, verifiers, schema fields, disclosure mode, and chaining relationships. Credentials and schemas are illustrative examples showing the type of data each credential would carry, not finalized specifications.

[Back to Small Trades Cooperative overview](/ecosystems/small-trades-cooperative/)

### Trade License

State-issued license to practice a specific trade. The entry floor — a valid license means you can work today, regardless of experience level.

| Property | Value |
|----------|-------|
| **ID** | `trade_license` |
| **Issuer** | State Licensing Board |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, City Building Department, Trades Cooperative, Credentialed Inspector |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `trade_category` | string | yes |
| `license_number` | string | yes |
| `jurisdiction` | string | yes |
| `issue_date` | date | yes |
| `expiration_date` | date | yes |
| `license_status` | string | yes |
| `restrictions` | string[] | no |

### Insurance Coverage

Proof of liability insurance. Verifiers see coverage status and amount but not policy details, premiums, or claims history. Auto-expires when the policy lapses — no more stale certificates floating around.

| Property | Value |
|----------|-------|
| **ID** | `insurance_credential` |
| **Issuer** | External: Insurance Ecosystem |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, City Building Department, Trades Cooperative |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `coverage_type` | string | yes |
| `coverage_amount` | number | yes |
| `policy_status` | string | yes |
| `effective_date` | date | yes |
| `expiration_date` | date | yes |
| `insurer_name` | string | yes |

### Surety Bond

Proof of surety bond for larger jobs. Guarantees financial recourse if work is not completed. In the mature ecosystem, escrow may replace most bonding needs for smaller jobs.

| Property | Value |
|----------|-------|
| **ID** | `bond_credential` |
| **Issuer** | External: Surety Ecosystem |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, City Building Department |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `bond_amount` | number | yes |
| `bond_type` | string | yes |
| `surety_company` | string | yes |
| `effective_date` | date | yes |
| `expiration_date` | date | yes |

### Background Check

Pass/fail background check. The credential says "cleared" — not "here's the report." Issued once, held by the worker, verifiable by any coop without re-running the check.

| Property | Value |
|----------|-------|
| **ID** | `background_check` |
| **Issuer** | External: Issuing Agency |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Trades Cooperative |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `check_type` | string | yes |
| `result` | string | yes |
| `check_date` | date | yes |
| `valid_until` | date | yes |
| `issuing_agency` | string | yes |

### Drug Screening

Pass/fail drug screening result. Disclosed to coop membership verification only — homeowners see "coop member" which implies the coop's requirements are met. Worker controls further disclosure.

| Property | Value |
|----------|-------|
| **ID** | `drug_test` |
| **Issuer** | External: Testing Facility |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Trades Cooperative |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `test_type` | string | yes |
| `result` | string | yes |
| `test_date` | date | yes |
| `valid_until` | date | yes |

### Safety Certification

Job site safety training completion (OSHA 10, OSHA 30, trade-specific certifications). Full disclosure — there's no reason to hide that you're safety-trained.

| Property | Value |
|----------|-------|
| **ID** | `safety_certification` |
| **Issuer** | External: Training Provider |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, Credentialed Inspector, City Building Department, Trades Cooperative |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `certification_type` | string | yes |
| `issuing_organization` | string | yes |
| `completion_date` | date | yes |
| `expiration_date` | date | no |
| `hours_completed` | number | yes |

### Skill Level Designation

Apprentice, journeyman, or master designation for a specific trade. Issued by the licensing board (formal path) or by a master tradesperson (mentorship path). Both are verifiable — the market decides which it values more.

| Property | Value |
|----------|-------|
| **ID** | `skill_level` |
| **Issuer** | State Licensing Board |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, Trades Cooperative, Credentialed Inspector |
| **Disclosure** | full |
| **Chained From** | Trade License (`trade_license`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `trade_category` | string | yes |
| `level` | string | yes |
| `attested_by` | string | yes |
| `attestation_date` | date | yes |
| `years_experience` | number | no |

### Apprenticeship Attestation

Issued by a master tradesperson to their apprentice. Records scope of training, duration, and mentor identity. Creates a verifiable lineage — who trained you, who trained them. The mentor stakes their reputation on the apprentice's competence.

| Property | Value |
|----------|-------|
| **ID** | `apprenticeship_attestation` |
| **Issuer** | Master Tradesperson |
| **Holder** | Apprentice |
| **Verifiers** | Property Owner / Client, Trades Cooperative, State Licensing Board |
| **Disclosure** | full |
| **Chained From** | Master Designation (`master_designation`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `mentor_aid` | string | yes |
| `mentor_name` | string | yes |
| `trade_category` | string | yes |
| `training_scope` | string[] | yes |
| `start_date` | date | yes |
| `end_date` | date | no |
| `hours_completed` | number | yes |
| `competency_areas` | string[] | yes |

### Master Designation

Recognized master-level practitioner in a specific trade. Authorizes the holder to mentor apprentices and issue apprenticeship attestations. In the mature ecosystem, a master with strong reputation becomes a trust anchor independent of institutional backing.

| Property | Value |
|----------|-------|
| **ID** | `master_designation` |
| **Issuer** | State Licensing Board |
| **Holder** | Master Tradesperson |
| **Verifiers** | Property Owner / Client, Trades Cooperative, Credentialed Inspector, Apprentice |
| **Disclosure** | full |
| **Chained From** | Trade License (`trade_license`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `trade_category` | string | yes |
| `designation_date` | date | yes |
| `years_in_trade` | number | yes |
| `specializations` | string[] | no |

### Cooperative Membership

Active member of a specific cooperative. Implies the coop's published membership requirements are met — background check, drug test, insurance minimums, whatever the coop demands. Workers present this instead of individual screening credentials to homeowners.

| Property | Value |
|----------|-------|
| **ID** | `coop_membership` |
| **Issuer** | Trades Cooperative |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, Connector / Recommendation Platform |
| **Disclosure** | full |
| **Chained From** | Coop Governance Framework (`coop_governance_framework`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `coop_name` | string | yes |
| `coop_aid` | string | yes |
| `membership_date` | date | yes |
| `trade_categories` | string[] | yes |
| `membership_tier` | string | no |

### Coop Governance Framework

Public document of membership requirements and standards. This is how homeowners see what "member of XYZ Coop" actually means. Different coops have different requirements — premium coops demand more, budget coops demand the legal minimum. Both are transparent.

| Property | Value |
|----------|-------|
| **ID** | `coop_governance_framework` |
| **Issuer** | Trades Cooperative |
| **Holder** | Trades Cooperative |
| **Verifiers** | Property Owner / Client, Independent Tradesperson, Connector / Recommendation Platform |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `required_credentials` | string[] | yes |
| `minimum_insurance_amount` | number | yes |
| `background_check_required` | boolean | yes |
| `drug_test_required` | boolean | yes |
| `minimum_skill_level` | string | no |
| `continuing_education_required` | boolean | no |
| `version` | string | yes |
| `effective_date` | date | yes |

### Coop Registration

State or city registration as a legally recognized cooperative entity.

| Property | Value |
|----------|-------|
| **ID** | `coop_registration` |
| **Issuer** | State Licensing Board |
| **Holder** | Trades Cooperative |
| **Verifiers** | Property Owner / Client, Independent Tradesperson, City Building Department |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `registration_number` | string | yes |
| `jurisdiction` | string | yes |
| `registration_date` | date | yes |
| `entity_type` | string | yes |

### Job Completion Attestation

The atomic unit of reputation. Homeowner attests that a specific job was completed by a specific tradesperson. Not a review — just a fact. Signed as part of contract close-out with zero extra effort. Workers accumulate these as portable, verifiable work history.

| Property | Value |
|----------|-------|
| **ID** | `job_completion_attestation` |
| **Issuer** | Property Owner / Client |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, Trades Cooperative, Connector / Recommendation Platform, Credentialed Inspector |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `job_type` | string | yes |
| `job_description` | string | yes |
| `completion_date` | date | yes |
| `property_type` | string | yes |
| `trade_category` | string | yes |
| `contract_aid` | string | yes |
| `escrow_released_clean` | boolean | yes |

### Financial Responsibility

Proof that the homeowner can fund escrow for a given amount. Financial institution attests fundability without revealing bank balance, account details, or any other financial information. The tradesperson sees "escrow fundable to $X" — that's it.

| Property | Value |
|----------|-------|
| **ID** | `financial_responsibility` |
| **Issuer** | External: Financial Institution |
| **Holder** | Property Owner / Client |
| **Verifiers** | Independent Tradesperson, Escrow Protocol Service |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `fundable_amount` | number | yes |
| `attestation_date` | date | yes |
| `valid_until` | date | yes |

### Property Ownership

Proof of property ownership for permit and contract purposes. Selective disclosure — prove you own the property without revealing purchase price or mortgage details.

| Property | Value |
|----------|-------|
| **ID** | `property_credential` |
| **Issuer** | City Building Department |
| **Holder** | Property Owner / Client |
| **Verifiers** | Independent Tradesperson, City Building Department, Credentialed Inspector |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `property_address` | string | yes |
| `owner_name` | string | yes |
| `parcel_id` | string | yes |
| `ownership_type` | string | yes |

### Inspector License

Authorization to inspect and attest code compliance. In the mature ecosystem, private inspectors with this credential compete with city inspectors on speed and availability, breaking the inspection bottleneck.

| Property | Value |
|----------|-------|
| **ID** | `inspector_license` |
| **Issuer** | State Licensing Board |
| **Holder** | Credentialed Inspector |
| **Verifiers** | City Building Department, Property Owner / Client, Independent Tradesperson |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `inspection_categories` | string[] | yes |
| `jurisdiction` | string | yes |
| `license_number` | string | yes |
| `issue_date` | date | yes |
| `expiration_date` | date | yes |

### Code Compliance Attestation

Specific work items verified against building code schema. Inspector attests each item individually — electrical passed, plumbing passed, ventilation passed. City accepts the complete attestation set automatically when issued by a credentialed inspector.

| Property | Value |
|----------|-------|
| **ID** | `code_compliance_attestation` |
| **Issuer** | Credentialed Inspector |
| **Holder** | Independent Tradesperson |
| **Verifiers** | City Building Department, Property Owner / Client |
| **Disclosure** | full |
| **Chained From** | Inspector License (`inspector_license`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `permit_number` | string | yes |
| `inspection_type` | string | yes |
| `code_section` | string | yes |
| `result` | string | yes |
| `inspection_date` | date | yes |
| `deficiencies` | string[] | no |
| `re_inspection_required` | boolean | yes |
| `inspector_aid` | string | yes |

### Mediator Credential

Authorization to mediate disputes in trade contracts. Issued by a recognized arbitration body. Mediator selection happens before work begins — both parties agree in the contract.

| Property | Value |
|----------|-------|
| **ID** | `mediator_credential` |
| **Issuer** | External: Arbitration Body |
| **Holder** | Dispute Mediator |
| **Verifiers** | Property Owner / Client, Independent Tradesperson, Escrow Protocol Service |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `certification_type` | string | yes |
| `issuing_body` | string | yes |
| `specializations` | string[] | no |
| `issue_date` | date | yes |
| `expiration_date` | date | yes |

### Mediation Ruling

Binding ruling on a dispute. Triggers escrow release or reallocation. The existence of a ruling and its outcome are part of the public record; the details are contractual disclosure only.

| Property | Value |
|----------|-------|
| **ID** | `mediation_ruling` |
| **Issuer** | Dispute Mediator |
| **Holder** | Property Owner / Client |
| **Verifiers** | Escrow Protocol Service |
| **Disclosure** | contractual |
| **Chained From** | Mediator Credential (`mediator_credential`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `contract_aid` | string | yes |
| `ruling_date` | date | yes |
| `ruling_summary` | string | yes |
| `escrow_disposition` | string | yes |
| `amount_to_tradesperson` | number | yes |
| `amount_to_homeowner` | number | yes |
| `mediator_fee` | number | yes |

### Escrow Contract

Funds locked between contract signing and job completion. Defines release conditions, milestone schedule, mediator designation, and mediator fee. Both parties sign before work begins. Cross-ecosystem primitive — this pattern applies to any service contract.

| Property | Value |
|----------|-------|
| **ID** | `escrow_contract` |
| **Issuer** | Escrow Protocol Service |
| **Holder** | Property Owner / Client |
| **Verifiers** | Independent Tradesperson, Dispute Mediator |
| **Disclosure** | contractual |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `total_amount` | number | yes |
| `milestones` | object[] | no |
| `release_conditions` | string | yes |
| `mediator_aid` | string | yes |
| `mediator_fee_percentage` | number | yes |
| `dispute_deadline_days` | number | yes |
| `contract_date` | date | yes |
| `homeowner_aid` | string | yes |
| `tradesperson_aid` | string | yes |

### Escrow Release

Funds released — either clean close (mutual attestation) or mediated ruling. The release record shows outcome (clean or mediated) but not amounts. A tradesperson with many clean releases has a strong signal.

| Property | Value |
|----------|-------|
| **ID** | `escrow_release` |
| **Issuer** | Escrow Protocol Service |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Property Owner / Client, Trades Cooperative, Connector / Recommendation Platform |
| **Disclosure** | selective |
| **Chained From** | Escrow Contract (`escrow_contract`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `contract_aid` | string | yes |
| `release_type` | string | yes |
| `release_date` | date | yes |
| `clean_release` | boolean | yes |

### Witnessed Recording

Hardware-attested photo or video of work — timestamped, geolocated, and witnessed by KERI witnesses. A locked box: existence is known, content is encrypted, access requires a credentialed adjudicator and a dispute trigger defined in the contract. Both parties can create witnessed recordings.

| Property | Value |
|----------|-------|
| **ID** | `witnessed_recording` |
| **Issuer** | Hardware Device Attestor |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Dispute Mediator, Credentialed Inspector |
| **Disclosure** | contractual |
| **Chained From** | Hardware Device Attestation (`hardware_attestation`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `recording_type` | string | yes |
| `capture_timestamp` | datetime | yes |
| `geolocation` | string | yes |
| `device_aid` | string | yes |
| `content_hash` | string | yes |
| `access_conditions` | string | yes |

### Hardware Device Attestation

Certifies that a device produces authentic, unmodified media. Enables the physical-to-digital bridge — without this, photos and videos have no cryptographic provenance.

| Property | Value |
|----------|-------|
| **ID** | `hardware_attestation` |
| **Issuer** | Hardware Device Attestor |
| **Holder** | Independent Tradesperson |
| **Verifiers** | Credentialed Inspector, Dispute Mediator |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `device_model` | string | yes |
| `device_serial` | string | yes |
| `manufacturer` | string | yes |
| `certification_standard` | string | yes |
| `certification_date` | date | yes |

### Connector Registration

Registration for a recommendation/matching service. Deliberately lightweight — connectors are optional, replaceable, and must not accumulate power over reputation data.

| Property | Value |
|----------|-------|
| **ID** | `connector_registration` |
| **Issuer** | Trades Cooperative |
| **Holder** | Connector / Recommendation Platform |
| **Verifiers** | Property Owner / Client, Independent Tradesperson |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `service_name` | string | yes |
| `service_type` | string | yes |
| `coverage_area` | string | yes |
| `registration_date` | date | yes |

---

*[Back to Small Trades Cooperative overview](/ecosystems/small-trades-cooperative/)*

*Generated from [`docs/small-trades-cooperative/ecosystem.yaml`](https://github.com/seriouscoderone/kerihost/blob/main/docs/small-trades-cooperative/ecosystem.yaml) by the `/ecosystem-package` skill.*
