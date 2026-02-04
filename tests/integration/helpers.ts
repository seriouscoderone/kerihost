/**
 * Test helpers for KERI witness integration tests
 *
 * Uses KERI standard field names:
 * - t: event type (icp, rot, ixn, dip, drt)
 * - i: identifier prefix (AID)
 * - s: sequence number (hex string)
 * - p: prior event digest
 * - k: signing keys array
 * - kt: signing threshold
 * - n: next key digest (commitment)
 * - b: witnesses array
 * - bt: witness threshold
 * - a: anchors/seals
 * - d: event digest (SAID)
 */

import * as crypto from "crypto";

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

/**
 * Generate a random prefix (AID-like identifier)
 */
export function randomPrefix(): string {
  // Generate 32 random bytes and encode as qualified base64
  // D prefix = Ed25519 public key (but we're faking it for tests)
  const bytes = crypto.randomBytes(32);
  const b64 = bytes.toString("base64url").replace(/=/g, "");
  return `D${b64.substring(0, 43)}`; // 44 chars total
}

/**
 * Generate a random digest
 */
export function randomDigest(): string {
  // E prefix = Blake3-256 digest
  const bytes = crypto.randomBytes(32);
  const b64 = bytes.toString("base64url").replace(/=/g, "");
  return `E${b64.substring(0, 43)}`;
}

/**
 * Generate a random signing key
 */
export function randomSigningKey(): string {
  return randomPrefix(); // Same format as prefix
}

/**
 * Generate a random signature (placeholder)
 */
export function randomSignature(): string {
  // AA prefix = Ed25519 signature
  const bytes = crypto.randomBytes(64);
  const b64 = bytes.toString("base64url").replace(/=/g, "");
  return `AA${b64.substring(0, 86)}`; // 88 chars total
}

/**
 * Convert number to hex string for sequence number
 */
function snToHex(sn: number): string {
  return sn.toString(16);
}

/**
 * Create a KERI-compatible inception event
 */
export function createTestInceptionEvent(prefix?: string): {
  t: string;
  i: string;
  s: string;
  k: string[];
  kt: string;
  n: string[];
  bt: string;
  b: string[];
  a: any[];
  d: string;
  // Keep these for test tracking
  _prefix: string;
  _sn: number;
  _digest: string;
} {
  const p = prefix || randomPrefix();
  const signingKey = randomSigningKey();
  const nextDigest = randomDigest();
  const eventDigest = randomDigest();

  return {
    t: "icp",
    i: p,
    s: "0",
    k: [signingKey],
    kt: "1",
    n: [nextDigest],
    bt: "0",
    b: [],
    a: [],
    d: eventDigest,
    // Helper fields for tests
    _prefix: p,
    _sn: 0,
    _digest: eventDigest,
  };
}

/**
 * Create a KERI-compatible interaction event
 */
export function createTestInteractionEvent(
  prefix: string,
  sn: number,
  priorDigest: string
): {
  t: string;
  i: string;
  s: string;
  p: string;
  a: any[];
  d: string;
  // Keep these for test tracking
  _prefix: string;
  _sn: number;
  _digest: string;
} {
  const eventDigest = randomDigest();

  return {
    t: "ixn",
    i: prefix,
    s: snToHex(sn),
    p: priorDigest,
    a: [],
    d: eventDigest,
    // Helper fields for tests
    _prefix: prefix,
    _sn: sn,
    _digest: eventDigest,
  };
}

/**
 * Create a signed event wrapper for submission to the witness
 *
 * The Lambda expects the raw event JSON as the body, with signatures attached
 */
export function createSignedEvent(event: object): object {
  // The Lambda's process handler expects raw bytes which are parsed as JSON
  // The SignedEvent::from_cesr expects: { event fields at root level }
  // But we also need to include signatures
  // Looking at the processor code, it seems to parse the event directly
  //
  // For now, let's send the event as-is (the Lambda parses raw bytes as JSON)
  // The signature would be attached separately in real CESR, but for this
  // JSON-based test, we can include it in the body
  return event;
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
 * Submit an event to the witness
 */
export async function submitEvent(event: object): Promise<{
  status: number;
  data: any;
}> {
  // Send the event as raw JSON bytes to /process
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
