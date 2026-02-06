import * as cdk from "aws-cdk-lib";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as route53Targets from "aws-cdk-lib/aws-route53-targets";
import { Construct } from "constructs";
import { API_DEFAULTS } from "../config/constants";

export interface ApiStackProps extends cdk.StackProps {
  /**
   * Custom domain name for the API (e.g., 'api.keri.host')
   */
  domainName: string;

  /**
   * Route53 hosted zone ID for DNS validation
   */
  hostedZoneId: string;
}

/**
 * ApiStack contains shared API infrastructure:
 * - REST API Gateway
 * - ACM Certificate with DNS validation
 * - Custom domain mapping
 * - Route53 A record
 *
 * Service stacks (Witness, Watcher, Controller) attach their routes to this API.
 */
export class ApiStack extends cdk.Stack {
  public readonly api: apigateway.RestApi;
  public readonly customDomainUrl: string;

  constructor(scope: Construct, id: string, props: ApiStackProps) {
    super(scope, id, props);

    const { domainName, hostedZoneId } = props;

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
    // API Gateway
    // =======================================================================

    this.api = new apigateway.RestApi(this, "Api", {
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

    new cdk.CfnOutput(this, "ApiId", {
      value: this.api.restApiId,
      description: "API Gateway REST API ID",
      exportName: `${this.stackName}-ApiId`,
    });
  }
}
