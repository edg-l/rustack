//! Mailer actor
//!
//! This actor is used to queue and send emails
//!

use std::fmt;

use crate::settings::Settings;
use actix::prelude::*;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message as MailMessage, SmtpTransport, Transport};
use tracing::debug;

#[derive(Message, Debug)]
#[rtype(result = "Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error>")]
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
    type Result = Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error>;

    #[tracing::instrument]
    fn handle(&mut self, msg: MailMsg, _: &mut Self::Context) -> Self::Result {
        debug!("Sending email: {:?}", msg);
        self.transport.send(&msg.0)
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
