/**
 * Test helpers for KERI witness integration tests
 *
 * Uses signify-ts to construct real CESR-formatted events with valid SAIDs
 * and Ed25519 signatures. The witness parses these as genuine KERI events.
 */

import * as crypto from "crypto";
import {
  ready,
  Signer,
  MtrDex,
  Diger,
  incept,
  interact,
  messagize,
  Serder,
  Siger,
  b,
} from "signify-ts";

// Get API URL from environment
export const API_URL = process.env.WITNESS_API_URL || "";

/**
 * Check if API URL is configured
 */
export function requireApiUrl(): string {
  if (!API_URL) {
    throw new Error(
      "WITNESS_API_URL environment variable is required. Deploy the stack first with `cd infrastructure && cdk deploy --outputs-file outputs.json`"
    );
  }
  return API_URL;
}

// Re-export types used by tests
export type { Signer, Serder };

/**
 * Initialize libsodium + blake3 (call once in beforeAll)
 */
let cryptoReady = false;
export async function initCrypto() {
  if (!cryptoReady) {
    await ready();
    cryptoReady = true;
  }
}

/**
 * Fetch and cache the witness identifier from /introduce.
 * The witness requires itself to be listed in the `b` (witnesses) field
 * of inception events.
 */
let cachedWitnessPrefix: string | null = null;
export async function getWitnessPrefix(): Promise<string> {
  if (cachedWitnessPrefix) return cachedWitnessPrefix;
  const { data } = await getWitnessIntroduction();
  cachedWitnessPrefix = data.witness;
  return cachedWitnessPrefix!;
}

/**
 * Generate a random prefix (AID-like identifier)
 * Used for "unknown identifier" tests where we need a fake prefix.
 */
export function randomPrefix(): string {
  const bytes = crypto.randomBytes(32);
  const b64 = bytes.toString("base64url").replace(/=/g, "");
  return `D${b64.substring(0, 43)}`;
}

/**
 * Generate a random digest
 * Used for "unknown digest" tests where we need a fake digest.
 */
export function randomDigest(): string {
  const bytes = crypto.randomBytes(32);
  const b64 = bytes.toString("base64url").replace(/=/g, "");
  return `E${b64.substring(0, 43)}`;
}

/**
 * Create a real CESR inception event using signify-ts.
 *
 * Generates an Ed25519 keypair, builds a SAID-valid inception event,
 * signs it, and serializes to CESR bytes ready for submission.
 *
 * @param witnesses - Witness identifiers to include in `b` field.
 *   The deployed witness requires itself to be listed here.
 */
export async function createRealInceptionEvent(
  witnesses: string[] = []
): Promise<{
  cesr: Uint8Array;
  prefix: string;
  digest: string;
  signer: Signer;
  serder: Serder;
}> {
  const signer = new Signer({ transferable: true });
  const nextSigner = new Signer({ transferable: true });
  const nextDiger = new Diger(
    { code: MtrDex.Blake3_256 },
    nextSigner.verfer.qb64b
  );

  const serder = incept({
    keys: [signer.verfer.qb64],
    ndigs: [nextDiger.qb64],
    isith: 1,
    nsith: 1,
    wits: witnesses,
    toad: witnesses.length > 0 ? witnesses.length : undefined,
  });

  const sig = signer.sign(b(serder.raw), 0, true) as Siger;
  const cesr = messagize(serder, [sig]);

  return {
    cesr,
    prefix: serder.pre,
    digest: serder.ked.d as string,
    signer,
    serder,
  };
}

/**
 * Create a real CESR interaction event using signify-ts.
 *
 * Builds a SAID-valid interaction event referencing a prior event,
 * signs it with the same signer, and serializes to CESR bytes.
 */
export async function createRealInteractionEvent(
  prefix: string,
  sn: number,
  priorDigest: string,
  signer: Signer
): Promise<{
  cesr: Uint8Array;
  digest: string;
  serder: Serder;
}> {
  const serder = interact({
    pre: prefix,
    dig: priorDigest,
    sn,
    data: [],
    version: undefined,
    kind: undefined,
  });

  const sig = signer.sign(b(serder.raw), 0, true) as Siger;
  const cesr = messagize(serder, [sig]);

  return {
    cesr,
    digest: serder.ked.d as string,
    serder,
  };
}

/**
 * Make a request to the witness API
 */
export async function witnessRequest(
  endpoint: string,
  method: string = "GET",
  body?: object | string
): Promise<{ status: number; data: any }> {
  const url = `${requireApiUrl()}${endpoint}`;

  const options: RequestInit = {
    method,
    headers: {
      "Content-Type": "application/json",
    },
  };

  if (body) {
    options.body = typeof body === "string" ? body : JSON.stringify(body);
  }

  const response = await fetch(url, options);

  let data;
  try {
    data = await response.json();
  } catch {
    data = { error: "Failed to parse response" };
  }

  return {
    status: response.status,
    data,
  };
}

/**
 * Submit an event to the witness.
 * Accepts raw CESR bytes (Uint8Array) or a plain object (for malformed-event tests).
 */
export async function submitEvent(
  event: Uint8Array | object
): Promise<{ status: number; data: any }> {
  if (event instanceof Uint8Array) {
    const url = `${requireApiUrl()}/process`;
    const body = new TextDecoder().decode(event);
    const response = await fetch(url, {
      method: "POST",
      body,
    });

    let data;
    try {
      data = await response.json();
    } catch {
      data = { error: "Failed to parse response" };
    }
    return { status: response.status, data };
  }

  // Legacy path: send JSON object (for malformed-event tests)
  return witnessRequest("/process", "POST", event);
}

/**
 * Query the witness
 */
export async function queryWitness(
  queryType: string,
  params: object
): Promise<{
  status: number;
  data: any;
}> {
  return witnessRequest("/query", "POST", {
    query_type: queryType,
    ...params,
  });
}

/**
 * Get witness introduction (OOBI)
 */
export async function getWitnessIntroduction(): Promise<{
  status: number;
  data: any;
}> {
  return witnessRequest("/introduce");
}

/**
 * Resolve OOBI for an identifier
 */
export async function resolveOobi(prefix: string): Promise<{
  status: number;
  data: any;
}> {
  return witnessRequest(`/oobi/${prefix}`);
}

/**
 * Wait for a condition with timeout
 */
export async function waitFor(
  condition: () => Promise<boolean>,
  timeoutMs: number = 10000,
  intervalMs: number = 500
): Promise<boolean> {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    if (await condition()) {
      return true;
    }
    await new Promise((resolve) => setTimeout(resolve, intervalMs));
  }
  return false;
}
