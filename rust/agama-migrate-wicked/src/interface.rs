use agama_dbus_server::network::model::{self, IpConfig, IpMethod, Parent};
use cidr::IpInet;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{
    formats::CommaSeparator, serde_as, skip_serializing_none, DeserializeFromStr, SerializeDisplay,
    StringWithSeparator,
};
use std::{collections::HashMap, str::FromStr};
use strum_macros::{Display, EnumString};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond: Option<Bond>,
    #[serde(rename = "@origin")]
    pub origin: String,
}

#[derive(Debug, PartialEq, Serialize, Clone, Deserialize)]
pub enum ParentKind {
    Bond,
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
pub struct Link {
    #[serde(rename = "master", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ParentKind>,
}

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
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv6Static {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Address {
    pub local: String,
}

#[serde_as]
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv6Dhcp {
    pub enabled: bool,
    pub flags: String,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
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

#[serde_as]
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ipv6Auto {
    pub enabled: bool,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub update: Vec<String>,
}

#[derive(Debug, PartialEq, SerializeDisplay, DeserializeFromStr, EnumString, Display)]
#[strum(serialize_all = "kebab_case")]
pub enum FailOverMac {
    None,
    Active,
    Follow,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[skip_serializing_none]
pub struct Bond {
    pub mode: String,
    pub miimon: Option<Miimon>,
    pub arpmon: Option<ArpMon>,
    #[serde(deserialize_with = "unwrap_slaves")]
    pub slaves: Vec<Slave>,
    #[serde(rename = "fail-over-mac")]
    pub fail_over_mac: Option<FailOverMac>,
}

impl Bond {
    pub fn primary(self: &Bond) -> Option<&String> {
        for s in self.slaves.iter() {
            if s.primary.is_some() && s.primary.unwrap() {
                return Some(&s.device);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Slave {
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Miimon {
    pub frequency: u32,
    #[serde(rename = "carrier-detect")]
    pub carrier_detect: String,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArpMon {
    pub interval: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<String>,
    #[serde(rename = "validate-target")]
    pub validate_target: Option<String>,
    pub targets: Vec<ArpMonTargetAddressV4>,
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ArpMonTargetAddressV4 {
    #[serde(rename = "ipv4-address")]
    pub ipv4_address: String,
}

fn unwrap_slaves<'de, D>(deserializer: D) -> Result<Vec<Slave>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    struct Slaves {
        slave: Vec<Slave>,
    }
    Ok(Slaves::deserialize(deserializer)?.slave)
}

impl From<Bond> for HashMap<String, String> {
    fn from(bond: Bond) -> HashMap<String, String> {
        let mut h: HashMap<String, String> = HashMap::new();

        h.insert(String::from("mode"), bond.mode.clone());
        if let Some(p) = bond.primary() {
            h.insert(String::from("primary"), p.clone());
        }

        if let Some(m) = &bond.miimon {
            h.insert(String::from("miimon"), format!("{}", m.frequency));
        }

        if let Some(a) = &bond.arpmon {
            h.insert(String::from("arp_interval"), format!("{}", a.interval));
            if let Some(v) = &a.validate {
                h.insert(String::from("arp_validate"), v.clone());
            }

            if !a.targets.is_empty() {
                let sv = a
                    .targets
                    .iter()
                    .map(|c| c.ipv4_address.as_ref())
                    .collect::<Vec<&str>>()
                    .join(",");
                h.insert(String::from("arp_ip_target"), sv);
            }

            if let Some(v) = &a.validate_target {
                h.insert(String::from("arp_all_targets"), v.clone());
            }
        }

        if let Some(fom) = &bond.fail_over_mac {
            h.insert(String::from("fail-over-mac"), fom.to_string());
        }
        h
    }
}

impl From<Interface> for model::Connection {
    fn from(ifc: Interface) -> model::Connection {
        let mut base = model::BaseConnection {
            id: ifc.name.clone(),
            interface: ifc.name.clone(),
            ip_config: (&ifc).into(),
            ..Default::default()
        };

        if ifc.link.kind.is_some() && ifc.link.parent.is_some() {
            let interface = ifc.link.parent.clone().unwrap();
            let kind = match ifc.link.kind {
                Some(p) => match &p {
                    ParentKind::Bond => model::ParentKind::Bond,
                },
                None => panic!("Missing ParentType"),
            };
            base.parent = Some(Parent { interface, kind });
        }

        if let Some(b) = ifc.bond {
            model::Connection::Bond(model::BondConnection {
                base,
                bond: model::BondConfig { options: b.into() },
            })
        } else {
            model::Connection::Ethernet(model::EthernetConnection { base })
        }
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
        if let Some(ipv4_static) = &val.ipv4_static {
            if let Some(addresses_in) = &ipv4_static.addresses {
                for addr in addresses_in {
                    addresses.push(IpInet::from_str(addr.local.as_str()).unwrap());
                }
            }
        }
        if let Some(ipv6_static) = &val.ipv6_static {
            if let Some(addresses_in) = &ipv6_static.addresses {
                for addr in addresses_in {
                    addresses.push(IpInet::from_str(addr.local.as_str()).unwrap());
                }
            }
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
                addresses: Some(vec![Address {
                    local: "127.0.0.1/8".to_string(),
                }]),
            }),
            ipv6: Ipv6 {
                enabled: true,
                ..Default::default()
            },
            ipv6_static: Some(Ipv6Static {
                addresses: Some(vec![Address {
                    local: "::1/128".to_string(),
                }]),
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

    #[test]
    fn test_bond_options() {
        let bond = Bond {
            fail_over_mac: Some(FailOverMac::Active),
            ..Default::default()
        };

        let bond: HashMap<String, String> = bond.into();
        assert!(bond.contains_key("fail-over-mac"));
        assert_eq!(bond.get("fail-over-mac").unwrap(), "active");
    }
}
