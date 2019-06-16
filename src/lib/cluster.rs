extern crate reqwest;

use crate::models::{Cluster, ClusterList};
use reqwest::{Client, Response, Url};
use std::error::Error;
use std::io;

/**
 * List all clusters
 */
pub fn list(host: String) -> Result<ClusterList, Box<Error>> {
  let mut res: Response = Client::new()
    .get(&(host + "/admin/v2/clusters"))
    .send()?;

  if !res.status().is_success() {
    return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("error {}", res.status()))));
  }

  let body: ClusterList = res.json()?;
  Ok(body)
}

/**
 * Get the specified cluster
 */
pub fn get(host: String, name: &str) -> Result<Cluster, Box<dyn Error>> {
  let url: Url = Url::parse(&(host + "/admin/v2/clusters/" + name))?;
  let mut res: Response = Client::new()
    .get(url)
    .send()?;

  if !res.status().is_success() {
    return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("error {}", res.status()))));
  }

  let mut body: Cluster = res.json()?;
  body.name = name.to_string();
  Ok(body)
}

/**
 * Create a new cluster
 */
pub fn create(_host: String, _name: String, _url: String, _broker_url: String) -> Result<Cluster, Box<dyn Error>> {
  Ok(Cluster::new("".to_string(), "".to_string(), "".to_string()))
}

/// Return cluster failure domains
pub fn get_failure_domains(_host: String, _name: &str) -> Result<(), Box<dyn Error>> {
  Ok(())
}