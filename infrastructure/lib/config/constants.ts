/**
 * Shared constants for Kerihost infrastructure
 */

export const TABLE_NAMES = {
  KEL: "kerihost-kel",
  STATES: "kerihost-states",
  RECEIPTS: "kerihost-receipts",
  ESCROWS: "kerihost-escrows",
} as const;

export const SECRET_NAMES = {
  WITNESS_SEED: "kerihost/witness-seed",
} as const;

export const LAMBDA_DEFAULTS = {
  MEMORY_SIZE: 256,
  TIMEOUT_SECONDS: 30,
} as const;

export const API_DEFAULTS = {
  THROTTLE_RATE_LIMIT: 1000,
  THROTTLE_BURST_LIMIT: 2000,
} as const;
