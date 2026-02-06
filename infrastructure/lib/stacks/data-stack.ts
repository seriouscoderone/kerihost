import * as cdk from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { TABLE_SLUGS, GSI_SLUGS, SECRET_NAMES } from "../config/constants";

/**
 * DataStack contains all persistent data resources:
 * - DynamoDB tables for KEL, states, receipts, and escrows
 * - Reference to witness seed secret
 *
 * This stack is the foundation layer that other stacks depend on.
 * Data protection is achieved through stack separation - this stack
 * is never destroyed during normal service deployments.
 *
 * Resource names are derived from stack name: {StackName}-{slug}
 */
export class DataStack extends cdk.Stack {
  public readonly tables: {
    kel: dynamodb.Table;
    states: dynamodb.Table;
    receipts: dynamodb.Table;
    escrows: dynamodb.Table;
  };

  public readonly witnessSeed: secretsmanager.ISecret;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Helper to create resource name: {StackName}-{slug}
    const resourceName = (slug: string) => `${this.stackName}-${slug}`;

    // =======================================================================
    // DynamoDB Tables
    // =======================================================================

    // KEL (Key Event Log) Table
    // PK: aid (AID/prefix), SK: sn (zero-padded sequence number)
    const kelTable = new dynamodb.Table(this, "KelTable", {
      tableName: resourceName(TABLE_SLUGS.KEL),
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sn", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      pointInTimeRecoverySpecification: { pointInTimeRecoveryEnabled: true },
    });

    // States Table (current key state for each identifier)
    // PK: aid (AID/prefix)
    const statesTable = new dynamodb.Table(this, "StatesTable", {
      tableName: resourceName(TABLE_SLUGS.STATES),
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
    });

    // Receipts Table (witness receipts for events)
    // PK: event_digest, SK: witness_aid
    const receiptsTable = new dynamodb.Table(this, "ReceiptsTable", {
      tableName: resourceName(TABLE_SLUGS.RECEIPTS),
      partitionKey: {
        name: "event_digest",
        type: dynamodb.AttributeType.STRING,
      },
      sortKey: { name: "witness_aid", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
    });

    // Escrows Table (events waiting for conditions to be met)
    // PK: aid, SK: reason#digest
    // TTL enabled for automatic expiration
    const escrowsTable = new dynamodb.Table(this, "EscrowsTable", {
      tableName: resourceName(TABLE_SLUGS.ESCROWS),
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "reason_digest", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      timeToLiveAttribute: "ttl",
    });

    // Global Secondary Index for escrows - to query all escrows regardless of aid
    escrowsTable.addGlobalSecondaryIndex({
      indexName: `${TABLE_SLUGS.ESCROWS}-${GSI_SLUGS.ESCROWS_BY_REASON}`,
      partitionKey: { name: "reason", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "created", type: dynamodb.AttributeType.NUMBER },
      projectionType: dynamodb.ProjectionType.ALL,
    });

    // =======================================================================
    // Secrets
    // =======================================================================

    // Witness seed secret (must be created manually before deployment)
    const witnessSeedSecret = secretsmanager.Secret.fromSecretNameV2(
      this,
      "WitnessSeedSecret",
      SECRET_NAMES.WITNESS_SEED
    );

    // =======================================================================
    // Export references
    // =======================================================================

    this.tables = {
      kel: kelTable,
      states: statesTable,
      receipts: receiptsTable,
      escrows: escrowsTable,
    };

    this.witnessSeed = witnessSeedSecret;

    // =======================================================================
    // Outputs
    // =======================================================================

    new cdk.CfnOutput(this, "KelTableName", {
      value: kelTable.tableName,
      description: "DynamoDB table for Key Event Log",
      exportName: `${this.stackName}-KelTableName`,
    });

    new cdk.CfnOutput(this, "StatesTableName", {
      value: statesTable.tableName,
      description: "DynamoDB table for Key States",
      exportName: `${this.stackName}-StatesTableName`,
    });

    new cdk.CfnOutput(this, "ReceiptsTableName", {
      value: receiptsTable.tableName,
      description: "DynamoDB table for Witness Receipts",
      exportName: `${this.stackName}-ReceiptsTableName`,
    });

    new cdk.CfnOutput(this, "EscrowsTableName", {
      value: escrowsTable.tableName,
      description: "DynamoDB table for Escrowed Events",
      exportName: `${this.stackName}-EscrowsTableName`,
    });
  }
}
