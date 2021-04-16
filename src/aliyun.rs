use chrono::Utc;
use ring::hmac;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Aliyun<'a> {
    access_key_id: &'a str,
    access_secret: &'a str,
}

impl<'a> Aliyun<'a> {
    pub fn new(access_key_id: &'a str, access_secret: &'a str) -> Self {
        Self {
            access_key_id,
            access_secret,
        }
    }

    pub async fn send_sms(
        &self,
        phone_num: &'a str,
        sign_name: &'a str,
        template_code: &'a str,
        template_param: &'a str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut parameters = Vec::new();
        parameters.push(get_param_str("AccessKeyId", self.access_key_id));
        parameters.push(get_param_str("SignatureVersion", "1.0"));
        parameters.push(get_param_str("SignatureMethod", "HMAC-SHA1"));
        parameters.push(get_param_str(
            "SignatureNonce",
            Uuid::new_v4().to_string().as_str(),
        ));
        parameters.push(get_param_str(
            "Timestamp",
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string().as_str(),
        ));
        parameters.push(get_param_str("Format", "json"));
        parameters.push(get_param_str("Action", "SendSms"));
        parameters.push(get_param_str("Version", "2017-05-25"));
        parameters.push(get_param_str("RegionId", "cn-hangzhou"));
        parameters.push(get_param_str("SignName", sign_name));
        parameters.push(get_param_str("TemplateCode", template_code));

        // let mut rng = rand::thread_rng();
        // let code = format!(
        //     "{{\"code\":\"{}\",\"product\":\"幻果软件\"}}",
        //     rng.gen_range(1000..=9999)
        // );
        parameters.push(get_param_str("TemplateParam", template_param));
        parameters.push(get_param_str("PhoneNumbers", phone_num));

        parameters.sort();

        let sort_query_string = parameters.join("&");

        let string_to_sign = format!("GET&%2F&{}", special_url_encode(sort_query_string.as_str()));

        // 加密
        let key = hmac::Key::new(
            hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
            format!("{}&", self.access_secret).as_bytes(),
        );
        let sign = hmac::sign(&key, string_to_sign.as_bytes());

        let signature = base64::encode(sign.as_ref()).to_string();
        // println!("Sign: {}", signature);

        parameters.push(get_param_str("Signature", signature.as_str()));

        let url = parameters.join("&");

        let resp = reqwest::get(format!("https://dysmsapi.aliyuncs.com/?{}", url).as_str())
            .await?
            .json::<HashMap<String, String>>()
            .await?;

        Ok(resp)
    }
}

fn get_param_str(key: &str, value: &str) -> String {
    format!("{}={}", special_url_encode(key), special_url_encode(value))
}

fn special_url_encode(value: &str) -> String {
    form_urlencoded::Serializer::new(String::new())
        .append_key_only(value)
        .finish()
        .replace("+", "%20")
        .replace("*", "%2A")
        .replace("%7E", "~")
}
