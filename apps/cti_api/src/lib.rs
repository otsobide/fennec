use std::sync::Arc;

use actix_web::web;

use ::ioc::ioc::application::create_ioc::create_ioc_command_handler::CreateIocCommandHandler;
use ::ioc::ioc::application::create_ioc::ioc_creator::IocCreator;
use ::ioc::ioc::application::delete_ioc::delete_ioc_command_handler::DeleteIocCommandHandler;
use ::ioc::ioc::application::delete_ioc::ioc_deleter::IocDeleter;
use ::ioc::ioc::application::find_ioc::find_ioc_query_handler::FindIocQueryHandler;
use ::ioc::ioc::application::find_ioc::ioc_finder::IocFinder;
use ::ioc::ioc::domain::repositories::ioc_repository::IocRepository;
use ::ioc::ioc::infrastructure::persistence::in_memory::in_memory_ioc_repository::InMemoryIocRepository;
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
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_cqrs::query::domain::query_bus::QueryBus;
use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;

pub mod health;
pub mod ioc;
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
    let query_bus: Arc<dyn QueryBus> = Arc::new(query_bus);

    web::Data::new(AppState { command_bus, query_bus })
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
        .service(ioc::controllers::delete::handler);
}
