# SMS

Rust Send SMS SDK

English | [简体中文](./README.zh-CN.md)

Use Rust to connect to the SMS interface of the cloud platform, and the following cloud platform interfaces have been implemented

* aliyun

```rust
use sms::aliyun::Aliyun;

let aliyun = Aliyun::new("accessKeyId", "accessSecret");
let resp = aliyun.send_sms.await.unwarp();

println("{:?}", resp);
```

## License

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2020-present, Yang (Echo) Li