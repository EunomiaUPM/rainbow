use axum::Router;
use rainbow_catalog::core::ds_protocol::ds_protocol::DSProtocolCatalogService;
use rainbow_catalog::core::rainbow_entities::catalog::RainbowCatalogCatalogService;
use rainbow_catalog::core::rainbow_entities::data_service::RainbowCatalogDataServiceService;
use rainbow_catalog::core::rainbow_entities::dataset::RainbowCatalogDatasetService;
use rainbow_catalog::core::rainbow_entities::distribution::RainbowCatalogDistributionService;
use rainbow_catalog::core::rainbow_entities::policies::RainbowCatalogPoliciesService;
use rainbow_catalog::core::rainbow_rpc::rainbow_rpc::RainbowRPCCatalogService;
use rainbow_catalog::http::rainbow_entities::catalog::RainbowCatalogCatalogRouter;
use rainbow_catalog::http::rainbow_entities::data_service::RainbowCatalogDataServiceRouter;
use rainbow_catalog::http::rainbow_entities::dataset::RainbowCatalogDatasetRouter;
use rainbow_catalog::http::rainbow_entities::distribution::RainbowCatalogDistributionRouter;
use rainbow_catalog::http::rainbow_entities::policies::RainbowCatalogPoliciesRouter;
use rainbow_catalog::http::rainbow_rpc::rainbow_rpc::RainbowRPCCatalogRouter;
use rainbow_contracts::provider::core::ds_protocol::ds_protocol::DSProtocolContractNegotiationProviderService;
use rainbow_contracts::provider::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationProviderService;
use rainbow_contracts::provider::core::rainbow_entities::rainbow_entities::RainbowEntitiesContractNegotiationProviderService;
use rainbow_contracts::provider::http::ds_protocol::ds_protocol::DSProtocolContractNegotiationProviderRouter;
use rainbow_contracts::provider::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCContractNegotiationProviderRouter;
use rainbow_contracts::provider::http::rainbow_entities::rainbow_entities::RainbowEntitesContractNegotiationProviderRouter;
use rainbow_db::catalog::repo::sql::CatalogRepoForSql;
use rainbow_db::catalog::repo::CatalogRepoFactory;
use rainbow_db::contracts_provider::repo::sql::ContractNegotiationProviderRepoForSql;
use rainbow_db::contracts_provider::repo::ContractNegotiationProviderRepoFactory;
use rainbow_db::events::repo::sql::EventsRepoForSql;
use rainbow_db::events::repo::EventsRepoFactory;
use rainbow_db::transfer_provider::repo::sql::TransferProviderRepoForSql;
use rainbow_db::transfer_provider::repo::TransferProviderRepoFactory;
use rainbow_events::core::notification::notification::RainbowEventsNotificationsService;
use rainbow_events::core::subscription::subscription::RainbowEventsSubscriptionService;
use rainbow_events::http::notification::notification::RainbowEventsNotificationRouter;
use rainbow_events::http::subscription::subscription::RainbowEventsSubscriptionRouter;
use rainbow_transfer::provider::core::data_plane_facade::data_plane_facade::DataPlaneProviderFacadeImpl;
use rainbow_transfer::provider::core::data_service_resolver_facade::data_service_resolver_facade::DataServiceFacadeImpl;
use rainbow_transfer::provider::core::ds_protocol::ds_protocol::DSProtocolTransferProviderImpl;
use rainbow_transfer::provider::core::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferProviderService;
use rainbow_transfer::provider::core::rainbow_entities::rainbow_entities::RainbowTransferProviderServiceImpl;
use rainbow_transfer::provider::http::ds_protocol::ds_protocol::DSProtocolTransferProviderRouter;
use rainbow_transfer::provider::http::ds_protocol_rpc::ds_protocol_rpc::DSRPCTransferProviderProviderRouter;
use rainbow_transfer::provider::http::rainbow_entities::rainbow_entities::RainbowTransferProviderEntitiesRouter;
use sea_orm::Database;
use std::sync::Arc;

pub async fn create_core_provider_router(db_url: String) -> Router {
    let db_connection = Database::connect(db_url).await.expect("Database can't connect");

    // DB repos
    let subscription_repo = Arc::new(EventsRepoForSql::create_repo(db_connection.clone()));
    let transfer_provider_repo = Arc::new(TransferProviderRepoForSql::create_repo(
        db_connection.clone(),
    ));
    let cn_provider_repo = Arc::new(ContractNegotiationProviderRepoForSql::create_repo(
        db_connection.clone(),
    ));
    let catalog_repo = Arc::new(CatalogRepoForSql::create_repo(db_connection));

    // =====================
    // EVENTS
    // =====================

    // Events router
    let subscription_service = Arc::new(RainbowEventsSubscriptionService::new(
        subscription_repo.clone(),
    ));
    let subscription_router = RainbowEventsSubscriptionRouter::new(subscription_service, None).router();
    let notification_service = Arc::new(RainbowEventsNotificationsService::new(subscription_repo));
    let notification_router = RainbowEventsNotificationRouter::new(notification_service.clone(), None).router();

    // =====================
    // CATALOG
    // =====================

    // Rainbow Entities Dependency injection
    let catalog_ds_protocol_service = Arc::new(DSProtocolCatalogService::new(catalog_repo.clone()));
    let catalog_rainbow_catalog_service = Arc::new(RainbowCatalogCatalogService::new(catalog_repo.clone()));
    let catalog_rainbow_data_service_service = Arc::new(RainbowCatalogDataServiceService::new(catalog_repo.clone()));
    let catalog_rainbow_dataset_service = Arc::new(RainbowCatalogDatasetService::new(catalog_repo.clone()));
    let catalog_rainbow_distribution_service = Arc::new(RainbowCatalogDistributionService::new(catalog_repo.clone()));
    let catalog_rainbow_policies_service = Arc::new(RainbowCatalogPoliciesService::new(catalog_repo.clone()));

    let catalog_rainbow_catalog_router = RainbowCatalogCatalogRouter::new(
        catalog_rainbow_catalog_service,
        catalog_ds_protocol_service.clone(),
    )
        .router();
    let catalog_rainbow_data_service_router =
        RainbowCatalogDataServiceRouter::new(catalog_rainbow_data_service_service.clone()).router();
    let catalog_rainbow_dataset_router =
        RainbowCatalogDatasetRouter::new(catalog_rainbow_dataset_service.clone()).router();
    let catalog_rainbow_distributions_router =
        RainbowCatalogDistributionRouter::new(catalog_rainbow_distribution_service.clone()).router();
    let catalog_rainbow_policies_router =
        RainbowCatalogPoliciesRouter::new(catalog_rainbow_policies_service.clone()).router();

    // RPC Dependency injection
    let catalog_rainbow_rpc_service = Arc::new(RainbowRPCCatalogService::new(catalog_repo.clone()));
    let catalog_rainbow_rpc_router = RainbowRPCCatalogRouter::new(catalog_rainbow_rpc_service.clone()).router();

    // =====================
    // CONTRACT NEGOTIATION
    // =====================

    // Rainbow Entities Dependency injection
    let cn_rainbow_entities_service = Arc::new(RainbowEntitiesContractNegotiationProviderService::new(
        cn_provider_repo.clone(),
    ));
    let cn_rainbow_entities_router =
        RainbowEntitesContractNegotiationProviderRouter::new(cn_rainbow_entities_service.clone()).router();

    // DSProtocol Dependency injection
    let cn_ds_protocol_service = Arc::new(DSProtocolContractNegotiationProviderService::new(
        cn_provider_repo.clone(),
    ));
    let cn_ds_protocol_router =
        DSProtocolContractNegotiationProviderRouter::new(cn_ds_protocol_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let cn_ds_protocol_rpc_service = Arc::new(DSRPCContractNegotiationProviderService::new(
        cn_provider_repo.clone(),
    ));
    let cn_ds_protocol_rpc_router =
        DSRPCContractNegotiationProviderRouter::new(cn_ds_protocol_rpc_service.clone()).router();

    // =====================
    // TRANSFER
    // =====================

    // Rainbow Entities Dependency injection
    let transfer_rainbow_entities_service =
        RainbowTransferProviderServiceImpl::new(transfer_provider_repo.clone(), notification_service.clone());
    let transfer_rainbow_entities_router =
        RainbowTransferProviderEntitiesRouter::new(Arc::new(transfer_rainbow_entities_service)).router();

    // DSProtocol Dependency injection
    let transfer_data_plane_facade = Arc::new(DataPlaneProviderFacadeImpl::new());
    let transfer_data_service_facade = Arc::new(DataServiceFacadeImpl::new());
    let transfer_ds_protocol_service = Arc::new(DSProtocolTransferProviderImpl::new(
        transfer_provider_repo.clone(),
        transfer_data_service_facade.clone(),
        transfer_data_plane_facade.clone(),
        notification_service.clone(),
    ));
    let transfer_ds_protocol_router =
        DSProtocolTransferProviderRouter::new(transfer_ds_protocol_service.clone()).router();

    // DSRPCProtocol Dependency injection
    let transfer_ds_protocol_rpc_service = Arc::new(DSRPCTransferProviderService::new(
        transfer_provider_repo.clone(),
        transfer_data_service_facade,
        transfer_data_plane_facade,
        notification_service.clone(),
    ));
    let transfer_ds_protocol_rpc_router =
        DSRPCTransferProviderProviderRouter::new(transfer_ds_protocol_rpc_service.clone()).router();

    // =====================
    // ROUTER
    // =====================

    let the_router = Router::new()
        .merge(catalog_rainbow_catalog_router)
        .merge(catalog_rainbow_data_service_router)
        .merge(catalog_rainbow_dataset_router)
        .merge(catalog_rainbow_distributions_router)
        .merge(catalog_rainbow_policies_router)
        .merge(catalog_rainbow_rpc_router)
        .merge(cn_rainbow_entities_router)
        .merge(cn_ds_protocol_router)
        .merge(cn_ds_protocol_rpc_router)
        .merge(transfer_rainbow_entities_router)
        .merge(transfer_ds_protocol_router)
        .merge(transfer_ds_protocol_rpc_router)
        .nest("/api/v1/transfers", subscription_router.clone())
        .nest("/api/v1/transfers", notification_router.clone())
        .nest("/api/v1/catalog", subscription_router.clone())
        .nest("/api/v1/catalog", notification_router.clone())
        .nest("/api/v1/contract-negotiation", subscription_router)
        .nest("/api/v1/contract-negotiation", notification_router);

    the_router
}
