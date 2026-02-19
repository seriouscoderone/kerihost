---
title: "Humanitarian Service Marketplace — Credentials"
permalink: /ecosystems/humanitarian-service-marketplace/credentials/
layout: single
toc: true
toc_sticky: true
description: "Complete credential catalog for the Humanitarian Service Marketplace ecosystem."
---

This is the complete credential catalog for the Humanitarian Service Marketplace ecosystem. Each credential below includes its issuer, holder, verifiers, schema fields, disclosure mode, and chaining relationships. Credentials and schemas are illustrative examples showing the type of data each credential would carry, not finalized specifications.

[Back to Humanitarian Service Marketplace overview](/ecosystems/humanitarian-service-marketplace/)

### State-Endorsed Digital Identity

Government-issued digital identity backed by KERI protocols. The gold-standard root of identity — Utah is leading adoption with multi-state expansion underway. Provides the foundational identity that all other credentials reference.

| Property | Value |
|----------|-------|
| **ID** | `sedi_identity` |
| **Issuer** | Government Agency |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, KYV Provider, Credential Provider, Government Agency |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `legal_name` | string | yes |
| `date_of_birth` | date | yes |
| `jurisdiction` | string | yes |
| `sedi_id` | string | yes |
| `address` | string | no |
| `photo_hash` | string | no |

### Know Your Volunteer Credential

Composable credential attesting that a volunteer has been vetted to a specific standard. Built from component credentials — background checks, training certs, psych evals. Multiple KYV standards coexist, each with different compositions and trust levels. ACDC chaining makes the composition transparent so verifiers see exactly what went into it.

| Property | Value |
|----------|-------|
| **ID** | `kyv_credential` |
| **Issuer** | KYV Provider |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Individual |
| **Disclosure** | selective |
| **Chained From** | Background Check Result (`background_check`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `kyv_standard_said` | string | yes |
| `vetting_date` | date | yes |
| `expiration_date` | date | yes |
| `component_credentials` | string[] | yes |
| `verification_methods` | string[] | yes |
| `in_person_verified` | boolean | yes |

### Skill/Training Certificate

Credential issued by a training provider, trade school, or licensing body attesting to a specific skill. CPR certification, electrical license, painting certification, medical training — the issuing entity is also the verifier.

| Property | Value |
|----------|-------|
| **ID** | `skill_credential` |
| **Issuer** | Credential Provider |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Individual, KYV Provider |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `skill_name` | string | yes |
| `skill_category` | string | yes |
| `issuing_institution` | string | yes |
| `issue_date` | date | yes |
| `expiration_date` | date | no |
| `skill_level` | string | no |

### Verified Service Record

Credential attesting that a volunteer performed a specific service. Accumulates over time to build portable reputation. Includes verification method metadata (geolocation, biometric, digital tool, manual attestation) so verifiers can assess proof strength.

| Property | Value |
|----------|-------|
| **ID** | `service_record` |
| **Issuer** | Service Platform |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Government Agency, Individual |
| **Disclosure** | selective |
| **Chained From** | Proof of Service (`proof_of_service`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `service_description` | string | yes |
| `service_date` | date | yes |
| `hours_served` | number | yes |
| `service_category` | string | yes |
| `verification_method` | string | yes |
| `verification_strength` | string | yes |
| `location` | string | no |
| `requester_satisfaction` | string | no |
| `skills_applied` | string[] | no |

### Equipment/Tool Availability

Self-asserted credential listing tools and equipment a volunteer has access to — from gloves and shovels to backhoes and heavy equipment. Can be endorsed by organizations that have witnessed the equipment in use, which increases trust.

| Property | Value |
|----------|-------|
| **ID** | `equipment_inventory` |
| **Issuer** | Individual |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Individual |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `equipment_name` | string | yes |
| `equipment_category` | string | yes |
| `availability` | string | yes |
| `endorsements` | string[] | no |

### Organization Membership

Credential attesting that an individual is a member of a service organization or grassroots group. Enables cross-platform discovery and attribution tracking.

| Property | Value |
|----------|-------|
| **ID** | `org_membership` |
| **Issuer** | Service Platform |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Individual |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `organization_name` | string | yes |
| `membership_type` | string | yes |
| `join_date` | date | yes |
| `status` | string | yes |

### Service Need Publication

Declarative credential publishing a need for service. Uses selective disclosure to protect requester vulnerability — only qualified volunteers see full details. Publication scope follows a graduated model: friends/family, neighborhood, community, city, organizations.

| Property | Value |
|----------|-------|
| **ID** | `service_need` |
| **Issuer** | Individual |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Individual |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `need_description` | string | yes |
| `service_category` | string | yes |
| `location` | string | yes |
| `timeframe` | string | yes |
| `required_skills` | string[] | no |
| `required_credentials` | string[] | no |
| `required_kyv_level` | string | no |
| `publication_scope` | string | yes |
| `formality_level` | string | yes |
| `estimated_hours` | number | no |
| `volunteers_needed` | number | no |

### Service Commitment Agreement

Contractual credential where a volunteer commits to meeting a published service need. Binds the volunteer to the terms. Forms the basis for proof-of-service verification — you cannot prove you did something you did not agree to do.

| Property | Value |
|----------|-------|
| **ID** | `service_commitment` |
| **Issuer** | Individual |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Individual |
| **Disclosure** | full |
| **Chained From** | Service Need Publication (`service_need`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `service_need_said` | string | yes |
| `volunteer_aid` | string | yes |
| `commitment_date` | date | yes |
| `agreed_terms` | string | yes |
| `verification_plan` | string | yes |

### Proof of Service

Automated or manual proof that service was rendered. Multi-modal verification: geolocation + barometric + biometric (phone KEL logs), digital tool interaction logs, or manual sponsor attestation. Each method produces a different verification strength.

| Property | Value |
|----------|-------|
| **ID** | `proof_of_service` |
| **Issuer** | Verification Service |
| **Holder** | Individual |
| **Verifiers** | Service Platform, Grassroots Organization, Individual, Government Agency |
| **Disclosure** | selective |
| **Chained From** | Service Commitment Agreement (`service_commitment`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `service_commitment_said` | string | yes |
| `verification_method` | string | yes |
| `verification_strength` | string | yes |
| `start_time` | datetime | yes |
| `end_time` | datetime | yes |
| `location_proofs` | string[] | no |
| `biometric_confirmed` | boolean | no |
| `digital_tool_logs` | string[] | no |
| `manual_attestation_aid` | string | no |

### Platform Registration

Government-issued credential recognizing a service platform as a registered nonprofit or service organization. Not required for grassroots participation but necessary for formal organizations seeking tax-exempt status and government reporting integration.

| Property | Value |
|----------|-------|
| **ID** | `platform_registration` |
| **Issuer** | Government Agency |
| **Holder** | Service Platform |
| **Verifiers** | Government Agency, Service Platform, Individual |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `organization_name` | string | yes |
| `registration_number` | string | yes |
| `jurisdiction` | string | yes |
| `registration_type` | string | yes |
| `registration_date` | date | yes |
| `tax_exempt_status` | string | no |

### KYV Standard Definition

Published definition of a Know Your Volunteer standard — specifying what component credentials are required, what verification methods are accepted, and what governance processes must be followed. Multiple standards coexist. Organizations can adopt existing standards or define their own.

| Property | Value |
|----------|-------|
| **ID** | `kyv_standard` |
| **Issuer** | Service Platform |
| **Holder** | KYV Provider |
| **Verifiers** | KYV Provider, Individual, Service Platform |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `standard_name` | string | yes |
| `version` | string | yes |
| `required_components` | string[] | yes |
| `accepted_verification_methods` | string[] | yes |
| `in_person_required` | boolean | yes |
| `renewal_period` | string | yes |

### Interoperability Agreement

Mutual credential between service platforms establishing cross-platform interoperability — shared service need publication, volunteer credential recognition, and service record portability. Defines which KYV standards each platform recognizes from the other.

| Property | Value |
|----------|-------|
| **ID** | `interop_agreement` |
| **Issuer** | Service Platform |
| **Holder** | Service Platform |
| **Verifiers** | Service Platform, Grassroots Organization |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `partner_platform_aid` | string | yes |
| `agreement_date` | date | yes |
| `recognized_kyv_standards` | string[] | yes |
| `shared_service_categories` | string[] | yes |
| `attribution_terms` | string | yes |

### Background Check Result

Credential issued by a verification service attesting to the results of a background check. A component credential that feeds into KYV credentials. Uses selective disclosure — the KYV provider sees what they need, but full details are not exposed beyond that.

| Property | Value |
|----------|-------|
| **ID** | `background_check` |
| **Issuer** | Verification Service |
| **Holder** | Individual |
| **Verifiers** | KYV Provider |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `check_type` | string | yes |
| `check_date` | date | yes |
| `result` | string | yes |
| `jurisdiction` | string | yes |
| `expiration_date` | date | yes |

### Guardian Delegation Authority

Credential establishing a guardian's authority to manage a delegated AID on behalf of a ward (minor, elderly, person with disabilities). The ward retains their own AID — the guardian's delegated AID operates within a cryptographically bounded scope. Revocable when the ward gains capacity.

| Property | Value |
|----------|-------|
| **ID** | `guardian_delegation` |
| **Issuer** | Individual |
| **Holder** | Guardian |
| **Verifiers** | Service Platform, Grassroots Organization, Government Agency |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `ward_aid` | string | yes |
| `guardian_aid` | string | yes |
| `scope` | string | yes |
| `authority_source` | string | yes |
| `effective_date` | date | yes |
| `expiration_date` | date | no |
| `revocation_conditions` | string | yes |

### Service Attribution Agreement

Contractual credential between platforms defining how service hours are attributed when volunteers cross organizational boundaries. Based on the social graph of contribution — who published the need, who matched the volunteer, where credentials came from, who verified completion.

| Property | Value |
|----------|-------|
| **ID** | `attribution_agreement` |
| **Issuer** | Service Platform |
| **Holder** | Service Platform |
| **Verifiers** | Service Platform, Government Agency |
| **Disclosure** | full |
| **Chained From** | Interoperability Agreement (`interop_agreement`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `partner_platform_aid` | string | yes |
| `attribution_model` | string | yes |
| `attribution_factors` | string[] | yes |
| `reporting_method` | string | yes |
| `effective_date` | date | yes |

---

*[Back to Humanitarian Service Marketplace overview](/ecosystems/humanitarian-service-marketplace/)*

*Generated from [`docs/humanitarian-service-marketplace/ecosystem.yaml`](https://github.com/seriouscoderone/kerihost/blob/main/docs/humanitarian-service-marketplace/ecosystem.yaml) by the `/ecosystem-package` skill.*
