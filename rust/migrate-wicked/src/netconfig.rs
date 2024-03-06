use ini::Ini;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Netconfig {
    pub static_dns_servers: Option<Vec<String>>,
    pub static_dns_searchlist: Option<Vec<String>>,
}

pub fn read_netconfig(path: impl AsRef<Path>) -> Result<Netconfig, anyhow::Error> {
    if let Err(e) = dotenv::from_filename(path) {
        return Err(e.into());
    };
    let mut netconfig = Netconfig::default();
    if let Ok(dns_policy) = dotenv::var("NETCONFIG_DNS_POLICY") {
        if !dns_policy
            .split(' ')
            .collect::<Vec<&str>>()
            .contains(&"STATIC")
        {
            return Ok(netconfig);
        }
    }
    if let Ok(static_dns_servers) = dotenv::var("NETCONFIG_DNS_STATIC_SERVERS") {
        if !static_dns_servers.is_empty() {
            netconfig.static_dns_servers = Some(
                static_dns_servers
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }
    }
    if let Ok(static_dns_searchlist) = dotenv::var("NETCONFIG_DNS_STATIC_SEARCHLIST") {
        if !static_dns_searchlist.is_empty() {
            netconfig.static_dns_searchlist = Some(
                static_dns_searchlist
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
        }
    }
    Ok(netconfig)
}

pub fn write_nmconf_netconfig(netconfig: Netconfig, nm_dropin_dir: String) -> std::io::Result<()> {
    if netconfig == Netconfig::default() {
        return Ok(());
    }
    let mut conf = Ini::new();
    if let Some(static_dns_searchlist) = netconfig.static_dns_searchlist {
        conf.with_section(Some("global-dns"))
            .set("servers", static_dns_searchlist.join(","));
    }
    if let Some(static_dns_servers) = netconfig.static_dns_servers {
        conf.with_section(Some("global-dns-domain-*"))
            .set("servers", static_dns_servers.join(","));
    }
    conf.write_to_file(format!("{}/global_dns.conf", nm_dropin_dir))?;
    Ok(())
}
