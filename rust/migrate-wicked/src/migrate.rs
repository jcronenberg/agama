use crate::bridge::BridgePort;
use crate::{reader::read as wicked_read, MIGRATION_SETTINGS};
use agama_dbus_server::network::{model, Adapter, NetworkManagerAdapter, NetworkState};
use async_trait::async_trait;
use std::{collections::HashMap, error::Error};
use uuid::Uuid;

struct WickedAdapter {
    paths: Vec<String>,
    current_state: NetworkState,
}

impl WickedAdapter {
    pub fn new(paths: Vec<String>, current_state: NetworkState) -> Self {
        Self {
            paths,
            current_state,
        }
    }
}

fn update_parent_connection(
    state: &mut model::NetworkState,
    parents: HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let settings = MIGRATION_SETTINGS.get().unwrap();
    let mut parent_uuid: HashMap<String, Uuid> = HashMap::new();

    for (id, parent) in parents {
        if let Some(parent_con) = state.get_connection_by_interface(&parent) {
            parent_uuid.insert(id, parent_con.uuid);
        } else {
            log::warn!("Missing parent {} connection for {}", parent, id);
            if !settings.continue_migration {
                return Err(anyhow::anyhow!("Migration of {} failed", id).into());
            }
        }
    }

    for (id, uuid) in parent_uuid {
        if let Some(connection) = state.get_connection_mut(&id) {
            connection.controller = Some(uuid);
        } else {
            return Err(anyhow::anyhow!("Unexpected failure - missing connection {}", id).into());
        }
    }

    Ok(())
}

fn update_bridge_ports(
    state: &mut model::NetworkState,
    portconfigs: HashMap<String, BridgePort>,
) -> Result<(), Box<dyn std::error::Error>> {
    let settings = MIGRATION_SETTINGS.get().unwrap();

    for (ifc, portconfig) in portconfigs {
        let id;
        if let Some(c) = state.get_connection_by_interface(&ifc) {
            id = c.id.clone();
        } else {
            log::warn!("Missing bridge port {} connection", ifc);
            if !settings.continue_migration {
                return Err(anyhow::anyhow!(
                    "Migration failed, because bridge is missing port {}",
                    ifc
                )
                .into());
            }
            continue;
        }

        let c = state.get_connection_mut(&id).unwrap();
        c.port_config = (&portconfig).into();
    }

    Ok(())
}

#[async_trait]
impl Adapter for WickedAdapter {
    async fn read(&self) -> Result<model::NetworkState, Box<dyn std::error::Error>> {
        let interfaces = wicked_read(self.paths.clone())?;
        let settings = MIGRATION_SETTINGS.get().unwrap();
        let mut parents: HashMap<String, String> = HashMap::new();
        let mut bridge_ports: HashMap<String, BridgePort> = HashMap::new();

        if !settings.continue_migration && interfaces.warning.is_some() {
            return Err(interfaces.warning.unwrap().into());
        }

        let mut state = NetworkState::new(vec![], vec![]);

        for interface in interfaces.interfaces {
            let connection_result = interface.to_connection()?;
            if !connection_result.warnings.is_empty() {
                for connection_error in &connection_result.warnings {
                    log::warn!("{}", connection_error);
                }
                if !settings.continue_migration {
                    return Err(anyhow::anyhow!(
                        "Migration of {} failed",
                        connection_result.connection.id
                    )
                    .into());
                }
            }

            if let Some(parent) = interface.link.master {
                parents.insert(connection_result.connection.id.clone(), parent.clone());
            }
            if let Some(bridge) = interface.bridge {
                for port in bridge.ports {
                    bridge_ports.insert(port.device.clone(), port.clone());
                }
            }
            state.add_connection(connection_result.connection)?;
        }

        if let Some(netconfig) = interfaces.netconfig {
            let mut loopback = self.current_state.get_connection("lo").unwrap().clone();
            loopback.ip_config.nameservers = match netconfig.static_dns_servers() {
                Ok(nameservers) => nameservers,
                Err(e) => {
                    let error = anyhow::anyhow!("Error when parsing static DNS servers: {}", e);
                    if !settings.continue_migration {
                        return Err(error.into());
                    } else {
                        log::warn!("{}", error);
                        vec![]
                    }
                }
            };
            if let Some(static_dns_searchlist) = netconfig.static_dns_searchlist {
                loopback.ip_config.dns_searchlist = static_dns_searchlist;
            }
            state.add_connection(loopback)?;
        }

        update_parent_connection(&mut state, parents)?;
        update_bridge_ports(&mut state, bridge_ports)?;

        Ok(state)
    }

    async fn write(
        &self,
        _network: &model::NetworkState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!("not needed");
    }
}

pub async fn migrate(paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    let nm = NetworkManagerAdapter::from_system().await?;
    let current_state = nm.read().await?;
    let wicked = WickedAdapter::new(paths, current_state);
    let state = wicked.read().await?;
    let settings = MIGRATION_SETTINGS.get().unwrap();
    if settings.dry_run {
        for connection in state.connections {
            log::debug!("{:#?}", connection);
        }
        return Ok(());
    }
    nm.write(&state).await?;
    Ok(())
}
