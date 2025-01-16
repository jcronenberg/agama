use crate::network::nm::dbus::VlanConfig;
use crate::network::nm::dbus::VlanProtocol;
use crate::network::{model::*, nm::error::NmError};
use agama_lib::dbus::{get_optional_property, get_property, NestedHash, OwnedNestedHash};
use std::{collections::HashMap, str::FromStr};
use zbus::zvariant::{self, Value};

use super::BridgeConfig;

pub const BRIDGE_KEY: &str = "bridge";
pub const BRIDGE_PORT_KEY: &str = "bridge-port";
pub const INFINIBAND_KEY: &str = "infiniband";
pub const TUN_KEY: &str = "tun";
pub const VLAN_KEY: &str = "vlan";

pub fn bridge_config_to_dbus(bridge: &BridgeConfig) -> HashMap<&str, zvariant::Value> {
    let mut hash = HashMap::new();

    hash.insert("stp", bridge.stp.into());
    if let Some(prio) = bridge.priority {
        hash.insert("priority", prio.into());
    }
    if let Some(fwd_delay) = bridge.forward_delay {
        hash.insert("forward-delay", fwd_delay.into());
    }
    if let Some(hello_time) = bridge.hello_time {
        hash.insert("hello-time", hello_time.into());
    }
    if let Some(max_age) = bridge.max_age {
        hash.insert("max-age", max_age.into());
    }
    if let Some(ageing_time) = bridge.ageing_time {
        hash.insert("ageing-time", ageing_time.into());
    }

    hash
}

pub fn bridge_config_from_dbus(conn: &OwnedNestedHash) -> Result<Option<BridgeConfig>, NmError> {
    let Some(bridge) = conn.get(BRIDGE_KEY) else {
        return Ok(None);
    };

    Ok(Some(BridgeConfig {
        stp: get_property(bridge, "stp")?,
        priority: get_optional_property(bridge, "priority")?,
        forward_delay: get_optional_property(bridge, "forward-delay")?,
        hello_time: get_optional_property(bridge, "hello-time")?,
        max_age: get_optional_property(bridge, "max-age")?,
        ageing_time: get_optional_property(bridge, "ageing-time")?,
    }))
}

pub fn bridge_port_config_to_dbus(
    bridge_port: &BridgePortConfig,
) -> HashMap<&str, zvariant::Value> {
    let mut hash = HashMap::new();

    if let Some(prio) = bridge_port.priority {
        hash.insert("priority", prio.into());
    }
    if let Some(pc) = bridge_port.path_cost {
        hash.insert("path-cost", pc.into());
    }

    hash
}

pub fn bridge_port_config_from_dbus(
    conn: &OwnedNestedHash,
) -> Result<Option<BridgePortConfig>, NmError> {
    let Some(bridge_port) = conn.get(BRIDGE_PORT_KEY) else {
        return Ok(None);
    };

    Ok(Some(BridgePortConfig {
        priority: get_optional_property(bridge_port, "priority")?,
        path_cost: get_optional_property(bridge_port, "path_cost")?,
    }))
}

pub fn infiniband_config_to_dbus(config: &InfinibandConfig) -> HashMap<&str, zvariant::Value> {
    let mut infiniband_config: HashMap<&str, zvariant::Value> = HashMap::from([
        (
            "transport-mode",
            Value::new(config.transport_mode.to_string()),
        ),
        ("p-key", Value::new(config.p_key.unwrap_or(-1))),
    ]);

    if let Some(parent) = &config.parent {
        infiniband_config.insert("parent", parent.into());
    }

    infiniband_config
}

pub fn infiniband_config_from_dbus(
    conn: &OwnedNestedHash,
) -> Result<Option<InfinibandConfig>, NmError> {
    let Some(infiniband) = conn.get(INFINIBAND_KEY) else {
        return Ok(None);
    };

    let mut config = InfinibandConfig {
        p_key: get_optional_property(infiniband, "p-key")?,
        parent: get_optional_property(infiniband, "parent")?,
        ..Default::default()
    };

    if let Some(transport_mode) = get_optional_property::<String>(infiniband, "transport-mode")? {
        config.transport_mode = InfinibandTransportMode::from_str(transport_mode.as_str())?;
    }

    Ok(Some(config))
}

pub fn tun_config_to_dbus(config: &TunConfig) -> HashMap<&str, zvariant::Value> {
    let mut tun_config: HashMap<&str, zvariant::Value> =
        HashMap::from([("mode", Value::new(config.mode.clone() as u32))]);

    if let Some(group) = &config.group {
        tun_config.insert("group", group.into());
    }

    if let Some(owner) = &config.owner {
        tun_config.insert("owner", owner.into());
    }

    tun_config
}

pub fn tun_config_from_dbus(conn: &OwnedNestedHash) -> Result<Option<TunConfig>, NmError> {
    let Some(tun) = conn.get(TUN_KEY) else {
        return Ok(None);
    };

    let mode = match get_property::<u32>(tun, "mode") {
        Ok(2) => TunMode::Tap,
        _ => TunMode::Tun,
    };

    Ok(Some(TunConfig {
        mode,
        group: get_optional_property(tun, "group")?,
        owner: get_optional_property(tun, "owner")?,
    }))
}

pub fn vlan_config_to_dbus(cfg: &VlanConfig) -> NestedHash {
    let vlan: HashMap<&str, zvariant::Value> = HashMap::from([
        ("id", cfg.id.into()),
        ("parent", cfg.parent.clone().into()),
        ("protocol", cfg.protocol.to_string().into()),
    ]);

    NestedHash::from([("vlan", vlan)])
}

pub fn vlan_config_from_dbus(conn: &OwnedNestedHash) -> Result<Option<VlanConfig>, NmError> {
    let Some(vlan) = conn.get(VLAN_KEY) else {
        return Ok(None);
    };

    let protocol = match get_property::<String>(vlan, "protocol") {
        Ok(protocol) => VlanProtocol::from_str(protocol.as_str()).unwrap_or_default(),
        _ => Default::default(),
    };

    Ok(Some(VlanConfig {
        id: get_property(vlan, "id")?,
        parent: get_property(vlan, "parent")?,
        protocol,
    }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::network::nm::dbus::{
        connection_from_dbus, connection_to_dbus,
        test::{build_base_connection, hi},
    };
    use uuid::Uuid;

    #[test]
    fn test_connection_from_dbus_infiniband() -> anyhow::Result<()> {
        let uuid = Uuid::new_v4().to_string();
        let connection_section = HashMap::from([hi("id", "ib0")?, hi("uuid", uuid)?]);

        let infiniband_section = HashMap::from([
            hi("p-key", 0x8001_i32)?,
            hi("parent", "ib0")?,
            hi("transport-mode", "datagram")?,
        ]);

        let dbus_conn = HashMap::from([
            ("connection".to_string(), connection_section),
            (INFINIBAND_KEY.to_string(), infiniband_section),
        ]);

        let connection = connection_from_dbus(dbus_conn).unwrap();
        let ConnectionConfig::Infiniband(infiniband) = &connection.config else {
            panic!("Wrong connection type")
        };
        assert_eq!(infiniband.p_key, Some(0x8001));
        assert_eq!(infiniband.parent, Some("ib0".to_string()));
        assert_eq!(infiniband.transport_mode, InfinibandTransportMode::Datagram);

        Ok(())
    }

    #[test]
    fn test_dbus_from_infiniband_connection() -> anyhow::Result<()> {
        let config = InfinibandConfig {
            p_key: Some(0x8002),
            parent: Some("ib1".to_string()),
            transport_mode: InfinibandTransportMode::Connected,
        };
        let mut infiniband = build_base_connection();
        infiniband.config = ConnectionConfig::Infiniband(config);
        let infiniband_dbus = connection_to_dbus(&infiniband, None);

        let infiniband = infiniband_dbus.get(INFINIBAND_KEY).unwrap();
        let p_key = infiniband.get("p-key").unwrap().downcast_ref::<i32>()?;
        assert_eq!(p_key, 0x8002);
        let parent: &str = infiniband.get("parent").unwrap().downcast_ref().unwrap();
        assert_eq!(parent, "ib1");
        let transport_mode: &str = infiniband
            .get("transport-mode")
            .unwrap()
            .downcast_ref()
            .unwrap();
        assert_eq!(
            transport_mode,
            InfinibandTransportMode::Connected.to_string()
        );

        Ok(())
    }
}
