import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";

import "dotenv/config";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as targets from "aws-cdk-lib/aws-route53-targets";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as apigateway from "aws-cdk-lib/aws-apigateway";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as ec2 from "aws-cdk-lib/aws-ec2";

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    let domain = process.env.DOMAIN || "";
    let acmId = process.env.ACM_ID || "";
    let hostedZoneId = process.env.HOSTED_ZONE_ID || "";
    let tableName = process.env.TABLE_NAME || "";
    let vpcId = process.env.VPC_ID || "";
    let securityGroupId = process.env.SECURITY_GROUP_ID || "";
    let subnet1Id = process.env.SUBNET1_ID || "";
    let subnet2Id = process.env.SUBNET2_ID || "";

    const assetsBucket = new s3.Bucket(this, "Bucket", {
      bucketName: domain,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    const certificate = acm.Certificate.fromCertificateArn(
      this,
      "Certificate",
      acmId,
    );

    const table = new dynamodb.Table(this, "DynamoDB", {
      partitionKey: {
        name: "key",
        type: dynamodb.AttributeType.STRING,
      },
      tableName,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
    });

    const vpc = ec2.Vpc.fromLookup(this, "VPC", {
      vpcId,
    });
    const securityGroup = ec2.SecurityGroup.fromSecurityGroupId(
      this,
      "lambdaSecurityGroup",
      securityGroupId,
    );

    const subnet1 = ec2.Subnet.fromSubnetId(this, "subnet1", subnet1Id);
    const subnet2 = ec2.Subnet.fromSubnetId(this, "subnet2", subnet2Id);

    const func = new lambda.Function(this, "Function", {
      runtime: lambda.Runtime.PROVIDED_AL2023,
      code: lambda.Code.fromAsset(__dirname + "/../../../dist"),
      handler: "bootstrap",
      environment: {
        NO_COLOR: "true",
        ASSETS_PATH: "./",
      },
      // NOTE: Connection for Redis
      vpc,
      securityGroups: [securityGroup],
      memorySize: 512,
      timeout: cdk.Duration.seconds(10),
      vpcSubnets: {
        subnets: [subnet1, subnet2],
      },
    });

    table.grantReadWriteData(func);

    const api = new apigateway.LambdaRestApi(this, "Api", {
      handler: func,
      proxy: true,
    });

    const s3Origin = new origins.S3Origin(assetsBucket);
    const apiOrigin = new origins.RestApiOrigin(api);

    const cf = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior: {
        origin: apiOrigin,
        cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
        allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
        cachedMethods: cloudfront.CachedMethods.CACHE_GET_HEAD_OPTIONS,
        originRequestPolicy:
          cloudfront.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
      },
      additionalBehaviors: {
        "/assets/*": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.js": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.css": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.html": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.ico": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/*.svg": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/icons/*": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        "/images/*": {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
      },
      domainNames: [domain],
      certificate,
    });

    const zone = route53.HostedZone.fromHostedZoneAttributes(
      this,
      "zone-attribute",
      {
        zoneName: domain,
        hostedZoneId,
      },
    );

    new route53.ARecord(this, "IpV4Record", {
      zone,
      target: route53.RecordTarget.fromAlias(new targets.CloudFrontTarget(cf)),
    });

    new route53.AaaaRecord(this, "IpV6Record", {
      zone,
      target: route53.RecordTarget.fromAlias(new targets.CloudFrontTarget(cf)),
    });
  }
}
