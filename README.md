# Easy to use Rust API for Cloudflare's R2 service 

Simple Rust API for Cloudflare's R2 and OVH Object Storage (possibly others as well but not tested) with **create_bucket**, **delete_bucket** and  **upload**, **get**, **delete** file operations. Production ready since it's a thin layer above AWS SDK S3. Updated to be using latest versions.

### cargo.toml
```TOML
[dependencies]
cloudflare-r2-rs = "0.6.2"
```

### example use
```rust

//Cloudflare
let r2_manager = R2Manager::new(
    //Bucket Name
    "my-bucket", 
    //Cloudflare URI endpoint
    "https://some-id-55353-53535.r2.cloudflarestorage.com",
    //API Token's Access Key ID
    "some-id-55353-53535",
    //API Token's Secret Access Key
    "some-long-secret-key-55353-53535-55353-53535"
).await;

//OVH
let _r2_manager2 = R2Manager::new_with_region(
    "bucket-name", 
    "https://s3.<region>.io.cloud.ovh.net",
    "some-id-55353-53535",
    "some-secret-55353-53535",
    "<region>").await;

//Gives control to set Cache-Control header and Content-Type header
r2_manager.upload("test", b"Hello world", Some("max-age=60"), Some("text/plain")).await;
let bytes = r2_manager.get("test").await.unwrap();
println!("{}", String::from_utf8(bytes).unwrap());
```