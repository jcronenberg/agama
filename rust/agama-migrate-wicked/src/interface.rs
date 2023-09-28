use agama_dbus_server::network::model::{self, IpConfig, IpMethod};
use agama_lib::network::types::DeviceType;
use cidr::IpInet;
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Interface {
    pub name: String,
    pub control: Control,
    pub firewall: Firewall,
    pub link: Link,
    pub ipv4: Ipv4,
    #[serde(rename = "ipv4-static", skip_serializing_if = "Option::is_none")]
    pub ipv4_static: Option<Ipv4Static>,
    pub ipv6: Ipv6,
    #[serde(rename = "ipv6-static", skip_serializing_if = "Option::is_none")]
    pub ipv6_static: Option<Ipv6Static>,
    #[serde(rename = "ipv6-dhcp", skip_serializing_if = "Option::is_none")]
    pub ipv6_dhcp: Option<Ipv6Dhcp>,
    #[serde(rename = "ipv6-auto", skip_serializing_if = "Option::is_none")]
    pub ipv6_auto: Option<Ipv6Auto>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Control {
    pub mode: String,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Firewall {}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Link {}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv4 {
    pub enabled: bool,
    #[serde(rename = "arp-verify")]
    pub arp_verify: bool,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv6 {
    pub enabled: bool,
    pub privacy: String,
    #[serde(rename = "accept-redir")]
    pub accept_redirects: bool,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv4Static {
    pub address: Address,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv6Static {
    pub address: Address,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Address {
    pub local: String,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv6Dhcp {
    pub enabled: bool,
    pub flags: String,
    #[serde(deserialize_with = "comma_separated_string_deserialize")]
    pub update: Vec<String>,
    pub mode: String,
    #[serde(rename = "rapid-commit")]
    pub rapid_commit: String,
    pub hostname: String,
    #[serde(rename = "defer-timeout")]
    pub defer_timeout: u32,
    #[serde(rename = "recover-lease")]
    pub recover_lease: bool,
    #[serde(rename = "refresh-lease")]
    pub refresh_lease: bool,
    #[serde(rename = "release-lease")]
    pub release_lease: bool,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv6Auto {
    pub enabled: bool,
    #[serde(deserialize_with = "comma_separated_string_deserialize")]
    pub update: Vec<String>,
}

// https://stackoverflow.com/questions/54006221/how-can-i-deserialize-a-comma-separated-json-string-as-a-vector-of-separate-stri
fn comma_separated_string_deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(str_sequence
        .split(',')
        .map(|item| item.to_owned())
        .collect())
}

impl From<Interface> for model::Connection {
    fn from(val: Interface) -> Self {
        let mut con = model::Connection::new(val.name.clone(), DeviceType::Ethernet);
        let base_connection = con.base_mut();
        base_connection.interface = val.name.clone();
        base_connection.ip_config = (&val).into();
        con
    }
}

impl From<&Interface> for IpConfig {
    fn from(val: &Interface) -> Self {
        let method4 = IpMethod::from_str(if val.ipv4.enabled && val.ipv4_static.is_some() {
            "manual"
        } else if !val.ipv4.enabled {
            "disabled"
        } else {
            "auto"
        })
        .unwrap();
        let method6 = IpMethod::from_str(if val.ipv6.enabled && val.ipv6_static.is_some() {
            "manual"
        // currently not implemented by agama
        // FIXME uncomment when implemented
        // } else if val.ipv6.enabled && val.ipv6_dhcp.is_some() && val.ipv6_dhcp.as_ref().unwrap().mode == "managed" {
        //     "dhcp"
        } else if !val.ipv6.enabled {
            "disabled"
        } else {
            "auto"
        })
        .unwrap();

        let mut addresses: Vec<IpInet> = vec![];
        if val.ipv4_static.is_some() {
            addresses.push(
                IpInet::from_str(val.ipv4_static.as_ref().unwrap().address.local.as_str()).unwrap(),
            );
        }
        if val.ipv6_static.is_some() {
            addresses.push(
                IpInet::from_str(val.ipv6_static.as_ref().unwrap().address.local.as_str()).unwrap(),
            );
        }

        IpConfig {
            addresses,
            method4,
            method6,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_interface_to_connection() {
        let static_interface = Interface {
            ipv4: Ipv4 {
                enabled: true,
                ..Default::default()
            },
            ipv4_static: Some(Ipv4Static {
                address: Address {
                    local: "127.0.0.1/8".to_string(),
                },
            }),
            ipv6: Ipv6 {
                enabled: true,
                ..Default::default()
            },
            ipv6_static: Some(Ipv6Static {
                address: Address {
                    local: "::1/128".to_string(),
                },
            }),
            ..Default::default()
        };

        let static_connection: model::Connection = static_interface.into();
        assert_eq!(static_connection.base().ip_config.method4, IpMethod::Manual);
        assert_eq!(
            static_connection.base().ip_config.addresses[0].to_string(),
            "127.0.0.1/8"
        );
        assert_eq!(static_connection.base().ip_config.method6, IpMethod::Manual);
        assert_eq!(
            static_connection.base().ip_config.addresses[1].to_string(),
            "::1"
        );
        assert_eq!(
            static_connection.base().ip_config.addresses[1]
                .network_length()
                .to_string(),
            "128"
        );
    }

    #[test]
    fn test_dhcp_interface_to_connection() {
        let static_interface = Interface {
            ipv4: Ipv4 {
                enabled: true,
                ..Default::default()
            },
            ipv6: Ipv6 {
                enabled: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let static_connection: model::Connection = static_interface.into();
        assert_eq!(static_connection.base().ip_config.method4, IpMethod::Auto);
        assert_eq!(static_connection.base().ip_config.method6, IpMethod::Auto);
        assert_eq!(static_connection.base().ip_config.addresses.len(), 0);
    }
}