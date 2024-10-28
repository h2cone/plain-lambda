# plain-lambda

An AWS Lambda function using Rust to demonstrate S3 object transformation during a GetObject request.

## Build and Deploy

1. Install [cargo-lambda](https://www.cargo-lambda.info/guide/installation.html)
2. Run `cargo lambda build --release --arm64 --output-format zip` in this project directory
3. Upload `target/lambda/plain-lambda/bootstrap.zip` to AWS Lambda, select Amazon Linux 2023 as the runtime, and ARM64 as the architecture
4. Configure permissions and resources in AWS S3, AWS Lambda, and AWS IAM. For details, refer to [object-lambda](https://aws.amazon.com/s3/features/object-lambda)

## Read the Transformed Object

Upload `assets/sonnets.txt` to your S3 bucket and create an S3 Object Lambda Access Point with the Lambda function you deployed.

You can mount the Object Lambda Access Point using [awslabs/mountpoint-s3](https://github.com/awslabs/mountpoint-s3) to read the transformed object.

```shell
mount-s3 <Object Lambda Access Point Alias> /path/to/mnt/name
```

Open the `/path/to/mnt/name/sonnets.txt` and you should see the content of the original object transformed by the Lambda function.

## Related Links

* [basic-s3-object-lambda-hello](https://github.com/peterborkuti/basic-s3-object-lambda-hello)
* [basic-streaming-response](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples/basic-streaming-response)
* [Building Lambda functions with Rust](https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html)
* [Working with Range and partNumber headers](https://docs.aws.amazon.com/AmazonS3/latest/userguide/range-get-olap.html)
