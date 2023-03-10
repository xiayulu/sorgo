use crate::error::Error;
use lettre::{
    message::{Mailbox, MultiPart},
    transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};
use log::error;
use rand::{thread_rng, Rng};
use std::env;

pub fn get_captcha() -> String {
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = thread_rng();

    (0..4)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            char::from(CHARSET[idx])
        })
        .collect()
}

pub fn send_captcha(captcha: &String, recipient: &String) -> Result<(), Error> {
    let html_msg = format!(
        r#"<table cellpadding="0" cellspacing="0" border="0" align="center">
    <tr>
        <td style="font-size: 18px;">喵喵喵, 您的验证码是:</td>
    </tr>
    <tr>
        <td>
            <h1 style="margin: 5px;">{}</h1>
        </td>
    </tr>
    <tr>
        <td style="font-size: 15px;">十分钟有效。</td>
    </tr>
    </table>"#,
        captcha
    );

    let from_box = Mailbox::new(
        Some("森喵".to_string()),
        env::var("EMAIL_HOST_USER")
            .unwrap()
            .parse::<Address>()
            .map_err(|e| {
                let e = format!("send email error:{}", e);
                error!("{}", e);
                Error::Service(e)
            })?,
    );
    let to_box = Mailbox::new(
        None,
        recipient.parse::<Address>().map_err(|e| {
            let e = format!("send email error:{}", e);
            error!("{}", e);
            Error::Service(e)
        })?,
    );

    let email = Message::builder()
        .from(from_box)
        .to(to_box)
        .subject("欢迎来到森喵喵")
        .multipart(MultiPart::alternative_plain_html(
            format!("欢迎来到森喵, 您的验证码是：{} 喵, 十分钟有效喵.", captcha),
            html_msg,
        ))
        .map_err(|e| {
            let e = format!("send email error:{}", e);
            error!("{}", e);
            Error::Service(e)
        })?;

    let creds = Credentials::new(
        env::var("EMAIL_HOST_USER").unwrap(),
        env::var("EMAIL_HOST_PASSWORD").unwrap(),
    );

    // Open a remote connection to the SMTP relay server
    let mailer = SmtpTransport::relay(env::var("EMAIL_HOST").unwrap().as_str())
        .map_err(|e| {
            let e = format!("send email error:{}", e);
            error!("{}", e);
            Error::Service(e)
        })?
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email).map_err(|e| {
        let e = format!("send email error:{}", e);
        error!("{}", e);
        Error::Service(e)
    })?;

    Ok(())
}
