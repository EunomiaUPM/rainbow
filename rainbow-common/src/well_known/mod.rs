use crate::well_known::dspace_version::WellKnownDSpaceVersionService;
use crate::well_known::router::WellKnownRouter;

mod dspace_version;
pub mod router;

pub struct WellKnownRoot;
impl WellKnownRoot {
    pub fn get_router() -> anyhow::Result<axum::Router> {
        let dspace_version_service = WellKnownDSpaceVersionService {};
        let router = WellKnownRouter::new(dspace_version_service);
        Ok(router.router())
    }
}
