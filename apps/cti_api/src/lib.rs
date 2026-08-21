use std::sync::Arc;

use actix_web::web;

use kernel::ioc::application::create_ioc::create_ioc_command_handler::CreateIocCommandHandler;
use kernel::ioc::application::create_ioc::ioc_creator::IocCreator;
use kernel::ioc::application::delete_ioc::delete_ioc_command_handler::DeleteIocCommandHandler;
use kernel::ioc::application::delete_ioc::ioc_deleter::IocDeleter;
use kernel::ioc::application::find_ioc::find_ioc_query_handler::FindIocQueryHandler;
use kernel::ioc::application::find_ioc::ioc_finder::IocFinder;
use kernel::ioc::domain::repositories::ioc_repository::IocRepository;
use kernel::ioc::infrastructure::persistence::in_memory::in_memory_ioc_repository::InMemoryIocRepository;
use kernel::source::application::create_source::create_source_command_handler::CreateSourceCommandHandler;
use kernel::source::application::create_source::source_creator::SourceCreator;
use kernel::source::application::delete_source::delete_source_command_handler::DeleteSourceCommandHandler;
use kernel::source::application::delete_source::source_deleter::SourceDeleter;
use kernel::source::application::find_source::find_source_query_handler::FindSourceQueryHandler;
use kernel::source::application::find_source::source_finder::SourceFinder;
use kernel::source::application::update_source::source_updater::SourceUpdater;
use kernel::source::application::update_source::update_source_command_handler::UpdateSourceCommandHandler;
use kernel::source::domain::repositories::source_repository::SourceRepository;
use kernel::source::infrastructure::persistence::in_memory::in_memory_source_repository::InMemorySourceRepository;
use kernel::url_source::application::create_url_source::create_url_source_command_handler::CreateUrlSourceCommandHandler;
use kernel::url_source::application::create_url_source::url_source_creator::UrlSourceCreator;
use kernel::url_source::application::delete_url_source::delete_url_source_command_handler::DeleteUrlSourceCommandHandler;
use kernel::url_source::application::delete_url_source::url_source_deleter::UrlSourceDeleter;
use kernel::url_source::application::find_url_source::find_url_source_query_handler::FindUrlSourceQueryHandler;
use kernel::url_source::application::find_url_source::url_source_finder::UrlSourceFinder;
use kernel::url_source::application::update_url_source::update_url_source_command_handler::UpdateUrlSourceCommandHandler;
use kernel::url_source::application::update_url_source::url_source_updater::UrlSourceUpdater;
use kernel::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use kernel::url_source::infrastructure::persistence::in_memory::in_memory_url_source_repository::InMemoryUrlSourceRepository;
use ::sighting::sighting::application::create_sighting::create_sighting_command_handler::CreateSightingCommandHandler;
use ::sighting::sighting::application::create_sighting::sighting_creator::SightingCreator;
use ::sighting::sighting::application::delete_sighting::delete_sighting_command_handler::DeleteSightingCommandHandler;
use ::sighting::sighting::application::delete_sighting::sighting_deleter::SightingDeleter;
use ::sighting::sighting::application::find_sighting::find_sighting_query_handler::FindSightingQueryHandler;
use ::sighting::sighting::application::find_sighting::sighting_finder::SightingFinder;
use ::sighting::sighting::application::list_sightings_by_ioc::list_sightings_by_ioc_query_handler::ListSightingsByIocQueryHandler;
use ::sighting::sighting::application::list_sightings_by_ioc::sightings_by_ioc_lister::SightingsByIocLister;
use ::sighting::sighting::application::list_sightings_by_source::list_sightings_by_source_query_handler::ListSightingsBySourceQueryHandler;
use ::sighting::sighting::application::list_sightings_by_source::sightings_by_source_lister::SightingsBySourceLister;
use ::sighting::sighting::application::observe_sighting::observe_sighting_command_handler::ObserveSightingCommandHandler;
use ::sighting::sighting::application::observe_sighting::sighting_observer::SightingObserver;
use ::sighting::sighting::domain::repositories::sighting_repository::SightingRepository;
use ::sighting::sighting::infrastructure::persistence::in_memory::in_memory_sighting_repository::InMemorySightingRepository;
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_cqrs::query::domain::query_bus::QueryBus;
use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;

pub mod health;
pub mod ioc;
pub mod sighting;
pub mod source;
pub mod url_source;

/// Shared application state injected into every Actix-Web request handler.
pub struct AppState {
    pub command_bus: Arc<dyn CommandBus>,
    pub query_bus: Arc<dyn QueryBus>,
}

/// Wires all repositories, services and buses together and returns the shared
/// application state. Each call produces an isolated in-memory store, making
/// this function safe to call once per test.
pub fn build_state() -> web::Data<AppState> {
    let event_bus: Arc<dyn EventBus> = Arc::new(InMemoryEventBus::new());

    // --- source (kernel) ---
    let source_repo: Arc<dyn SourceRepository> = Arc::new(InMemorySourceRepository::new());

    let source_creator = SourceCreator::new(Arc::clone(&source_repo), Arc::clone(&event_bus));
    let source_finder = SourceFinder::new(Arc::clone(&source_repo));
    let source_updater = SourceUpdater::new(Arc::clone(&source_repo), Arc::clone(&event_bus));
    let source_deleter = SourceDeleter::new(Arc::clone(&source_repo), Arc::clone(&event_bus));

    // --- url_source (kernel) ---
    let url_source_repo: Arc<dyn UrlSourceRepository> =
        Arc::new(InMemoryUrlSourceRepository::new());

    let url_source_creator =
        UrlSourceCreator::new(Arc::clone(&url_source_repo), Arc::clone(&event_bus));
    let url_source_finder = UrlSourceFinder::new(Arc::clone(&url_source_repo));
    let url_source_updater =
        UrlSourceUpdater::new(Arc::clone(&url_source_repo), Arc::clone(&event_bus));
    let url_source_deleter =
        UrlSourceDeleter::new(Arc::clone(&url_source_repo), Arc::clone(&event_bus));

    // --- ioc ---
    let ioc_repo: Arc<dyn IocRepository> = Arc::new(InMemoryIocRepository::new());

    let ioc_creator = IocCreator::new(Arc::clone(&ioc_repo), Arc::clone(&event_bus));
    let ioc_finder = IocFinder::new(Arc::clone(&ioc_repo));
    let ioc_deleter = IocDeleter::new(Arc::clone(&ioc_repo), Arc::clone(&event_bus));

    // --- sighting ---
    let sighting_repo: Arc<dyn SightingRepository> = Arc::new(InMemorySightingRepository::new());

    let sighting_creator = SightingCreator::new(Arc::clone(&sighting_repo), Arc::clone(&event_bus));
    let sighting_observer =
        SightingObserver::new(Arc::clone(&sighting_repo), Arc::clone(&event_bus));
    let sighting_deleter = SightingDeleter::new(Arc::clone(&sighting_repo), Arc::clone(&event_bus));
    let sighting_finder = SightingFinder::new(Arc::clone(&sighting_repo));
    let sightings_by_ioc_lister = SightingsByIocLister::new(Arc::clone(&sighting_repo));
    let sightings_by_source_lister = SightingsBySourceLister::new(Arc::clone(&sighting_repo));

    // --- command bus ---
    let mut command_bus = InMemoryCommandBus::new();
    command_bus
        .register(CreateSourceCommandHandler::new(source_creator))
        .expect("Failed to register CreateSourceCommandHandler");
    command_bus
        .register(UpdateSourceCommandHandler::new(source_updater))
        .expect("Failed to register UpdateSourceCommandHandler");
    command_bus
        .register(DeleteSourceCommandHandler::new(source_deleter))
        .expect("Failed to register DeleteSourceCommandHandler");
    command_bus
        .register(CreateUrlSourceCommandHandler::new(url_source_creator))
        .expect("Failed to register CreateUrlSourceCommandHandler");
    command_bus
        .register(UpdateUrlSourceCommandHandler::new(url_source_updater))
        .expect("Failed to register UpdateUrlSourceCommandHandler");
    command_bus
        .register(DeleteUrlSourceCommandHandler::new(url_source_deleter))
        .expect("Failed to register DeleteUrlSourceCommandHandler");
    command_bus
        .register(CreateIocCommandHandler::new(ioc_creator))
        .expect("Failed to register CreateIocCommandHandler");
    command_bus
        .register(DeleteIocCommandHandler::new(ioc_deleter))
        .expect("Failed to register DeleteIocCommandHandler");
    command_bus
        .register(CreateSightingCommandHandler::new(sighting_creator))
        .expect("Failed to register CreateSightingCommandHandler");
    command_bus
        .register(ObserveSightingCommandHandler::new(sighting_observer))
        .expect("Failed to register ObserveSightingCommandHandler");
    command_bus
        .register(DeleteSightingCommandHandler::new(sighting_deleter))
        .expect("Failed to register DeleteSightingCommandHandler");
    let command_bus: Arc<dyn CommandBus> = Arc::new(command_bus);

    // --- query bus ---
    let mut query_bus = InMemoryQueryBus::new();
    query_bus
        .register(FindSourceQueryHandler::new(source_finder))
        .expect("Failed to register FindSourceQueryHandler");
    query_bus
        .register(FindUrlSourceQueryHandler::new(url_source_finder))
        .expect("Failed to register FindUrlSourceQueryHandler");
    query_bus
        .register(FindIocQueryHandler::new(ioc_finder))
        .expect("Failed to register FindIocQueryHandler");
    query_bus
        .register(FindSightingQueryHandler::new(sighting_finder))
        .expect("Failed to register FindSightingQueryHandler");
    query_bus
        .register(ListSightingsByIocQueryHandler::new(sightings_by_ioc_lister))
        .expect("Failed to register ListSightingsByIocQueryHandler");
    query_bus
        .register(ListSightingsBySourceQueryHandler::new(
            sightings_by_source_lister,
        ))
        .expect("Failed to register ListSightingsBySourceQueryHandler");
    let query_bus: Arc<dyn QueryBus> = Arc::new(query_bus);

    web::Data::new(AppState {
        command_bus,
        query_bus,
    })
}

/// Registers all HTTP routes onto an Actix-Web [`ServiceConfig`].
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::get::handler)
        .service(source::controllers::post::handler)
        .service(source::controllers::get::handler)
        .service(source::controllers::put::handler)
        .service(source::controllers::delete::handler)
        .service(url_source::controllers::post::handler)
        .service(url_source::controllers::get::handler)
        .service(url_source::controllers::put::handler)
        .service(url_source::controllers::delete::handler)
        .service(ioc::controllers::post::handler)
        .service(ioc::controllers::get::handler)
        .service(ioc::controllers::delete::handler)
        .service(crate::sighting::controllers::post::handler)
        .service(crate::sighting::controllers::post_observation::handler)
        .service(crate::sighting::controllers::get::handler)
        .service(crate::sighting::controllers::delete::handler)
        .service(crate::sighting::controllers::list_by_ioc::handler)
        .service(crate::sighting::controllers::list_by_source::handler);
}
