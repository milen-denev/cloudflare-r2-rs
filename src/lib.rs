/// # Easy to use rust API for Cloudflare's R2 and OVH Object Storage service (and possibly others as well) services
/// ### Examples
/// ```rust
/// use cloudflare_r2_rs::r2::{R2Endpoint, R2Manager};
/// //Cloudflare
/// let r2_manager = R2Manager::new(
///     //Bucket Name
///     "my-bucket", 
///     //Cloudflare URI endpoint
///     R2Endpoint::Http("https://some-id-55353-53535.r2.cloudflarestorage.com".to_string()),
///     //API Token's Access Key ID
///     "some-id-55353-53535",
///     //API Token's Secret Access Key
///     "some-long-secret-key-55353-53535-55353-53535",
///     //Region (None defaults to "us-east-1")
///     None
/// ).await;
/// 
/// //OVH
/// let _ovh_manager = R2Manager::new(
///     "bucket-name",
///     R2Endpoint::Http("https://s3.<region>.io.cloud.ovh.net".to_string()),
///     "some-id-55353-53535",
///     "some-secret-55353-53535",
///     Some("<region>".to_string())
/// ).await;
/// 
/// //AWS-style endpoint built from bucket + region
/// let _aws_style = R2Manager::new(
///     "bucket-name",
///     R2Endpoint::Bucket,
///     "iam-access-key-id",
///     "iam-secret-access-key",
///     Some("<region>".to_string())
/// ).await;
/// 
/// //Gives control to set Cache-Control header and Content-Type header
/// r2_manager.upload("test", b"Hello world", Some("max-age=60"), Some("text/plain")).await;
/// let bytes = r2_manager.get("test").await.unwrap();
/// println!("{}", String::from_utf8(bytes).unwrap());
/// ```
pub mod r2;