use rand::prelude::*;
use sms::aliyun::Aliyun;

#[tokio::test]
async fn send_sms() {
    let aliyun = Aliyun::new("xxxx", "xxxx");

    let mut rng = rand::thread_rng();
    let code = format!(
        r#"{{"code":"{}","product":"EchoLi"}}"#,
        rng.gen_range(1000..=9999)
    );

    let resp = aliyun
        .send_sms("18888888888", "登录验证", "SMS_123456", code.as_str())
        .await
        .unwrap();

    assert_eq!(resp.get(&"Code".to_string()), Some(&"OK".to_string()));
}
