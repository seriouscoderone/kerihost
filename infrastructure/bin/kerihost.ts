#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { DataStack } from "../lib/stacks/data-stack";
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
  region: process.env.CDK_DEFAULT_REGION || "us-east-1",
};

// =======================================================================
// Stack Composition
// =======================================================================

// Layer 1: Data (DynamoDB tables, secrets)
const dataStack = new DataStack(app, "KerihostDataStack", {
  env,
  description: "KERI Host Data Layer - DynamoDB tables and secrets",
});

// Layer 2: Witness Service (API Gateway, Lambdas, routes, EventBridge)
const witnessStack = new WitnessStack(app, "KerihostWitnessStack", {
  env,
  tables: dataStack.tables,
  witnessSeed: dataStack.witnessSeed,
  domainName,
  hostedZoneId,
  basePath: "witness",
  description: "KERI Host Witness Service - API Gateway, Lambdas, and routes",
});

// =======================================================================
// Stack Dependencies
// =======================================================================

witnessStack.addDependency(dataStack);
