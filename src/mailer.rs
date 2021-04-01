use std::fmt;

use crate::settings::Settings;
use actix::prelude::*;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message as MailMessage, SmtpTransport, Transport};
use tracing::{error, info};

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct MailMsg(pub MailMessage);

pub struct Mailer {
    pub transport: SmtpTransport,
}

impl fmt::Debug for Mailer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mailer")
    }
}

impl Actor for Mailer {
    type Context = Context<Self>;
}

impl Handler<MailMsg> for Mailer {
    type Result = ();

    #[tracing::instrument]
    fn handle(&mut self, msg: MailMsg, _: &mut Self::Context) -> Self::Result {
        match self.transport.send(&msg.0) {
            Err(e) => error!("error sending email: {:?}", e),
            _ => info!("sent email message"),
        }
    }
}

impl Mailer {
    pub fn new(settings: &Settings) -> Self {
        Self {
            transport: SmtpTransport::relay(&settings.smtp.domain)
                .unwrap()
                .credentials(Credentials::new(
                    settings.smtp.user.clone(),
                    settings.smtp.pass.clone(),
                ))
                .build(),
        }
    }
}
