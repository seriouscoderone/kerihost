#!/usr/bin/env node
import "source-map-support/register";
import * as cdk from "aws-cdk-lib";
import { KerihostStack } from "../lib/kerihost-stack";

const app = new cdk.App();

new KerihostStack(app, "KerihostStack", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: process.env.CDK_DEFAULT_REGION || "us-east-1",
  },
  description: "KERI Witness Infrastructure - keri.host",
});
