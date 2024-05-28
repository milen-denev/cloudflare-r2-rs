/// # Easy to use rust API for Cloudflare's R2 service 
/// ### Examples
/// ```rust
/// let r2_manager = R2Manager::new(
///     //Bucket Name
///     "my-bucket", 
///     //Cloudflare URI endpoint
///     "https://some-id-55353-53535.r2.cloudflarestorage.com",
///     //API Token's Access Key ID
///     "some-id-55353-53535",
///     //API Token's Secret Access Key
///     "some-long-secret-key-55353-53535-55353-53535"
/// );
/// 
/// //Gives control to set Cache-Control header and Content-Type header
/// r2_manager.upload("test", b"Hello world", Some("max-age=60"), Some("text/plain")).await;
/// let bytes = r2_manager.get("test").await.unwrap();
/// println!("{}", String::from_utf8(bytes).unwrap());
/// ```
pub mod r2;