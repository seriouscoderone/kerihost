/**
 * Shared constants for Kerihost infrastructure
 *
 * Resource names are derived from stack names at runtime.
 * These slugs are appended to the stack name: {StackName}-{slug}
 */

/**
 * Slugs for DynamoDB table names (appended to stack name)
 */
export const TABLE_SLUGS = {
  KEL: "kel",
  STATES: "states",
  RECEIPTS: "receipts",
  ESCROWS: "escrows",
} as const;

/**
 * Slugs for Lambda function names (appended to stack name)
 */
export const LAMBDA_SLUGS = {
  PROCESS: "process",
  QUERY: "query",
  OOBI: "oobi",
  ESCROW_CHECK: "escrow-check",
} as const;

/**
 * Secret names - these are external resources created manually
 */
export const SECRET_NAMES = {
  WITNESS_SEED: "kerihost/witness-seed",
} as const;

/**
 * GSI names (slugs appended to table name)
 */
export const GSI_SLUGS = {
  ESCROWS_BY_REASON: "by-reason",
} as const;

/**
 * API Gateway defaults
 */
export const API_DEFAULTS = {
  THROTTLE_RATE_LIMIT: 1000,
  THROTTLE_BURST_LIMIT: 2000,
} as const;

/**
 * Lambda defaults
 */
export const LAMBDA_DEFAULTS = {
  MEMORY_SIZE: 256,
  TIMEOUT_SECONDS: 30,
} as const;
