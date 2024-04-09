use crate::bridge::BridgePort;
use crate::{reader::read as wicked_read, MIGRATION_SETTINGS};
use agama_server::network::model::Connection;
use agama_server::network::{Adapter, NetworkManagerAdapter, NetworkState};
use std::{collections::HashMap, error::Error};
use uuid::Uuid;

fn update_parent_connection(
    connections: &mut [Connection],
    parents: HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    let settings = MIGRATION_SETTINGS.get().unwrap();
    let mut parent_uuid: HashMap<String, Uuid> = HashMap::new();

    for (id, parent) in parents {
        if let Some(parent_con) = connections
            .iter()
            .find(|c| c.interface.as_deref() == Some(&parent))
        {
            parent_uuid.insert(id, parent_con.uuid);
        } else {
            log::warn!("Missing parent {} connection for {}", parent, id);
            if !settings.continue_migration {
                return Err(anyhow::anyhow!("Migration of {} failed", id));
            }
        }
    }

    for (id, uuid) in parent_uuid {
        if let Some(connection) = connections
            .iter_mut()
            .find(|c| c.interface.as_deref() == Some(&id))
        {
            connection.controller = Some(uuid);
        } else {
            return Err(anyhow::anyhow!(
                "Unexpected failure - missing connection {}",
                id
            ));
        }
    }

    Ok(())
}

fn update_bridge_ports(
    connections: &mut [Connection],
    portconfigs: HashMap<String, BridgePort>,
) -> Result<(), anyhow::Error> {
    let settings = MIGRATION_SETTINGS.get().unwrap();

    for (ifc, portconfig) in portconfigs {
        let id;
        if let Some(c) = connections
            .iter()
            .find(|c| c.interface.as_deref() == Some(&ifc))
        {
            id = c.id.clone();
        } else {
            log::warn!("Missing bridge port {} connection", ifc);
            if !settings.continue_migration {
                return Err(anyhow::anyhow!(
                    "Migration failed, because bridge is missing port {}",
                    ifc
                ));
            }
            continue;
        }

        let c = connections.iter_mut().find(|c| c.id == id).unwrap();
        c.port_config = (&portconfig).into();
    }

    Ok(())
}

pub async fn migrate(paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    let interfaces = wicked_read(paths.clone())?;
    let settings = MIGRATION_SETTINGS.get().unwrap();
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut bridge_ports: HashMap<String, BridgePort> = HashMap::new();
    let mut connections: Vec<Connection> = vec![];

    if !settings.continue_migration && interfaces.warning.is_some() {
        return Err(interfaces.warning.unwrap().into());
    }

    for interface in interfaces.interfaces {
        let connection_result = interface.to_connection()?;
        if !connection_result.warnings.is_empty() {
            for connection_error in &connection_result.warnings {
                log::warn!("{}", connection_error);
            }
            if !settings.continue_migration {
                return Err(anyhow::anyhow!(
                    "Migration of {} failed",
                    connection_result.connections[0].id
                )
                .into());
            }
        }

        for connection in connection_result.connections {
            if let Some(parent) = &interface.link.master {
                parents.insert(connection.id.clone(), parent.clone());
            }
            connections.push(connection);
            if let Some(bridge) = &interface.bridge {
                for port in &bridge.ports {
                    bridge_ports.insert(port.device.clone(), port.clone());
                }
            }
        }
    }

    update_parent_connection(&mut connections, parents)?;
    update_bridge_ports(&mut connections, bridge_ports)?;

    let mut state = NetworkState::new(vec![], vec![]);
    for connection in &connections {
        state.add_connection(connection.clone())?;
    }

    if settings.dry_run {
        for connection in state.connections {
            log::debug!("{:#?}", connection);
        }
        return Ok(());
    }
    let nm = NetworkManagerAdapter::from_system().await?;
    nm.write(&state).await?;
    Ok(())
}
