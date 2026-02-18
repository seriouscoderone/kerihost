/**
 * Integration tests for Kerihost Witness Service
 *
 * These tests run against the deployed API Gateway endpoint.
 * They use signify-ts to construct real CESR events with valid SAIDs
 * and Ed25519 signatures.
 *
 * Run with:
 *   WITNESS_API_URL=https://api.keri.host/witness pnpm test
 */

import { describe, it, expect, beforeAll, beforeEach } from "vitest";
import {
  API_URL,
  requireApiUrl,
  randomPrefix,
  randomDigest,
  initCrypto,
  getWitnessPrefix,
  createRealInceptionEvent,
  createRealInteractionEvent,
  submitEvent,
  queryWitness,
  getWitnessIntroduction,
  resolveOobi,
} from "./helpers";

// Skip all tests if no API URL configured
const describeWithApi = API_URL ? describe : describe.skip;

describeWithApi("Witness API Gateway Integration Tests", () => {
  let apiUrl: string;
  let wits: string[]; // witness identifiers for inception events

  beforeAll(async () => {
    await initCrypto();
    apiUrl = requireApiUrl();
    const witnessPrefix = await getWitnessPrefix();
    wits = [witnessPrefix];
    console.log(`Testing against API: ${apiUrl} (witness: ${witnessPrefix})`);
  });

  describe("GET /introduce - Witness Introduction", () => {
    it("should return witness OOBI", async () => {
      const { status, data } = await getWitnessIntroduction();

      expect(status).toBe(200);
      expect(data.oobi).toBeDefined();
      expect(data.witness).toBeDefined();
      expect(data.asOf).toBeDefined();

      // OOBI should be a URL
      expect(data.oobi).toContain("http");
    });

    it("should include timestamp in response", async () => {
      const { data } = await getWitnessIntroduction();

      expect(data.asOf).toBeDefined();
      // Should be a valid ISO timestamp
      const timestamp = new Date(data.asOf);
      expect(timestamp.getTime()).not.toBeNaN();
    });
  });

  describe("POST /process - Event Processing", () => {
    describe("Inception Events", () => {
      it("should accept valid inception event with KERI-honest response", async () => {
        const { cesr, prefix } = await createRealInceptionEvent(wits);

        const { status, data } = await submitEvent(cesr);

        expect(status).toBe(200);
        expect(data.status).toBe("accepted");

        // KERI-honest response fields MUST be present
        expect(data.asOf).toBeDefined();

        // State should be returned
        if (data.state) {
          expect(data.state.prefix).toBe(prefix);
          expect(data.state.sn).toBe(0);
        }
      });

      it("should return duplicate for same inception submitted twice", async () => {
        const { cesr } = await createRealInceptionEvent(wits);

        // First submission
        const first = await submitEvent(cesr);
        expect(first.status).toBe(200);
        expect(first.data.status).toBe("accepted");

        // Second submission - should be duplicate
        const second = await submitEvent(cesr);
        expect(second.status).toBe(200);
        expect(second.data.status).toBe("duplicate");
        expect(second.data.asOf).toBeDefined();
      });

      it("should reject malformed event", async () => {
        const { status, data } = await submitEvent({
          // Missing required fields
          t: "icp",
          i: "bad",
        });

        expect(status).toBe(400);
        expect(data.error).toBeDefined();
        expect(data.asOf).toBeDefined();
      });
    });

    describe("Out-of-Order Events", () => {
      it("should escrow out-of-order event with KERI-honest response", async () => {
        // Create inception and submit it first
        const { cesr: icpCesr, prefix, digest, signer } =
          await createRealInceptionEvent(wits);
        await submitEvent(icpCesr);

        // Submit interaction at sn=5 (skipping sn=1-4) — out of order
        const { cesr: ixnCesr } = await createRealInteractionEvent(
          prefix,
          5,
          digest,
          signer
        );

        const { status, data } = await submitEvent(ixnCesr);

        // HTTP 202 = escrowed (not rejected)
        expect(status).toBe(202);
        expect(data.status).toBe("escrowed");
        expect(data.reason).toBe("out_of_order");

        // KERI-honest: escrow is state, not error
        expect(data.asOf).toBeDefined();
      });
    });

    describe("Sequential Events", () => {
      it("should accept inception then interaction in sequence", async () => {
        // Create and submit inception
        const { cesr: icpCesr, prefix, digest, signer } =
          await createRealInceptionEvent(wits);

        const icpResult = await submitEvent(icpCesr);
        expect(icpResult.status).toBe(200);
        expect(icpResult.data.status).toBe("accepted");

        // Create and submit interaction at sn=1
        const { cesr: ixnCesr } = await createRealInteractionEvent(
          prefix,
          1,
          digest,
          signer
        );

        const ixnResult = await submitEvent(ixnCesr);
        expect(ixnResult.status).toBe(200);
        expect(ixnResult.data.status).toBe("accepted");

        // State should show sn=1
        if (ixnResult.data.state) {
          expect(ixnResult.data.state.sn).toBe(1);
        }
      });
    });
  });

  describe("POST /query - State Queries", () => {
    let testPrefix: string;
    let testDigest: string;

    beforeEach(async () => {
      // Create a real test identifier
      const { cesr, prefix, digest } = await createRealInceptionEvent(wits);
      testPrefix = prefix;
      testDigest = digest;
      await submitEvent(cesr);
    });

    describe("State Query", () => {
      it("should return key state for known identifier", async () => {
        const { status, data } = await queryWitness("state", {
          prefix: testPrefix,
        });

        expect(status).toBe(200);
        expect(data.state).toBeDefined();
        expect(data.state.prefix).toBe(testPrefix);
        expect(data.state.sn).toBe(0);
        expect(data.asOf).toBeDefined();
      });

      it("should return 404 for unknown identifier", async () => {
        const unknownPrefix = randomPrefix();

        const { status, data } = await queryWitness("state", {
          prefix: unknownPrefix,
        });

        expect(status).toBe(404);
        expect(data.error).toBeDefined();
        expect(data.asOf).toBeDefined();
      });

      it("should require prefix parameter", async () => {
        const { status, data } = await queryWitness("state", {});

        expect(status).toBe(400);
        expect(data.error).toContain("prefix");
      });
    });

    describe("KEL Query", () => {
      it("should return events for known identifier", async () => {
        const { status, data } = await queryWitness("kel", {
          prefix: testPrefix,
          start_sn: 0,
        });

        expect(status).toBe(200);
        expect(data.events).toBeDefined();
        expect(Array.isArray(data.events)).toBe(true);
        expect(data.count).toBeGreaterThanOrEqual(0);
        expect(data.asOf).toBeDefined();
      });

      it("should return empty array for unknown identifier", async () => {
        const unknownPrefix = randomPrefix();

        const { status, data } = await queryWitness("kel", {
          prefix: unknownPrefix,
          start_sn: 0,
        });

        expect(status).toBe(200);
        expect(data.events).toEqual([]);
        expect(data.count).toBe(0);
      });

      it("should support range queries", async () => {
        const { status, data } = await queryWitness("kel", {
          prefix: testPrefix,
          start_sn: 0,
          end_sn: 10,
        });

        expect(status).toBe(200);
        expect(data.events).toBeDefined();
      });
    });

    describe("Receipts Query", () => {
      it("should return receipts for event digest", async () => {
        const { status, data } = await queryWitness("receipts", {
          event_digest: testDigest,
        });

        expect(status).toBe(200);
        expect(data.receipts).toBeDefined();
        expect(Array.isArray(data.receipts)).toBe(true);
        expect(data.count).toBeGreaterThanOrEqual(0);
        expect(data.asOf).toBeDefined();
      });

      it("should return empty array for unknown digest", async () => {
        const { status, data } = await queryWitness("receipts", {
          event_digest: randomDigest(),
        });

        expect(status).toBe(200);
        expect(data.receipts).toEqual([]);
        expect(data.count).toBe(0);
      });

      it("should require event_digest parameter", async () => {
        const { status, data } = await queryWitness("receipts", {});

        expect(status).toBe(400);
        expect(data.error).toContain("event_digest");
      });
    });

    describe("Invalid Query Type", () => {
      it("should reject unknown query type", async () => {
        const { status, data } = await queryWitness("invalid_type", {});

        expect(status).toBe(400);
        expect(data.error).toContain("Unknown query type");
      });
    });
  });

  describe("GET /oobi/{id} - OOBI Resolution", () => {
    let testPrefix: string;

    beforeEach(async () => {
      // Create a real test identifier
      const { cesr, prefix } = await createRealInceptionEvent(wits);
      testPrefix = prefix;
      await submitEvent(cesr);
    });

    it("should resolve OOBI for known identifier", async () => {
      const { status, data } = await resolveOobi(testPrefix);

      expect(status).toBe(200);
      expect(data.state).toBeDefined();
      expect(data.oobi).toBeDefined();
      expect(data.asOf).toBeDefined();
    });

    it("should return 404 for unknown identifier", async () => {
      const unknownPrefix = randomPrefix();

      const { status, data } = await resolveOobi(unknownPrefix);

      expect(status).toBe(404);
      expect(data.error).toBeDefined();
      expect(data.asOf).toBeDefined();
    });
  });

  describe("KERI-Honest Design Verification", () => {
    it("should never return confidence level of FINAL", async () => {
      const { cesr } = await createRealInceptionEvent(wits);

      const { data } = await submitEvent(cesr);

      // Should have confidence field
      if (data.confidence) {
        // KERI-honest: never claim FINAL
        expect(data.confidence).not.toBe("FINAL");
        expect(data.confidence).toMatch(/LOCAL_ONLY|RECEIPT_THRESHOLD_MET/);
      }
    });

    it("should always include asOf timestamp", async () => {
      // Test /introduce
      const intro = await getWitnessIntroduction();
      expect(intro.data.asOf).toBeDefined();

      // Test /process
      const { cesr, prefix } = await createRealInceptionEvent(wits);
      const process = await submitEvent(cesr);
      expect(process.data.asOf).toBeDefined();

      // Test /query
      const query = await queryWitness("state", { prefix });
      expect(query.data.asOf).toBeDefined();

      // Test /oobi
      const oobi = await resolveOobi(prefix);
      expect(oobi.data.asOf).toBeDefined();
    });

    it("should treat escrow as state, not error", async () => {
      // Create inception and submit it first
      const { cesr: icpCesr, prefix, digest, signer } =
        await createRealInceptionEvent(wits);
      await submitEvent(icpCesr);

      // Submit interaction at sn=5 (out of order)
      const { cesr: ixnCesr } = await createRealInteractionEvent(
        prefix,
        5,
        digest,
        signer
      );

      const { status, data } = await submitEvent(ixnCesr);

      // HTTP 202 indicates successful escrow (not 4xx error)
      expect(status).toBe(202);
      expect(data.status).toBe("escrowed");
      // Escrow reason should be informative
      expect(data.reason).toBeDefined();
    });
  });

  describe("Error Handling", () => {
    it("should return proper error for missing body on /process", async () => {
      const response = await fetch(`${apiUrl}/process`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
      });

      const data = await response.json();

      expect(response.status).toBe(400);
      expect(data.error).toBeDefined();
      expect(data.asOf).toBeDefined();
    });

    it("should return proper error for missing body on /query", async () => {
      const response = await fetch(`${apiUrl}/query`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
      });

      const data = await response.json();

      expect(response.status).toBe(400);
      expect(data.error).toBeDefined();
      expect(data.asOf).toBeDefined();
    });
  });
});

// Unit tests that don't require the API
describe("Test Helpers (Unit Tests)", () => {
  beforeAll(async () => {
    await initCrypto();
  });

  it("should generate valid prefix format", () => {
    const prefix = randomPrefix();
    expect(prefix).toMatch(/^D[A-Za-z0-9_-]{43}$/);
  });

  it("should generate valid digest format", () => {
    const digest = randomDigest();
    expect(digest).toMatch(/^E[A-Za-z0-9_-]{43}$/);
  });

  it("should create real inception event with valid CESR", async () => {
    const { cesr, prefix, digest, signer, serder } =
      await createRealInceptionEvent();

    // CESR bytes should be non-empty
    expect(cesr.length).toBeGreaterThan(0);

    // Prefix should start with D (Ed25519 transferable)
    expect(prefix).toMatch(/^D/);

    // Digest should start with E (Blake3-256)
    expect(digest).toMatch(/^E/);

    // Serder should have matching fields
    expect(serder.pre).toBe(prefix);
    expect(serder.ked.t).toBe("icp");
    expect(serder.ked.s).toBe("0");
  });

  it("should create real interaction event with valid CESR", async () => {
    const { prefix, digest, signer } = await createRealInceptionEvent();
    const { cesr, digest: ixnDigest, serder } =
      await createRealInteractionEvent(prefix, 1, digest, signer);

    expect(cesr.length).toBeGreaterThan(0);
    expect(ixnDigest).toMatch(/^E/);
    expect(serder.ked.t).toBe("ixn");
    expect(serder.pre).toBe(prefix);
  });
});
