#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { DataStack } from "../lib/stacks/data-stack";
import { ApiStack } from "../lib/stacks/api-stack";
import { WitnessStack } from "../lib/stacks/witness-stack";

const app = new cdk.App();

// =======================================================================
// Configuration from CDK Context
// =======================================================================

const domainName = app.node.tryGetContext("domainName") || "api.keri.host";
const hostedZoneId = app.node.tryGetContext("hostedZoneId");

if (!hostedZoneId) {
  throw new Error(
    "hostedZoneId is required. Pass via CLI: -c hostedZoneId=Z0070723WLKQKTOACN5H"
  );
}

const env = {
  account: process.env.CDK_DEFAULT_ACCOUNT,
  region: process.env.CDK_DEFAULT_REGION || "us-west-2",
};

// =======================================================================
// Stack Composition
// =======================================================================

// Layer 1: Data (DynamoDB tables, secrets)
// Protected by stack separation - never destroyed during service deployments
const dataStack = new DataStack(app, "KerihostDataStack", {
  env,
  description: "KERI Host Data Layer - DynamoDB tables and secrets",
});

// Layer 2: API (API Gateway, custom domain, certificate, DNS)
// Shared infrastructure for all services
const apiStack = new ApiStack(app, "KerihostApiStack", {
  env,
  domainName,
  hostedZoneId,
  description: "KERI Host API Layer - API Gateway and custom domain",
});

// Layer 3: Witness Service (Lambdas, routes, EventBridge)
// Attaches routes to the shared API Gateway
const basePath = "witness";
const witnessStack = new WitnessStack(app, "KerihostWitnessStack", {
  env,
  tables: dataStack.tables,
  witnessSeed: dataStack.witnessSeed,
  api: apiStack.api,
  publicUrl: `${apiStack.customDomainUrl}/${basePath}`,
  basePath,
  description: "KERI Host Witness Service - Lambdas and API routes",
});

// =======================================================================
// Stack Dependencies
// =======================================================================
// CDK automatically infers dependencies from resource references.
// Explicit dependencies are not needed and can cause cyclic references.
