use ini::Ini;
use std::path::Path;

pub fn read_netconfig(path: impl AsRef<Path>) -> Result<Option<Vec<String>>, anyhow::Error> {
    if let Err(e) = dotenv::from_filename(path) {
        return Err(e.into());
    };
    if let Ok(dns_policy) = dotenv::var("NETCONFIG_DNS_POLICY") {
        if !dns_policy
            .split(' ')
            .collect::<Vec<&str>>()
            .contains(&"STATIC")
        {
            return Ok(None);
        }
    }
    if let Ok(static_dns_servers) = dotenv::var("NETCONFIG_DNS_STATIC_SERVERS") {
        if static_dns_servers.is_empty() {
            return Ok(None);
        }
        return Ok(Some(
            static_dns_servers
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        ));
    }
    Ok(None)
}

pub fn write_nmconf_netconfig(
    static_servers: Vec<String>,
    nm_dropin_dir: String,
) -> std::io::Result<()> {
    let servers_conf_string: String = static_servers.join(",");
    let mut conf = Ini::new();
    conf.with_section(Some("global-dns-domain-*"))
        .set("servers", servers_conf_string);
    conf.write_to_file(format!("{}/global_dns.conf", nm_dropin_dir))?;
    Ok(())
}
