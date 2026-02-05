import * as cdk from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { TABLE_NAMES, SECRET_NAMES } from "../config/constants";

/**
 * DataStack contains all persistent data resources:
 * - DynamoDB tables for KEL, states, receipts, and escrows
 * - Reference to witness seed secret
 *
 * This stack is the foundation layer that other stacks depend on.
 * Tables use RETAIN policy to prevent accidental data loss.
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

    // =======================================================================
    // DynamoDB Tables
    // =======================================================================

    // KEL (Key Event Log) Table
    // PK: aid (AID/prefix), SK: sn (zero-padded sequence number)
    const kelTable = new dynamodb.Table(this, "KelTable", {
      tableName: TABLE_NAMES.KEL,
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sn", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      pointInTimeRecoverySpecification: { pointInTimeRecoveryEnabled: true },
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    // States Table (current key state for each identifier)
    // PK: aid (AID/prefix)
    const statesTable = new dynamodb.Table(this, "StatesTable", {
      tableName: TABLE_NAMES.STATES,
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    // Receipts Table (witness receipts for events)
    // PK: event_digest, SK: witness_aid
    const receiptsTable = new dynamodb.Table(this, "ReceiptsTable", {
      tableName: TABLE_NAMES.RECEIPTS,
      partitionKey: {
        name: "event_digest",
        type: dynamodb.AttributeType.STRING,
      },
      sortKey: { name: "witness_aid", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    // Escrows Table (events waiting for conditions to be met)
    // PK: aid, SK: reason#digest
    // TTL enabled for automatic expiration
    const escrowsTable = new dynamodb.Table(this, "EscrowsTable", {
      tableName: TABLE_NAMES.ESCROWS,
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "reason_digest", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      timeToLiveAttribute: "ttl",
      removalPolicy: cdk.RemovalPolicy.DESTROY, // Escrows can be destroyed
    });

    // Global Secondary Index for escrows - to query all escrows regardless of aid
    escrowsTable.addGlobalSecondaryIndex({
      indexName: "escrows-by-reason",
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
      exportName: "KerihostKelTableName",
    });

    new cdk.CfnOutput(this, "StatesTableName", {
      value: statesTable.tableName,
      description: "DynamoDB table for Key States",
      exportName: "KerihostStatesTableName",
    });

    new cdk.CfnOutput(this, "ReceiptsTableName", {
      value: receiptsTable.tableName,
      description: "DynamoDB table for Witness Receipts",
      exportName: "KerihostReceiptsTableName",
    });

    new cdk.CfnOutput(this, "EscrowsTableName", {
      value: escrowsTable.tableName,
      description: "DynamoDB table for Escrowed Events",
      exportName: "KerihostEscrowsTableName",
    });
  }
}
