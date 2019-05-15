extern crate serde;


use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct ConnectionSettings {
  pub host: String,
}

// --------------
//
// Clusters
//
// --------------
pub type ClusterList = Vec<String>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
  #[serde(skip)]
  pub name: String,
  pub service_url: String,
  pub service_url_tls: Option<String>,
  pub broker_service_url: String,
  pub broker_service_url_tls: Option<String>,
  pub peer_cluster_names: Option<Vec<String>>
}

impl Cluster {
  pub fn new(name: String, service_url: String, broker_service_url: String) -> Cluster {
    return Cluster{
      name, service_url, broker_service_url,
      service_url_tls: None,
      broker_service_url_tls: None,
      peer_cluster_names: None,
    }
  }

  pub fn to_json(&self) -> Result<String> {
    return serde_json::to_string_pretty(self)
  }
}

impl ToString for Cluster {
  fn to_string(&self) -> String {
    let mut res = format!("{} -- service: {}, broker: {}", self.name, self.service_url, self.broker_service_url);

    if let (Some(service_url_tls), Some(broker_service_url_tls)) = (self.service_url_tls.clone(), self.broker_service_url_tls.clone()) {
      res = format!("{}, serviceTls: {}, brokerTls: {}", res, service_url_tls, broker_service_url_tls);
    }

    if let Some(peer_cluster_names) = self.peer_cluster_names.clone() {
      res = format!("{} | {}", res, peer_cluster_names.join(","))
    }

    return res
  }
}


// --------------
//
// Tenants
//
// --------------

pub type TenantList = Vec<String>;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tenant {
  #[serde(skip)]
  pub name: String,
  pub admin_roles: Vec<String>,
  pub allowed_clusters: Vec<String>,
}