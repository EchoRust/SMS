//! # aliyun SMS
//!
//! **阿里云短信sdk**
//!
//! 目前实现了发送短信功能
//!

use chrono::{SecondsFormat, Utc};
use ring::hmac;
use std::collections::HashMap;

/// SMS API 版本
const SMS_VERSION: &str = "2017-05-25";

/// 签名算法版本。目前为固定值 `1.0`。
const SIGNATURE_VERSION: &str = "1.0";

/// 签名方式。目前为固定值 `HMAC-SHA1`。
const SIGNATURE_METHOD: &str = "HMAC-SHA1";

/// 指定接口返回数据的格式。可以选择 `JSON` 或者 `XML`。默认为 `XML`。
///
/// 这里选择 `JSON`。
const FORMAT: &str = "json";

/// aliyun sms
pub struct Aliyun<'a> {
    access_key_id: &'a str,
    access_secret: &'a str,
}

impl<'a> Aliyun<'a> {
    /// init access key
    /// 初始化密钥
    ///
    /// ```rust,no_run
    /// use sms::aliyun::Aliyun;
    ///
    /// let aliyun = Aliyun::new("xxxx", "xxxx");
    ///
    /// ```
    pub fn new(access_key_id: &'a str, access_secret: &'a str) -> Self {
        Self {
            access_key_id,
            access_secret,
        }
    }

    /// send_sms
    /// 发送短信
    ///
    /// ```rust,no_run
    /// use sms::aliyun::Aliyun;
    /// use rand::prelude::*;
    ///
    /// let aliyun = Aliyun::new("xxxx", "xxxx");
    ///
    /// let mut rng = rand::thread_rng();
    /// let code = format!(
    ///     r#"{{"code":"{}","product":"EchoLi"}}"#,
    ///     rng.gen_range(1000..=9999)
    /// );
    ///
    /// let resp = aliyun
    ///     .send_sms("18888888888", "登录验证", "SMS_123456", code.as_str())
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{:?}", resp);
    /// ```
    pub async fn send_sms(
        &self,
        phone_numbers: &'a str,
        sign_name: &'a str,
        template_code: &'a str,
        template_param: &'a str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();

        params.insert("PhoneNumbers", phone_numbers);
        params.insert("SignName", sign_name);
        params.insert("TemplateCode", template_code);
        params.insert("RegionId", "cn-hangzhou");
        params.insert("TemplateParam", template_param);
        params.insert("Action", "SendSms");
        params.insert("Version", SMS_VERSION);

        // 构造规范化请求字符串
        let canonicalize_query_string = self.canonicalize_query_string(&params);

        // 构造签名字符串
        let signature = self.signature(
            format!(
                "GET&%2F&{}",
                urlencoding::encode(&canonicalize_query_string)
            )
            .as_bytes(),
        );

        let url = format!(
            "https://dysmsapi.aliyuncs.com/?{}&Signature={}",
            canonicalize_query_string, signature
        );

        let resp = reqwest::get(url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;

        Ok(resp)
    }

    /// 构造规范化请求字符串
    ///
    /// 详见链接: https://help.aliyun.com/document_detail/315526.html#sectiondiv-y9b-x9s-wvp
    fn canonicalize_query_string(&self, params: &HashMap<&str, &'a str>) -> String {
        let now = Utc::now();

        let signature_nonce = now.timestamp_micros().to_string();
        let timestamp = now.to_rfc3339_opts(SecondsFormat::Secs, true);

        let mut all_params = HashMap::new();

        all_params.insert("AccessKeyId", self.access_key_id);
        all_params.insert("Format", FORMAT);
        all_params.insert("SignatureMethod", SIGNATURE_METHOD);
        all_params.insert("SignatureNonce", signature_nonce.as_str());
        all_params.insert("SignatureVersion", SIGNATURE_VERSION);
        all_params.insert("Timestamp", timestamp.as_str());

        params.iter().for_each(|(&k, &v)| {
            all_params.insert(k, v);
        });

        let mut vec_arams: Vec<String> = all_params
            .iter()
            .map(|(&k, &v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect();

        vec_arams.sort();

        vec_arams.join("&")
    }

    /// 构建签名字符串
    fn signature(&self, string_to_sign: &[u8]) -> String {
        let key = hmac::Key::new(
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            format!("{}&", self.access_secret).as_bytes(),
        );

        let sign = hmac::sign(&key, string_to_sign);

        base64::encode(sign.as_ref())
    }
}
