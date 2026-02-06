# Anti-Corruption & Duplicity Case Studies

**Type:** Case study collection

**Priority:** 🟢 Phase 1 (Months 3–6)

---

## Purpose

Show where KERI becomes unavoidable — corruption detection.

## Corruption Scenarios

### Permit Bribery
**Scenario:** Official approves permits for payment under the table.

**Detection Mechanisms:**
- Approval patterns outside normal parameters
- Missing prerequisite verifications
- Timing anomalies (too fast, off-hours)
- Duplicity: different records shown to different parties

### Procurement Kickbacks
**Scenario:** Purchasing agent steers contracts to favored vendor.

**Detection Mechanisms:**
- Bid evaluation inconsistencies
- Missing competitive bids
- Price anomalies vs. market
- Relationship patterns (always same vendor)

### Vendor Favoritism
**Scenario:** Inspector passes unqualified work for specific contractors.

**Detection Mechanisms:**
- Pass rate anomalies by inspector-contractor pair
- Inspection duration outliers
- Missing documentation accepted
- Re-inspection patterns

### Credential Forgery
**Scenario:** Someone presents fake qualifications.

**Detection Mechanisms:**
- Credential chain doesn't verify
- Issuer denies issuance
- Schema mismatch
- Duplicity in claimed history

### Identity Impersonation
**Scenario:** Someone acts as another party.

**Detection Mechanisms:**
- Signature doesn't match claimed AID
- Key state inconsistent
- Witness receipts missing or forged
- Duplicity detected by watchers

## KERI-Native Detection Mechanisms

### Duplicity Proofs
- Same AID signing contradictory statements
- Fork in key event log
- Mathematically provable misconduct

### Inconsistent Logs
- Events that don't chain properly
- Missing sequence numbers
- Timestamp impossibilities

### Missing Receipts
- Claims of witness confirmation without receipts
- Selective receipt presentation
- Receipt forgery detection

### Illegitimate Delegation Chains
- Delegation from revoked AID
- Out-of-scope actions
- Broken chain of authority

## For Each Scenario

Show:
1. **How corruption becomes detectable** — What evidence exists
2. **How honest actors are protected** — False accusations prevented
3. **How blame is precise** — Individual accountability, not collective punishment

---

*TODO: Write detailed case studies*
