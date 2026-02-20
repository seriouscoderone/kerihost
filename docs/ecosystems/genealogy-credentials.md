---
title: "Genealogy Ecosystem — Credentials"
permalink: /ecosystems/genealogy/credentials/
layout: single
author_profile: true
toc: true
toc_sticky: true
description: "Complete credential catalog for the Genealogy Ecosystem."
---

This is the complete credential catalog for the Genealogy Ecosystem. Each credential below includes its issuer, holder, verifiers, schema fields, disclosure mode, and chaining relationships. Credentials and schemas are illustrative examples showing the type of data each credential would carry, not finalized specifications.

[Back to Genealogy Ecosystem overview](/ecosystems/genealogy/)

### Record Custody Credential

Attests that an archive is the custodian of a specified collection of physical records. The root credential in the provenance chain — everything downstream traces back to this.

| Property | Value |
|----------|-------|
| **ID** | `record_custody` |
| **Issuer** | Archive |
| **Holder** | Archive |
| **Verifiers** | Digitizer, Record Host, Researcher, Individual |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `collection_name` | string | yes |
| `collection_description` | string | yes |
| `geographic_scope` | string | yes |
| `date_range_start` | string | no |
| `date_range_end` | string | no |
| `record_types` | string[] | yes |
| `access_policy` | string | yes |

### Digitization Authorization

Authorizes a digitizer to create digital copies of records in a specified collection. Chains from record custody. Defines terms of use and copyright constraints — the archive controls how its records are digitized and redistributed.

| Property | Value |
|----------|-------|
| **ID** | `digitization_authorization` |
| **Issuer** | Archive |
| **Holder** | Digitizer |
| **Verifiers** | Record Attestor, Record Host, Researcher |
| **Disclosure** | full |
| **Chained From** | Record Custody Credential (`record_custody`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `collection_ref` | SAID | yes |
| `authorized_scope` | string | yes |
| `copyright_terms` | string | yes |
| `expiration_date` | date | no |
| `redistribution_allowed` | boolean | yes |

### Digitization Record

Attests that a digital copy was created from a physical source object. References the source object and capture conditions. May or may not chain from a digitization authorization — volunteer grave photos don't require archive authorization.

| Property | Value |
|----------|-------|
| **ID** | `digitization_record` |
| **Issuer** | Digitizer |
| **Holder** | Digitizer |
| **Verifiers** | Researcher, Individual, Record Attestor, Algorithm Service |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `source_object_description` | string | yes |
| `source_location` | string | yes |
| `capture_method` | string | yes |
| `capture_date` | date | yes |
| `resolution` | string | no |
| `content_said` | SAID | yes |
| `authorization_ref` | SAID | no |
| `freeform_notes` | string | no |

### Record Attestation

Verifies the fidelity of a digital copy against its physical original. Issued by a record attestor who has compared the digital and physical versions. Adds an independent verification layer to the provenance chain.

| Property | Value |
|----------|-------|
| **ID** | `record_attestation` |
| **Issuer** | Record Attestor |
| **Holder** | Digitizer |
| **Verifiers** | Researcher, Individual, Record Host |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `digitization_ref` | SAID | yes |
| `attestation_method` | string | yes |
| `attestation_date` | date | yes |
| `fidelity_assessment` | string | yes |
| `notes` | string | no |

### Index Entry Credential

A transcription or tagging of a digitized record. Links searchable text to the digitization record it was derived from. Discloses transcription method so you know whether a human, an AI, or both produced the index.

| Property | Value |
|----------|-------|
| **ID** | `index_entry` |
| **Issuer** | Indexer |
| **Holder** | Indexer |
| **Verifiers** | Researcher, Individual, Algorithm Service |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `digitization_ref` | SAID | yes |
| `transcription_method` | string | yes |
| `schema_ref` | SAID | yes |
| `structured_fields` | object | yes |
| `freeform_content` | string | no |
| `confidence_notes` | string | no |

### Record Hosting Agreement

Authorizes a record host to store and serve digital copies from a specified collection. Defines access controls, copyright restrictions, and terms of service. Chains from record custody — you can only host what you're authorized to host.

| Property | Value |
|----------|-------|
| **ID** | `hosting_agreement` |
| **Issuer** | Archive |
| **Holder** | Record Host |
| **Verifiers** | Researcher, Individual |
| **Disclosure** | full |
| **Chained From** | Record Custody Credential (`record_custody`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `collection_ref` | SAID | yes |
| `hosting_terms` | string | yes |
| `access_controls` | string | yes |
| `copyright_constraints` | string | yes |
| `expiration_date` | date | no |

### Research Claim

A signed assertion about a person, event, date, place, or relationship. References supporting evidence. The fundamental unit of genealogical research in this ecosystem — every conclusion is a credential with a proof chain.

| Property | Value |
|----------|-------|
| **ID** | `research_claim` |
| **Issuer** | Researcher |
| **Holder** | Researcher |
| **Verifiers** | Individual, Researcher, Algorithm Service |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `claim_type` | string | yes |
| `schema_ref` | SAID | yes |
| `subject_person_ref` | SAID | yes |
| `claim_data` | object | yes |
| `evidence_refs` | SAID[] | yes |
| `methodology_notes` | string | no |
| `confidence_assessment` | string | no |

### Identity Resolution Claim

Asserts that two or more records refer to the same person. The decentralized replacement for platform deduplication. Others can endorse or dispute this link. Never destructive — records stay separate, the link is an overlay.

| Property | Value |
|----------|-------|
| **ID** | `identity_resolution` |
| **Issuer** | Researcher |
| **Holder** | Researcher |
| **Verifiers** | Individual, Researcher, Algorithm Service |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `person_refs` | SAID[] | yes |
| `basis` | string | yes |
| `evidence_refs` | SAID[] | yes |
| `confidence_assessment` | string | no |

### Source Object Link

Asserts that multiple digitizations depict the same physical source object — three photos of the same gravestone, for example. Creates a virtual node representing the source object. Multiple digitizations stay separate but are linked.

| Property | Value |
|----------|-------|
| **ID** | `source_link` |
| **Issuer** | Researcher |
| **Holder** | Researcher |
| **Verifiers** | Individual, Researcher, Digitizer |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `digitization_refs` | SAID[] | yes |
| `source_object_description` | string | yes |
| `source_location` | string | no |
| `basis` | string | yes |

### Professional Genealogist Certification

Attests that a researcher holds a professional genealogy certification or degree. Authority is organic and reputation-based — the ecosystem doesn't anoint any single certification body. Selective disclosure allows proving certification without revealing personal details.

| Property | Value |
|----------|-------|
| **ID** | `professional_certification` |
| **Issuer** | Certification Body |
| **Holder** | Researcher |
| **Verifiers** | Individual, Researcher |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `certification_type` | string | yes |
| `issuing_body_name` | string | yes |
| `specializations` | string[] | no |
| `issue_date` | date | yes |
| `expiration_date` | date | no |
| `certificate_number` | string | yes |

### Program Accreditation

Accredits a genealogy education program or sub-organization. Enables delegation — an accredited program can issue its own certifications that chain from this accreditation.

| Property | Value |
|----------|-------|
| **ID** | `program_accreditation` |
| **Issuer** | Certification Body |
| **Holder** | Certification Body |
| **Verifiers** | Researcher, Individual |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `program_name` | string | yes |
| `institution` | string | yes |
| `accreditation_scope` | string | yes |
| `issue_date` | date | yes |
| `review_date` | date | no |

### DNA Result Credential

Raw DNA analysis results issued to the individual, not retained by the service. Selective disclosure allows proving specific genetic facts without revealing the full profile. Your DNA data is yours.

| Property | Value |
|----------|-------|
| **ID** | `dna_result` |
| **Issuer** | DNA Service |
| **Holder** | Individual |
| **Verifiers** | Researcher |
| **Disclosure** | selective |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `test_type` | string | yes |
| `sample_date` | date | yes |
| `processing_date` | date | yes |
| `result_data_said` | SAID | yes |
| `methodology` | string | yes |
| `lab_identifier` | string | yes |

### Ethnicity Estimate

Interpretation of DNA results as ethnicity/ancestry composition. Issued to the individual with selective disclosure. Estimates change as reference populations grow, so versioning matters — last year's estimate may differ from this year's.

| Property | Value |
|----------|-------|
| **ID** | `ethnicity_estimate` |
| **Issuer** | DNA Service |
| **Holder** | Individual |
| **Verifiers** | Researcher |
| **Disclosure** | selective |
| **Chained From** | DNA Result Credential (`dna_result`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `dna_result_ref` | SAID | yes |
| `estimate_version` | string | yes |
| `regions` | object[] | yes |
| `reference_population_size` | integer | no |
| `confidence_intervals` | object | no |

### Kinship Match

Asserts a genetic relationship between two individuals based on shared DNA segments. Issued to both matched individuals with selective disclosure. Enables biological family discovery while protecting privacy.

| Property | Value |
|----------|-------|
| **ID** | `kinship_match` |
| **Issuer** | DNA Service |
| **Holder** | Individual |
| **Verifiers** | Individual |
| **Disclosure** | selective |
| **Chained From** | DNA Result Credential (`dna_result`) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `matched_individual_ref` | SAID | yes |
| `shared_cm` | number | yes |
| `shared_segments` | integer | yes |
| `estimated_relationship` | string | yes |
| `confidence_level` | string | yes |

### Dispute Claim

A signed disagreement with another claim. States the basis for disagreement and optionally provides counter-evidence. Visible in the public claim graph — disputes are data, not drama.

| Property | Value |
|----------|-------|
| **ID** | `dispute_claim` |
| **Issuer** | Researcher |
| **Holder** | Researcher |
| **Verifiers** | Individual, Researcher, Algorithm Service |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `disputed_credential_ref` | SAID | yes |
| `dispute_basis` | string | yes |
| `counter_evidence_refs` | SAID[] | no |
| `proposed_correction` | string | no |

### Endorsement Claim

A signed agreement with another claim. Strengthens the corroboration signal for algorithm services. May optionally add supporting evidence beyond what the original claim referenced.

| Property | Value |
|----------|-------|
| **ID** | `endorsement` |
| **Issuer** | Researcher |
| **Holder** | Researcher |
| **Verifiers** | Individual, Researcher, Algorithm Service |
| **Disclosure** | full |
| **Chained From** | None (root credential) |

**Schema Fields:**

| Field | Type | Required |
|-------|------|----------|
| `endorsed_credential_ref` | SAID | yes |
| `endorsement_basis` | string | no |
| `additional_evidence_refs` | SAID[] | no |

---

*[Back to Genealogy Ecosystem overview](/ecosystems/genealogy/)*

*Generated from [`docs/genealogy/ecosystem.yaml`](https://github.com/seriouscoderone/kerihost/blob/main/docs/genealogy/ecosystem.yaml) by the `/ecosystem-package` skill.*
