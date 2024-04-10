use clap::Parser;
use colored::Colorize;
use prettytable::{color, format::Alignment, Attr, Cell, Row, Table};

#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct CliOpts {
    /// Domain name with TLD (eg: example.com)
    domain: String,
    /// DKIM selector
    #[arg(long, value_name = "DKIM-SELECTOR", default_value = "s1")]
    selector: String,
}

impl CliOpts {
    pub fn parse_cli() -> Self {
        CliOpts::parse()
    }
    /// Get the domain name with TLD
    pub fn get_domain(&self) -> &str {
        &self.domain
    }
    /// Get the DKIM selector
    pub fn get_selector(&self) -> &str {
        &self.selector
    }
}

fn main() {
    let cli = CliOpts::parse_cli();
    let domain = cli.get_domain();
    if domain.is_empty() {
        eprintln!(
            "{}",
            "Please provide a domain name with TLD (eg: example.com)"
                .red()
                .bold()
        );
        std::process::exit(1);
    } else if !domain.contains('.') {
        eprintln!(
            "{}",
            "Domain name should have a tld (eg: example.com)"
                .red()
                .bold()
        );
        std::process::exit(1);
    }

    let dkim = gen::generate_dkim(domain, cli.get_selector(), None);
    match dkim {
        Ok(records) => {
            println!("{} {}\n",
                "Add the following DNS records to your domain in order to enable DKIM, SPF and DMARC:".bold(),
                domain.blue().bold()
            );
            let mut tbl = Table::new();
            let tbl_format = prettytable::format::FormatBuilder::new()
                .column_separator(' ')
                .padding(1, 1)
                .build();
            tbl.set_format(tbl_format);
            tbl.set_titles(Row::new(vec![Cell::new_align(
                "DNS Records",
                Alignment::CENTER,
            )
            .with_style(Attr::Bold)
            .with_hspan(3)]));
            tbl.set_titles(Row::new(vec![
                Cell::new_align("TYPE", Alignment::LEFT)
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BLUE)),
                Cell::new_align("NAME", Alignment::LEFT)
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BLUE)),
                Cell::new_align("CONTENT", Alignment::LEFT)
                    .with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::BLUE)),
            ]));
            tbl.add_row(Row::new(vec![
                Cell::new_align(&records.spf.record_type, Alignment::LEFT).with_style(Attr::Bold),
                Cell::new_align(&records.spf.record_name, Alignment::LEFT).with_style(Attr::Bold),
                Cell::new_align(&records.spf.record_content, Alignment::LEFT)
                    .with_style(Attr::Bold),
            ]));
            tbl.add_row(Row::new(vec![
                Cell::new_align(&records.dmarc.record_type, Alignment::LEFT).with_style(Attr::Bold),
                Cell::new_align(&records.dmarc.record_name, Alignment::LEFT).with_style(Attr::Bold),
                Cell::new_align(&records.dmarc.record_content, Alignment::LEFT)
                    .with_style(Attr::Bold),
            ]));
            for dkim in records.dkim {
                tbl.add_row(Row::new(vec![
                    Cell::new_align(&dkim.record_type, Alignment::LEFT).with_style(Attr::Bold),
                    Cell::new_align(&dkim.record_name, Alignment::LEFT).with_style(Attr::Bold),
                    Cell::new_align(&dkim.record_content, Alignment::LEFT).with_style(Attr::Bold),
                ]));
            }
            tbl.printstd();

            println!();
            println!(
                "{}",
                "Save the following DKIM Private/Public keys for your SMTP Server"
                    .magenta()
                    .bold()
            );
            println!(
                "{} : You must enter this key in your DKIM signer. \
                It must be kept secret, as anyone with access to it can stamp \
                tokens pretending to be you.\n\n{}",
                "Private Key".bold(),
                &records.dkim_keys.private_key.red().bold()
            );
            println!(
                "{} : This is the public key in the original raw \"X509\" format. \
                It's not usable in DNS directly, but it might be useful for something else.\n\n{}",
                "Public Key".bold(),
                &records.dkim_keys.public_key.black().bold()
            );
        }
        Err(e) => {
            eprintln!("Error generating DKIM keys: {:?}", e);
            std::process::exit(1);
        }
    }
}
