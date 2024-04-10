pub mod error;
mod generate;

pub type Result<T> = std::result::Result<T, error::Error>;

use generate::DkimKeyPair;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DnsRecords {
    pub dkim: Vec<Record>,
    pub spf: Record,
    pub dmarc: Record,
    pub dkim_keys: DkimKeys,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DkimKeys {
    pub public_key: String,
    pub private_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    #[serde(default = "default_type")]
    pub record_type: String,
    pub record_name: String,
    pub record_content: String,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            record_type: default_type(),
            record_name: "".to_string(),
            record_content: "".to_string(),
        }
    }
}

pub fn generate_dns_records(
    domain: &str,
    selector: &str,
    bits: Option<usize>,
) -> Result<DnsRecords> {
    let bits = bits.unwrap_or(2048);
    let spf = Record {
        record_name: domain.to_string(),
        record_content: "v=spf1 -all".to_string(),
        ..Default::default()
    };
    let dmarc = Record {
        record_name: "_dmarc.".to_string() + domain,
        record_content: format!(
            "v=DMARC1; p=reject; rua=mailto:postmaster@{domain}; ruf=mailto:postmaster@{domain}"
        ),
        ..Default::default()
    };
    let dkim_rsa_pair = DkimKeyPair::generate_rsa(bits)?;

    // public key in string format
    let public_key_str = dkim_rsa_pair
        .public_key_x509
        .lines()
        .filter(|x| !x.starts_with('-'))
        .collect::<Vec<_>>()
        .join("");

    // DKIM Record Key/Value
    let dkim_record_key = format!("{selector}r._domainkey.{domain}");
    let dkim_record_value = format!("v=DKIM1; k=rsa; h=sha256; p={public_key_str}");

    let dkim_keys = DkimKeys {
        public_key: dkim_rsa_pair.public_key_x509,
        private_key: dkim_rsa_pair.private_key_x509,
    };
    let dkim = vec![Record {
        record_name: dkim_record_key,
        record_content: dkim_record_value,
        ..Default::default()
    }];
    Ok(DnsRecords {
        dkim,
        spf,
        dmarc,
        dkim_keys,
    })
}

fn default_type() -> String {
    "TXT".to_string()
}
