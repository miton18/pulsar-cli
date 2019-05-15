extern crate reqwest;

use crate::models::{TenantList, Tenant};
use reqwest::{Client, Response, Url};
use std::error::Error;
use std::io;

/**
 * List all tenants
 */
pub fn list(host: String) -> Result<TenantList, Box<Error>> {
  let mut res: Response = Client::new()
    .get(&(host + "/admin/v2/tenants"))
    .send()?;

  if !res.status().is_success() {
    return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("error {}", res.status()))));
  }

  let body: TenantList = res.json()?;
  Ok(body)
}

/**
 * Get the specified tenant
 */
pub fn get(host: String, name: &str) -> Result<Tenant, Box<dyn Error>> {
  let url: Url = Url::parse(&(host + "/admin/v2/tenants/" + name))?;
  let mut res: Response = Client::new()
    .get(url)
    .send()?;

  if !res.status().is_success() {
    return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("error {}", res.status()))));
  }

  let mut body: Tenant = res.json()?;
  body.name = name.to_string();
  Ok(body)
}

/**
 * Create a new tenant
 */
pub fn create(host: String, tenant: Tenant) -> Result<Tenant, Box<dyn Error>> {
  if tenant.name.is_empty() {
    return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Tenant name connot be empty")));
  }

  let url: Url = Url::parse(&(host + "/admin/v2/tenants/" + &tenant.name))?;
    let res: Response = Client::new()
      .put(url)
      .json(&tenant)
      .send()?;

    if !res.status().is_success() {
      return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("error {}", res.status()))));
    }

    Ok(tenant)
}

/**
 * Create a new tenant
 */
pub fn update(host: String, tenant: Tenant) -> Result<Tenant, Box<dyn Error>> {
  if tenant.name.is_empty() {
    return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Tenant name connot be empty")));
  }

  let url: Url = Url::parse(&(host + "/admin/v2/tenants/" + &tenant.name))?;
    let res: Response = Client::new()
      .post(url)
      .body(serde_json::to_string(&tenant)?)
      .send()?;

    if !res.status().is_success() {
      return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("error {}", res.status()))));
    }

    Ok(tenant)
}

/**
 * Delete the specified tenant
 */
pub fn delete(host: String, name: &str) -> Result<(), Box<dyn Error>> {
  let url: Url = Url::parse(&(host + "/admin/v2/tenants/" + name))?;
  let res: Response = Client::new()
    .delete(url)
    .send()?;

  if !res.status().is_success() {
    return Err(Box::new(io::Error::new(io::ErrorKind::Other, format!("error {}", res.status()))));
  }

  Ok(())
}
