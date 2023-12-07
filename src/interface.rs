use agama_dbus_server::network::model::{self, IpConfig, IpRoute, Ipv4Method, Ipv6Method};
use cidr::IpInet;
use serde::{Deserialize, Serialize};
use serde_with::{
    formats::CommaSeparator, serde_as, skip_serializing_none, DeserializeFromStr, SerializeDisplay,
    StringWithSeparator,
};
use std::{net::IpAddr, str::FromStr};
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Interface {
    pub name: String,
    pub control: Control,
    pub firewall: Firewall,
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
    #[serde(rename = "@origin")]
    pub origin: String,
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

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv4Static {
    #[serde(rename = "address")]
    pub addresses: Option<Vec<Address>>,
    #[serde(rename = "route")]
    pub routes: Option<Vec<Route>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipv6Static {
    #[serde(rename = "address")]
    pub addresses: Option<Vec<Address>>,
    #[serde(rename = "route")]
    pub routes: Option<Vec<Route>>,
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

#[skip_serializing_none]
#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Route {
    pub destination: Option<String>,
    #[serde(rename = "nexthop")]
    pub nexthops: Option<Vec<Nexthop>>,
    pub priority: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Nexthop {
    pub gateway: String,
}

pub struct ConnectionResult {
    pub connection: model::Connection,
    pub warnings: Vec<anyhow::Error>,
}

pub struct IpConfigResult {
    ip_config: IpConfig,
    warnings: Vec<anyhow::Error>,
}

impl Interface {
    pub fn to_connection(&self) -> Result<ConnectionResult, anyhow::Error> {
        let ip_config = self.to_ip_config()?;
        let base = model::BaseConnection {
            id: self.name.clone(),
            interface: self.name.clone(),
            ip_config: ip_config.ip_config,
            ..Default::default()
        };

        Ok(ConnectionResult {
            connection: model::Connection::Ethernet(model::EthernetConnection { base }),
            warnings: ip_config.warnings,
        })
    }

    pub fn to_ip_config(&self) -> Result<IpConfigResult, anyhow::Error> {
        let mut connection_result = IpConfigResult {
            ip_config: IpConfig {
                ..Default::default()
            },
            warnings: vec![],
        };
        let method4 = if self.ipv4.enabled && self.ipv4_static.is_some() {
            Ipv4Method::Manual
        } else if !self.ipv4.enabled {
            Ipv4Method::Disabled
        } else {
            Ipv4Method::Auto
        };
        let method6 = if self.ipv6.enabled && self.ipv6_static.is_some() {
            Ipv6Method::Manual
        } else if self.ipv6.enabled
            && self.ipv6_dhcp.is_some()
            && self.ipv6_dhcp.as_ref().unwrap().mode == "managed"
        {
            Ipv6Method::Dhcp
        } else if !self.ipv6.enabled {
            Ipv6Method::Disabled
        } else {
            Ipv6Method::Auto
        };

        let mut addresses: Vec<IpInet> = vec![];
        let mut new_routes4: Vec<IpRoute> = vec![];
        let mut new_routes6: Vec<IpRoute> = vec![];
        if let Some(ipv4_static) = &self.ipv4_static {
            if let Some(addresses_in) = &ipv4_static.addresses {
                for addr in addresses_in {
                    addresses.push(IpInet::from_str(addr.local.as_str()).unwrap());
                }
            }
            if let Some(routes) = &ipv4_static.routes {
                for route in routes {
                    new_routes4.push(match route.try_into() {
                        Ok(route) => route,
                        Err(e) => {
                            connection_result.warnings.push(e);
                            continue;
                        }
                    });
                }
            }
        }
        if let Some(ipv6_static) = &self.ipv6_static {
            if let Some(addresses_in) = &ipv6_static.addresses {
                for addr in addresses_in {
                    addresses.push(IpInet::from_str(addr.local.as_str()).unwrap());
                }
            }
            if let Some(routes) = &ipv6_static.routes {
                for route in routes {
                    new_routes6.push(match route.try_into() {
                        Ok(route) => route,
                        Err(e) => {
                            connection_result.warnings.push(e);
                            continue;
                        }
                    });
                }
            }
        }

        let routes4 = if !new_routes4.is_empty() {
            Some(new_routes4)
        } else {
            None
        };
        let routes6 = if !new_routes6.is_empty() {
            Some(new_routes6)
        } else {
            None
        };

        connection_result.ip_config = IpConfig {
            addresses,
            method4,
            method6,
            routes4,
            routes6,
            ..Default::default()
        };
        Ok(connection_result)
    }
}

impl TryFrom<&Route> for IpRoute {
    type Error = anyhow::Error;
    fn try_from(route: &Route) -> Result<Self, Self::Error> {
        let mut next_hop: Option<IpAddr> = None;
        if let Some(nexthops) = &route.nexthops {
            if nexthops.len() > 1 {
                return Err(anyhow::anyhow!(
                    "Multipath routing isn't natively supported by NetworkManager"
                ));
            } else {
                next_hop = Some(IpAddr::from_str(&nexthops[0].gateway).unwrap());
            }
        }
        let destination = if route.destination.is_some() {
            IpInet::from_str(route.destination.clone().unwrap().as_str())?
        } else if next_hop.is_some() {
            // default route
            let default_ip = if next_hop.unwrap().is_ipv4() {
                IpAddr::from_str("0.0.0.0")?
            } else {
                IpAddr::from_str("::")?
            };
            IpInet::new(default_ip, 0)?
        } else {
            return Err(anyhow::anyhow!("Error occurred when parsing a route"));
        };
        let metric = route.priority;
        Ok(IpRoute {
            destination,
            next_hop,
            metric,
        })
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
                routes: Some(vec![Route {
                    nexthops: Some(vec![Nexthop {
                        gateway: "127.0.0.1".to_string(),
                    }]),
                    ..Default::default()
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
                routes: Some(vec![Route {
                    nexthops: Some(vec![Nexthop {
                        gateway: "::1".to_string(),
                    }]),
                    ..Default::default()
                }]),
            }),
            ..Default::default()
        };

        let static_connection: model::Connection =
            static_interface.to_connection().unwrap().connection;
        assert_eq!(
            static_connection.base().ip_config.method4,
            Ipv4Method::Manual
        );
        assert_eq!(
            static_connection.base().ip_config.addresses[0].to_string(),
            "127.0.0.1/8"
        );
        assert_eq!(
            static_connection.base().ip_config.method6,
            Ipv6Method::Manual
        );
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
        assert!(static_connection.base().ip_config.routes4.is_some());
        assert!(
            static_connection
                .base()
                .ip_config
                .routes4
                .clone()
                .unwrap()
                .len()
                == 1
        );
        assert_eq!(
            static_connection.base().ip_config.routes4.clone().unwrap()[0]
                .destination
                .to_string(),
            "0.0.0.0/0"
        );
        assert_eq!(
            static_connection.base().ip_config.routes4.clone().unwrap()[0]
                .next_hop
                .unwrap()
                .to_string(),
            "127.0.0.1"
        );
        assert!(static_connection.base().ip_config.routes6.is_some());
        assert!(
            static_connection
                .base()
                .ip_config
                .routes6
                .clone()
                .unwrap()
                .len()
                == 1
        );
        assert_eq!(
            static_connection.base().ip_config.routes6.clone().unwrap()[0]
                .destination
                .to_string(),
            "::/0"
        );
        assert_eq!(
            static_connection.base().ip_config.routes6.clone().unwrap()[0]
                .next_hop
                .unwrap()
                .to_string(),
            "::1"
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

        let static_connection: model::Connection =
            static_interface.to_connection().unwrap().connection;
        assert_eq!(static_connection.base().ip_config.method4, Ipv4Method::Auto);
        assert_eq!(static_connection.base().ip_config.method6, Ipv6Method::Auto);
        assert_eq!(static_connection.base().ip_config.addresses.len(), 0);
    }
}
