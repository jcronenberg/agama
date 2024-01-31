use crate::MIGRATION_SETTINGS;
use agama_lib::network::types::SSID;
use agama_server::network::model::{self, WEPAuthAlg, WEPKeyType, WEPSecurity};
use anyhow::anyhow;
use macaddr::MacAddr6;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::formats::CommaSeparator;
use serde_with::StringWithSeparator;
use serde_with::{serde_as, skip_serializing_none, DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Wireless {
    #[serde(rename = "ap-scan")]
    pub ap_scan: u32,
    #[serde(default)]
    #[serde(deserialize_with = "unwrap_wireless_networks")]
    pub networks: Option<Vec<Network>>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Network {
    pub essid: String,
    #[serde(rename = "scan-ssid")]
    pub scan_ssid: bool,
    pub mode: WickedWirelessMode,
    #[serde(rename = "wpa-psk")]
    pub wpa_psk: Option<WpaPsk>,
    #[serde(default)]
    #[serde(rename = "key-management")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub key_management: Vec<String>,
    pub channel: Option<u32>,
    #[serde(rename = "access-point")]
    pub access_point: Option<String>,
    pub wep: Option<Wep>,
    #[serde(rename = "wpa-eap")]
    pub wpa_eap: Option<WpaEap>,
}

#[derive(Default, Debug, PartialEq, SerializeDisplay, DeserializeFromStr, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum WickedWirelessMode {
    AdHoc = 0,
    #[default]
    Infrastructure = 1,
    AP = 2,
}

impl From<&WickedWirelessMode> for model::WirelessMode {
    fn from(value: &WickedWirelessMode) -> Self {
        match value {
            WickedWirelessMode::AdHoc => model::WirelessMode::AdHoc,
            WickedWirelessMode::Infrastructure => model::WirelessMode::Infra,
            WickedWirelessMode::AP => model::WirelessMode::AP,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WpaPsk {
    pub passphrase: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Wep {
    #[serde(rename = "auth-algo")]
    pub auth_algo: String,
    #[serde(rename = "default-key")]
    pub default_key: u32,
    pub key: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WpaEap {
    pub method: EapMethod,
    #[serde(rename = "auth-proto")]
    pub auth_proto: EapAuthProto,
    #[serde(rename = "pairwise-cipher")]
    pub pairwise_cipher: EapPairwiseCipher,
    #[serde(rename = "group-cipher")]
    pub group_cipher: EapGroupCipher,
    pub identity: String,
    pub tls: Option<WickedTLS>,
}

#[derive(Default, Debug, PartialEq, SerializeDisplay, DeserializeFromStr, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum EapMethod {
    #[default]
    TLS,
    PEAP,
    TTLS,
}

#[derive(Default, Debug, PartialEq, SerializeDisplay, DeserializeFromStr, EnumString, Display)]
// TODO i don't think this is correct
// but tbh this is probably overkill anyway
#[strum(serialize_all = "kebab-case")]
pub enum EapAuthProto {
    #[default]
    WPA,
    NONE,
    MD5,
    TLS,
    PAP,
    CHAP,
    MSCHAP,
    MSCHAPV2,
    PEAP,
    TTLS,
    GTC,
    OTP,
    LEAP,
    PSK,
    PAX,
    SAKE,
    GPSK,
    WSC,
    IKEV2,
    TNC,
    FAST,
    AKA,
    AkaPrime,
    SIM,
}

// TODO will have to look into wicked code into what options the "inner" and "outer" get translated
impl TryFrom<EapAuthProto> for model::EAPMethod {
    type Error = anyhow::Error;

    fn try_from(value: EapAuthProto) -> Result<Self, Self::Error> {
        match value {
            EapAuthProto::LEAP => Ok(model::EAPMethod::LEAP),
            EapAuthProto::MD5 => Ok(model::EAPMethod::MD5),
            EapAuthProto::TLS => Ok(model::EAPMethod::TLS),
            EapAuthProto::PEAP => Ok(model::EAPMethod::PEAP),
            EapAuthProto::TTLS => Ok(model::EAPMethod::TTLS),
            EapAuthProto::FAST => Ok(model::EAPMethod::FAST),
            _ => Err(anyhow!("EAP auth-proto isn't supported by NetworkManager")),
        }
    }
}

#[derive(Default, Debug, PartialEq, SerializeDisplay, DeserializeFromStr, EnumString, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum EapPairwiseCipher {
    #[default]
    TKIP,
    CCMP,
}

#[derive(Default, Debug, PartialEq, SerializeDisplay, DeserializeFromStr, EnumString, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum EapGroupCipher {
    #[default]
    TKIP,
    CCMP,
    WEP104,
    WEP40,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WickedTLS {
    #[serde(rename = "ca-cert")]
    pub ca_cert: String,
    #[serde(rename = "client-cert")]
    pub client_cert: String,
    #[serde(rename = "client-key")]
    pub client_key: String,
    #[serde(rename = "client-key-passwd")]
    pub client_key_passwd: String,
}

fn unwrap_wireless_networks<'de, D>(deserializer: D) -> Result<Option<Vec<Network>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
    struct Networks {
        network: Vec<Network>,
    }
    Ok(Some(Networks::deserialize(deserializer)?.network))
}

fn wireless_security_protocol(
    wicked_value: &[String],
) -> Result<model::SecurityProtocol, anyhow::Error> {
    if wicked_value.contains(&"wpa-psk".to_string())
        || wicked_value.contains(&"wpa-psk-sha256".to_string())
    {
        Ok(model::SecurityProtocol::WPA2)
    } else if wicked_value.contains(&"sae".to_string()) {
        Ok(model::SecurityProtocol::WPA3Personal)
    } else if wicked_value.contains(&"wpa-eap".to_string())
        || wicked_value.contains(&"wpa-eap-sha256".to_string())
    {
        Ok(model::SecurityProtocol::WPA2Enterprise)
    } else if wicked_value.contains(&"owe".to_string()) {
        Ok(model::SecurityProtocol::OWE)
    } else if wicked_value.contains(&"wpa-eap-suite-b-192".to_string()) {
        Ok(model::SecurityProtocol::WPA3Only)
    } else if wicked_value.contains(&"none".to_string()) {
        Ok(model::SecurityProtocol::WEP)
    } else {
        Err(anyhow!("Unrecognized key-management protocol"))
    }
}

impl TryFrom<&Network> for model::ConnectionConfig {
    type Error = anyhow::Error;
    fn try_from(network: &Network) -> Result<Self, Self::Error> {
        let settings = MIGRATION_SETTINGS.get().unwrap();
        let mut config = model::WirelessConfig {
            ssid: SSID(network.essid.as_bytes().to_vec()),
            hidden: network.scan_ssid,
            ..Default::default()
        };

        if network.key_management.len() > 1 && settings.continue_migration {
            log::warn!("Migration of multiple key-management algorithms isn't supported")
        } else if network.key_management.len() > 1 {
            return Err(anyhow!(
                "Migration of multiple key-management algorithms isn't supported"
            ));
        }
        config.security = wireless_security_protocol(&network.key_management)?;

        if let Some(wpa_psk) = &network.wpa_psk {
            config.password = Some(wpa_psk.passphrase.clone())
        }
        if let Some(channel) = network.channel {
            config.channel = Some(channel);
            if channel <= 14 {
                config.band = Some("bg".try_into().unwrap());
            } else {
                config.band = Some("a".try_into().unwrap());
            }
            log::warn!(
                "NetworkManager requires setting a band for wireless when a channel is set. The band has been set to \"{}\". This may in certain regions be incorrect.",
                config.band.unwrap()
            );
        }
        if let Some(access_point) = &network.access_point {
            config.bssid = Some(MacAddr6::from_str(access_point)?);
        }

        if let Some(wep) = &network.wep {
            // filter out `s:`, `h:`, `:`, and `-` of wep keys
            let keys: Vec<String> = wep
                .key
                .clone()
                .into_iter()
                .map(|mut x| {
                    x = x.replace("s:", "");
                    x = x.replace("h:", "");
                    x = x.replace(':', "");
                    x.replace('-', "")
                })
                .collect();
            let wep_security = WEPSecurity {
                auth_alg: WEPAuthAlg::try_from(wep.auth_algo.as_str())?,
                wep_key_type: WEPKeyType::Key,
                keys,
                wep_key_index: wep.default_key,
            };
            config.wep_security = Some(wep_security);
        }

        config.mode = (&network.mode).into();
        Ok(model::ConnectionConfig::Wireless(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::*;
    use crate::MIGRATION_SETTINGS;

    #[allow(dead_code)]
    fn setup_default_migration_settings() {
        let _ = MIGRATION_SETTINGS.set(crate::MigrationSettings {
            continue_migration: false,
            dry_run: false,
            activate_connections: true,
        });
    }

    #[test]
    fn test_wireless_bands() {
        setup_default_migration_settings();
        let mut wireless_interface = Interface {
            wireless: Some(Wireless {
                networks: Some(vec![Network {
                    channel: Some(0),
                    essid: "testssid".to_string(),
                    scan_ssid: false,
                    mode: WickedWirelessMode::AP,
                    wpa_psk: None,
                    key_management: vec!["wpa-psk".to_string()],
                    access_point: None,
                    wep: None,
                    wpa_eap: None,
                }]),
                ap_scan: 0,
            }),
            ..Default::default()
        };
        let connections = wireless_interface.to_connection();
        assert!(connections.is_ok());
        let connection = &connections.unwrap().connections[0];
        let model::ConnectionConfig::Wireless(wireless) = &connection.config else {
            panic!()
        };
        assert_eq!(wireless.band, Some("bg".try_into().unwrap()));

        wireless_interface
            .wireless
            .as_mut()
            .unwrap()
            .networks
            .as_mut()
            .unwrap()[0]
            .channel = Some(32);
        let ifc = wireless_interface.to_connection();
        assert!(ifc.is_ok());
        let ifc = &ifc.unwrap().connections[0];
        let model::ConnectionConfig::Wireless(wireless) = &ifc.config else {
            panic!()
        };
        assert_eq!(wireless.band, Some("a".try_into().unwrap()));
    }

    #[test]
    fn test_wireless_migration() {
        setup_default_migration_settings();
        let wireless_interface = Interface {
            wireless: Some(Wireless {
                networks: Some(vec![Network {
                    essid: "testssid".to_string(),
                    scan_ssid: true,
                    mode: WickedWirelessMode::Infrastructure,
                    wpa_psk: Some(WpaPsk {
                        passphrase: "testpassword".to_string(),
                    }),
                    key_management: vec!["wpa-psk".to_string()],
                    channel: Some(14),
                    access_point: Some("12:34:56:78:9A:BC".to_string()),
                    wep: Some(Wep {
                        auth_algo: "open".to_string(),
                        default_key: 1,
                        key: vec!["01020304ff".to_string(), "s:hello".to_string()],
                    }),
                    wpa_eap: None,
                }]),
                ap_scan: 0,
            }),
            ..Default::default()
        };
        let connections = wireless_interface.to_connection();
        assert!(connections.is_ok());
        let connection = &connections.unwrap().connections[0];
        let model::ConnectionConfig::Wireless(wireless) = &connection.config else {
            panic!()
        };
        assert_eq!(wireless.ssid, SSID("testssid".as_bytes().to_vec()));
        assert!(wireless.hidden);
        assert_eq!(wireless.mode, model::WirelessMode::Infra);
        assert_eq!(wireless.password, Some("testpassword".to_string()));
        assert_eq!(wireless.security, model::SecurityProtocol::WPA2);
        assert_eq!(
            wireless.bssid,
            Some(MacAddr6::from_str("12:34:56:78:9A:BC").unwrap())
        );
        assert_eq!(
            wireless.wep_security,
            Some(WEPSecurity {
                auth_alg: WEPAuthAlg::Open,
                wep_key_type: WEPKeyType::Key,
                keys: vec!["01020304ff".to_string(), "hello".to_string()],
                wep_key_index: 1,
            })
        );
        assert_eq!(wireless.band, Some("bg".try_into().unwrap()));
    }
}
