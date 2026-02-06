import * as cdk from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as events from "aws-cdk-lib/aws-events";
import * as targets from "aws-cdk-lib/aws-events-targets";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as route53Targets from "aws-cdk-lib/aws-route53-targets";
import * as secretsmanager from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import * as path from "path";
import { API_DEFAULTS, LAMBDA_SLUGS } from "../config/constants";

export interface WitnessStackProps extends cdk.StackProps {
  /**
   * DynamoDB tables from DataStack
   */
  tables: {
    kel: dynamodb.ITable;
    states: dynamodb.ITable;
    receipts: dynamodb.ITable;
    escrows: dynamodb.ITable;
  };

  /**
   * Witness seed secret from DataStack
   */
  witnessSeed: secretsmanager.ISecret;

  /**
   * Custom domain name for the API (e.g., 'api.keri.host')
   */
  domainName: string;

  /**
   * Route53 hosted zone ID for DNS validation
   */
  hostedZoneId: string;

  /**
   * Base path for witness routes (e.g., 'witness')
   */
  basePath: string;
}

/**
 * WitnessStack contains the witness service including:
 * - API Gateway with custom domain
 * - 4 Rust Lambda functions (process, query, oobi, escrow-check)
 * - API routes under /{basePath}/...
 * - EventBridge schedule for escrow processing
 *
 * Resource names are derived from stack name: {StackName}-{slug}
 */
export class WitnessStack extends cdk.Stack {
  public readonly api: apigateway.RestApi;
  public readonly customDomainUrl: string;

  constructor(scope: Construct, id: string, props: WitnessStackProps) {
    super(scope, id, props);

    const { tables, witnessSeed, domainName, hostedZoneId, basePath } = props;

    // Helper to create resource name: {StackName}-{slug}
    const resourceName = (slug: string) => `${this.stackName}-${slug}`;

    // =======================================================================
    // Route53 Hosted Zone
    // =======================================================================

    const hostedZone = route53.HostedZone.fromHostedZoneAttributes(
      this,
      "HostedZone",
      {
        hostedZoneId,
        zoneName: domainName.split(".").slice(-2).join("."), // Extract root domain
      }
    );

    // =======================================================================
    // ACM Certificate
    // =======================================================================

    const certificate = new acm.Certificate(this, "ApiCertificate", {
      domainName,
      validation: acm.CertificateValidation.fromDns(hostedZone),
    });

    // =======================================================================
    // Lambda Environment Variables
    // =======================================================================

    const publicUrl = `https://${domainName}/${basePath}`;

    const lambdaEnv = {
      KEL_TABLE: tables.kel.tableName,
      STATES_TABLE: tables.states.tableName,
      RECEIPTS_TABLE: tables.receipts.tableName,
      ESCROWS_TABLE: tables.escrows.tableName,
      WITNESS_PREFIX: "BWitness_Kerihost_001", // Default prefix if no signer
      PUBLIC_URL: publicUrl,
      STRICT_VALIDATION: "false", // Lenient mode by default
      RUST_LOG: "info",
    };

    // Path to workspace root (relative to infrastructure)
    const workspaceRoot = path.join(__dirname, "../../..");

    // =======================================================================
    // Lambda Functions (Rust via cargo-lambda-cdk)
    // =======================================================================

    // Process Lambda - POST /process
    const processLambda = new RustFunction(this, "ProcessLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-process",
      functionName: resourceName(LAMBDA_SLUGS.PROCESS),
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(30),
      memorySize: 256,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // Query Lambda - POST /query
    const queryLambda = new RustFunction(this, "QueryLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-query",
      functionName: resourceName(LAMBDA_SLUGS.QUERY),
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(10),
      memorySize: 256,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // OOBI Lambda - GET /introduce, GET /oobi/{id}
    const oobiLambda = new RustFunction(this, "OobiLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-oobi",
      functionName: resourceName(LAMBDA_SLUGS.OOBI),
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(5),
      memorySize: 128,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // Escrow Check Lambda - Scheduled
    const escrowCheckLambda = new RustFunction(this, "EscrowCheckLambda", {
      manifestPath: path.join(workspaceRoot, "Cargo.toml"),
      binaryName: "witness-escrow-check",
      functionName: resourceName(LAMBDA_SLUGS.ESCROW_CHECK),
      environment: lambdaEnv,
      timeout: cdk.Duration.seconds(60),
      memorySize: 256,
      architecture: cdk.aws_lambda.Architecture.X86_64,
    });

    // =======================================================================
    // DynamoDB Permissions
    // =======================================================================

    // Process Lambda needs read/write to all tables
    tables.kel.grantReadWriteData(processLambda);
    tables.states.grantReadWriteData(processLambda);
    tables.receipts.grantReadWriteData(processLambda);
    tables.escrows.grantReadWriteData(processLambda);

    // Query Lambda only needs read access
    tables.kel.grantReadData(queryLambda);
    tables.states.grantReadData(queryLambda);
    tables.receipts.grantReadData(queryLambda);

    // OOBI Lambda needs to read states and receipts
    tables.states.grantReadData(oobiLambda);
    tables.receipts.grantReadData(oobiLambda);

    // Escrow Check Lambda needs read/write to escrows and read/write to KEL/states
    tables.kel.grantReadWriteData(escrowCheckLambda);
    tables.states.grantReadWriteData(escrowCheckLambda);
    tables.escrows.grantReadWriteData(escrowCheckLambda);

    // Grant secret read access to lambdas that need signing capability
    witnessSeed.grantRead(processLambda);
    witnessSeed.grantRead(escrowCheckLambda);

    // =======================================================================
    // API Gateway
    // =======================================================================

    this.api = new apigateway.RestApi(this, "WitnessApi", {
      restApiName: resourceName("api"),
      description: `KERI Host API Gateway - ${domainName}`,
      deployOptions: {
        stageName: "prod",
        throttlingRateLimit: API_DEFAULTS.THROTTLE_RATE_LIMIT,
        throttlingBurstLimit: API_DEFAULTS.THROTTLE_BURST_LIMIT,
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
      domainName: {
        domainName,
        certificate,
        endpointType: apigateway.EndpointType.REGIONAL,
      },
    });

    // =======================================================================
    // Route53 A Record
    // =======================================================================

    new route53.ARecord(this, "ApiARecord", {
      zone: hostedZone,
      recordName: domainName,
      target: route53.RecordTarget.fromAlias(
        new route53Targets.ApiGateway(this.api)
      ),
    });

    // =======================================================================
    // API Routes
    // =======================================================================

    // Create base resource for witness routes: /{basePath}
    const witnessResource = this.api.root.addResource(basePath);

    // GET /{basePath} - Witness OOBI at base path
    witnessResource.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, { proxy: true })
    );

    // GET /{basePath}/introduce - Get witness OOBI
    const introduceResource = witnessResource.addResource("introduce");
    introduceResource.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, { proxy: true })
    );

    // POST /{basePath}/process - Submit KERI events
    const processResource = witnessResource.addResource("process");
    processResource.addMethod(
      "POST",
      new apigateway.LambdaIntegration(processLambda, { proxy: true })
    );

    // POST /{basePath}/query - Query KEL, state, receipts
    const queryResource = witnessResource.addResource("query");
    queryResource.addMethod(
      "POST",
      new apigateway.LambdaIntegration(queryLambda, { proxy: true })
    );

    // GET /{basePath}/oobi/{id} - Resolve OOBI for identifier
    const oobiResource = witnessResource.addResource("oobi");
    const oobiIdResource = oobiResource.addResource("{id}");
    oobiIdResource.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, { proxy: true })
    );

    // Support nested OOBI paths: /{basePath}/oobi/{id}/witness/{witness}
    const oobiWitnessResource = oobiIdResource.addResource("witness");
    const oobiWitnessIdResource = oobiWitnessResource.addResource("{witness}");
    oobiWitnessIdResource.addMethod(
      "GET",
      new apigateway.LambdaIntegration(oobiLambda, { proxy: true })
    );

    // =======================================================================
    // Scheduled Escrow Processing
    // =======================================================================

    // Run escrow check every 5 minutes
    new events.Rule(this, "EscrowCheckSchedule", {
      ruleName: resourceName("escrow-schedule"),
      schedule: events.Schedule.rate(cdk.Duration.minutes(5)),
      targets: [new targets.LambdaFunction(escrowCheckLambda)],
      description: "Scheduled check for promotable escrowed events",
    });

    // =======================================================================
    // Exports
    // =======================================================================

    this.customDomainUrl = `https://${domainName}`;

    new cdk.CfnOutput(this, "ApiUrl", {
      value: this.api.url,
      description: "API Gateway default URL",
      exportName: `${this.stackName}-ApiUrl`,
    });

    new cdk.CfnOutput(this, "CustomDomainUrl", {
      value: this.customDomainUrl,
      description: "API Gateway custom domain URL",
      exportName: `${this.stackName}-CustomDomainUrl`,
    });

    new cdk.CfnOutput(this, "WitnessBasePath", {
      value: `/${basePath}`,
      description: "Base path for witness API routes",
    });

    new cdk.CfnOutput(this, "ProcessLambdaArn", {
      value: processLambda.functionArn,
      description: "Process Lambda function ARN",
    });

    new cdk.CfnOutput(this, "QueryLambdaArn", {
      value: queryLambda.functionArn,
      description: "Query Lambda function ARN",
    });

    new cdk.CfnOutput(this, "OobiLambdaArn", {
      value: oobiLambda.functionArn,
      description: "OOBI Lambda function ARN",
    });

    new cdk.CfnOutput(this, "EscrowCheckLambdaArn", {
      value: escrowCheckLambda.functionArn,
      description: "Escrow Check Lambda function ARN",
    });
  }
}
