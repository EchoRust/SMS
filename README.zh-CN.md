# 短信

Rust发送短信SDK

简体中文 | [English](./README.md)

使用Rust对接云平台短信接口，目前已经实现以下云平台接口

* 阿里云

```rust
use sms::aliyun::Aliyun;

let aliyun = Aliyun::new("accessKeyId", "accessSecret");
let resp = aliyun.send_sms("18888888888", "登录验证", "SMS_5003224", "验证码1234").await.unwarp();

println("{:?}", resp);
```

## 版权声明

[MIT](https://opensource.org/licenses/MIT)

Copyright (c) 2020-present, Yang (Echo) Li