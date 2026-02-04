import * as cdk from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as events from "aws-cdk-lib/aws-events";
import * as targets from "aws-cdk-lib/aws-events-targets";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import * as path from "path";

export class KerihostStack extends cdk.Stack {
  public readonly apiUrl: cdk.CfnOutput;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // =======================================================================
    // DynamoDB Tables
    // =======================================================================

    // KEL (Key Event Log) Table
    // PK: aid (AID/prefix), SK: sn (zero-padded sequence number)
    const kelTable = new dynamodb.Table(this, "KelTable", {
      tableName: "kerihost-kel",
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sn", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      pointInTimeRecovery: true,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    // States Table (current key state for each identifier)
    // PK: aid (AID/prefix)
    const statesTable = new dynamodb.Table(this, "StatesTable", {
      tableName: "kerihost-states",
      partitionKey: { name: "aid", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    // Receipts Table (witness receipts for events)
    // PK: event_digest, SK: witness_aid
    const receiptsTable = new dynamodb.Table(this, "ReceiptsTable", {
      tableName: "kerihost-receipts",
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
      tableName: "kerihost-escrows",
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
      "kerihost/witness-seed"
    );

    // =======================================================================
    // Lambda Environment Variables
    // =======================================================================

    const lambdaEnv = {
      KEL_TABLE: kelTable.tableName,
      STATES_TABLE: statesTable.tableName,
      RECEIPTS_TABLE: receiptsTable.tableName,
      ESCROWS_TABLE: escrowsTable.tableName,
      WITNESS_PREFIX: "BWitness_Kerihost_001", // Default prefix if no signer
      PUBLIC_URL: "https://witness.keri.host",
      STRICT_VALIDATION: "false", // Lenient mode by default
      RUST_LOG: "info",
    };

    // Path to workspace root (relative to infrastructure)
    const workspaceRoot = path.join(__dirname, "../..");

    // =======================================================================
    // Lambda Functions (Rust via cargo-lambda-cdk)
    // =======================================================================

    // Process Lambda - POST /process
    const processLambda = new RustFunction(this, "ProcessLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-process",
      functionName: "kerihost-witness-process",
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(30),
      memorySize: 256,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // Query Lambda - POST /query
    const queryLambda = new RustFunction(this, "QueryLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-query",
      functionName: "kerihost-witness-query",
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(10),
      memorySize: 256,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // OOBI Lambda - GET /introduce, GET /oobi/{id}
    const oobiLambda = new RustFunction(this, "OobiLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-oobi",
      functionName: "kerihost-witness-oobi",
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(5),
      memorySize: 128,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // Escrow Check Lambda - Scheduled
    const escrowCheckLambda = new RustFunction(this, "EscrowCheckLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-escrow-check",
      functionName: "kerihost-witness-escrow-check",
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(60),
      memorySize: 256,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // =======================================================================
    // DynamoDB Permissions
    // =======================================================================

    // Process Lambda needs read/write to all tables
    kelTable.grantReadWriteData(processLambda);
    statesTable.grantReadWriteData(processLambda);
    receiptsTable.grantReadWriteData(processLambda);
    escrowsTable.grantReadWriteData(processLambda);

    // Query Lambda only needs read access
    kelTable.grantReadData(queryLambda);
    statesTable.grantReadData(queryLambda);
    receiptsTable.grantReadData(queryLambda);

    // OOBI Lambda needs to read states and receipts (for enriching state with receipt count)
    statesTable.grantReadData(oobiLambda);
    receiptsTable.grantReadData(oobiLambda);

    // Escrow Check Lambda needs read/write to escrows and read/write to KEL/states
    kelTable.grantReadWriteData(escrowCheckLambda);
    statesTable.grantReadWriteData(escrowCheckLambda);
    escrowsTable.grantReadWriteData(escrowCheckLambda);

    // Grant secret read access to lambdas that need signing capability
    witnessSeedSecret.grantRead(processLambda);
    witnessSeedSecret.grantRead(escrowCheckLambda);

    // =======================================================================
    // API Gateway
    // =======================================================================

    const api = new apigateway.RestApi(this, "WitnessApi", {
      restApiName: "kerihost-witness",
      description: "KERI Witness Service - keri.host",
      deployOptions: {
        stageName: "prod",
        throttlingRateLimit: 1000,
        throttlingBurstLimit: 2000,
      },
      defaultCorsPreflightOptions: {
        allowOrigins: apigateway.Cors.ALL_ORIGINS,
        allowMethods: apigateway.Cors.ALL_METHODS,
        allowHeaders: [
          "Content-Type",
          "Authorization",
          "X-Amz-Date",
          "X-Api-Key",
        ],
      },
    });

    // POST /process - Submit KERI events
    const processResource = api.root.addResource("process");
    processResource.addMethod(
      "POST",
      new apigateway.LambdaIntegration(processLambda, {
        proxy: true,
      })
    );

    // POST /query - Query KEL, state, receipts
    const queryResource = api.root.addResource("query");
    queryResource.addMethod(
      "POST",
      new apigateway.LambdaIntegration(queryLambda, {
        proxy: true,
      })
    );

    // GET /introduce - Get witness OOBI
    const introduceResource = api.root.addResource("introduce");
    introduceResource.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, {
        proxy: true,
      })
    );

    // GET / - Also serve witness OOBI at root
    api.root.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, {
        proxy: true,
      })
    );

    // GET /oobi/{id} - Resolve OOBI for identifier
    const oobiResource = api.root.addResource("oobi");
    const oobiIdResource = oobiResource.addResource("{id}");
    oobiIdResource.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, {
        proxy: true,
      })
    );

    // Support nested OOBI paths: /oobi/{id}/witness/{witness}
    const witnessResource = oobiIdResource.addResource("witness");
    const witnessIdResource = witnessResource.addResource("{witness}");
    witnessIdResource.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, {
        proxy: true,
      })
    );

    // =======================================================================
    // Scheduled Escrow Processing
    // =======================================================================

    // Run escrow check every 5 minutes
    new events.Rule(this, "EscrowCheckSchedule", {
      schedule: events.Schedule.rate(cdk.Duration.minutes(5)),
      targets: [new targets.LambdaFunction(escrowCheckLambda)],
      description: "Scheduled check for promotable escrowed events",
    });

    // =======================================================================
    // Outputs
    // =======================================================================

    this.apiUrl = new cdk.CfnOutput(this, "ApiUrl", {
      value: api.url,
      description: "API Gateway URL for the witness service",
      exportName: "KerihostWitnessApiUrl",
    });

    new cdk.CfnOutput(this, "KelTableName", {
      value: kelTable.tableName,
      description: "DynamoDB table for Key Event Log",
    });

    new cdk.CfnOutput(this, "StatesTableName", {
      value: statesTable.tableName,
      description: "DynamoDB table for Key States",
    });

    new cdk.CfnOutput(this, "ReceiptsTableName", {
      value: receiptsTable.tableName,
      description: "DynamoDB table for Witness Receipts",
    });

    new cdk.CfnOutput(this, "EscrowsTableName", {
      value: escrowsTable.tableName,
      description: "DynamoDB table for Escrowed Events",
    });
  }
}
