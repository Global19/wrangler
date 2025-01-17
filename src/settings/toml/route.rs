use serde::{Deserialize, Serialize};

use cloudflare::endpoints::workers::WorkersRoute;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Route {
    pub id: Option<String>,
    pub script: Option<String>,
    pub pattern: String,
}

impl From<&WorkersRoute> for Route {
    fn from(api_route: &WorkersRoute) -> Route {
        Route {
            id: Some(api_route.id.clone()),
            script: api_route.script.clone(),
            pattern: api_route.pattern.clone(),
        }
    }
}

#[derive(Debug)]
pub struct RouteConfig {
    pub workers_dev: Option<bool>,
    pub route: Option<String>,
    pub routes: Option<Vec<String>>,
    pub zone_id: Option<String>,
    pub account_id: Option<String>,
}

impl RouteConfig {
    pub fn has_routes_defined(&self) -> bool {
        if self.route.is_some() {
            true
        } else if let Some(routes) = &self.routes {
            !routes.is_empty()
        } else {
            false
        }
    }

    pub fn routes(&self) -> impl Iterator<Item = &String> {
        self.route.iter().chain(self.routes.iter().flatten())
    }

    pub fn is_zoneless(&self) -> bool {
        self.workers_dev.unwrap_or_default()
    }

    pub fn is_zoned(&self) -> bool {
        self.has_routes_defined() && self.zone_id.is_some()
    }

    pub fn workers_dev_false_by_itself(&self) -> bool {
        if let Some(workers_dev) = self.workers_dev {
            !workers_dev && !self.has_routes_defined()
        } else {
            false
        }
    }
}
