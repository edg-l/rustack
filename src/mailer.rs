use crate::settings::Settings;
use actix::prelude::*;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, SmtpTransport, Transport};
use lettre_email::Email;

#[derive(Message)]
#[rtype(result = "()")]
pub struct MailMsg(pub Email);

pub struct Mailer {
    pub transport: SmtpTransport,
}

impl Actor for Mailer {
    type Context = Context<Self>;
}

impl Handler<MailMsg> for Mailer {
    type Result = ();

    fn handle(&mut self, msg: MailMsg, _: &mut Self::Context) -> Self::Result {
        match self.transport.send(msg.0.into()) {
            Err(e) => log::error!(target: "mailer", "error sending email: {:?}", e),
            _ => {}
        }
    }
}

impl Mailer {
    pub fn new(settings: &Settings) -> Self {
        Self {
            transport: SmtpTransport::new({
                let mut client = SmtpClient::new_simple(&settings.smtp.domain).unwrap();

                client = client.credentials(Credentials::new(
                    settings.smtp.user.clone(),
                    settings.smtp.pass.clone(),
                ));

                client
            }),
        }
    }
}
