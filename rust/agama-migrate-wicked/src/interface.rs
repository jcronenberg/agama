use agama_dbus_server::network::model::{self, IpAddress, IpMethod, Ipv4Config};
use agama_lib::network::types::DeviceType;
use serde::{Deserialize, Serialize};
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

impl From<Interface> for model::Connection {
    fn from(val: Interface) -> Self {
        let mut con = model::Connection::new(val.name.clone(), DeviceType::Ethernet);
        let base_connection = con.base_mut();
        base_connection.interface = val.name.clone();
        base_connection.ipv4 = val.into();
        con
    }
}

impl From<Interface> for Ipv4Config {
    fn from(val: Interface) -> Self {
        let method = if val.ipv4.enabled && val.ipv4_static.is_some() {
            "manual"
        } else if !val.ipv4.enabled {
            "disabled"
        } else {
            "auto"
        };

        let mut ipv4 = Ipv4Config::default();
        if val.ipv4_static.is_some() {
            ipv4.addresses =
                vec![IpAddress::from_str(val.ipv4_static.unwrap().address.local.as_str()).unwrap()]
        }
        ipv4.method = IpMethod::from_str(method).unwrap();

        ipv4
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
            ..Default::default()
        };

        let static_connection: model::Connection = static_interface.into();
        assert_eq!(static_connection.base().ipv4.method, IpMethod::Manual);
        assert_eq!(
            static_connection.base().ipv4.addresses[0].to_string(),
            "127.0.0.1/8"
        );
    }

    #[test]
    fn test_dhcp_interface_to_connection() {
        let static_interface = Interface {
            ipv4: Ipv4 {
                enabled: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let static_connection: model::Connection = static_interface.into();
        assert_eq!(static_connection.base().ipv4.method, IpMethod::Auto);
        assert_eq!(static_connection.base().ipv4.addresses.len(), 0);
    }
}
