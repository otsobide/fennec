use std::sync::Arc;

use actix_web::web;

use kernel::ipv4_address::application::create_ipv4_address::create_ipv4_address_command_handler::CreateIpv4AddressCommandHandler;
use kernel::ipv4_address::application::create_ipv4_address::ipv4_address_creator::Ipv4AddressCreator;
use kernel::ipv4_address::application::delete_ipv4_address::delete_ipv4_address_command_handler::DeleteIpv4AddressCommandHandler;
use kernel::ipv4_address::application::delete_ipv4_address::ipv4_address_deleter::Ipv4AddressDeleter;
use kernel::ipv4_address::application::find_ipv4_address::find_ipv4_address_query_handler::FindIpv4AddressQueryHandler;
use kernel::ipv4_address::application::find_ipv4_address::ipv4_address_finder::Ipv4AddressFinder;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use kernel::ipv4_address::infrastructure::persistence::in_memory::in_memory_ipv4_address_repository::InMemoryIpv4AddressRepository;
use kernel::ipv4_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv4_address_repository::SqlxPostgresIpv4AddressRepository;
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
use kernel::source::infrastructure::persistence::sqlx_postgres::sqlx_postgres_source_repository::SqlxPostgresSourceRepository;
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
use kernel::url_source::infrastructure::persistence::sqlx_postgres::sqlx_postgres_url_source_repository::SqlxPostgresUrlSourceRepository;
use kernel::domain_name::application::create_domain_name::create_domain_name_command_handler::CreateDomainNameCommandHandler;
use kernel::domain_name::application::create_domain_name::domain_name_creator::DomainNameCreator;
use kernel::domain_name::application::delete_domain_name::delete_domain_name_command_handler::DeleteDomainNameCommandHandler;
use kernel::domain_name::application::delete_domain_name::domain_name_deleter::DomainNameDeleter;
use kernel::domain_name::application::find_domain_name::find_domain_name_query_handler::FindDomainNameQueryHandler;
use kernel::domain_name::application::find_domain_name::domain_name_finder::DomainNameFinder;
use kernel::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use kernel::domain_name::infrastructure::persistence::in_memory::in_memory_domain_name_repository::InMemoryDomainNameRepository;
use kernel::domain_name::infrastructure::persistence::sqlx_postgres::sqlx_postgres_domain_name_repository::SqlxPostgresDomainNameRepository;
use kernel::hostname::application::create_hostname::create_hostname_command_handler::CreateHostnameCommandHandler;
use kernel::hostname::application::create_hostname::hostname_creator::HostnameCreator;
use kernel::hostname::application::delete_hostname::delete_hostname_command_handler::DeleteHostnameCommandHandler;
use kernel::hostname::application::delete_hostname::hostname_deleter::HostnameDeleter;
use kernel::hostname::application::find_hostname::find_hostname_query_handler::FindHostnameQueryHandler;
use kernel::hostname::application::find_hostname::hostname_finder::HostnameFinder;
use kernel::hostname::domain::repositories::hostname_repository::HostnameRepository;
use kernel::hostname::infrastructure::persistence::in_memory::in_memory_hostname_repository::InMemoryHostnameRepository;
use kernel::hostname::infrastructure::persistence::sqlx_postgres::sqlx_postgres_hostname_repository::SqlxPostgresHostnameRepository;
use kernel::email_address::application::create_email_address::create_email_address_command_handler::CreateEmailAddressCommandHandler;
use kernel::email_address::application::create_email_address::email_address_creator::EmailAddressCreator;
use kernel::email_address::application::delete_email_address::delete_email_address_command_handler::DeleteEmailAddressCommandHandler;
use kernel::email_address::application::delete_email_address::email_address_deleter::EmailAddressDeleter;
use kernel::email_address::application::find_email_address::find_email_address_query_handler::FindEmailAddressQueryHandler;
use kernel::email_address::application::find_email_address::email_address_finder::EmailAddressFinder;
use kernel::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use kernel::email_address::infrastructure::persistence::in_memory::in_memory_email_address_repository::InMemoryEmailAddressRepository;
use kernel::email_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_email_address_repository::SqlxPostgresEmailAddressRepository;
use kernel::ipv6_address::application::create_ipv6_address::create_ipv6_address_command_handler::CreateIpv6AddressCommandHandler;
use kernel::ipv6_address::application::create_ipv6_address::ipv6_address_creator::Ipv6AddressCreator;
use kernel::ipv6_address::application::delete_ipv6_address::delete_ipv6_address_command_handler::DeleteIpv6AddressCommandHandler;
use kernel::ipv6_address::application::delete_ipv6_address::ipv6_address_deleter::Ipv6AddressDeleter;
use kernel::ipv6_address::application::find_ipv6_address::find_ipv6_address_query_handler::FindIpv6AddressQueryHandler;
use kernel::ipv6_address::application::find_ipv6_address::ipv6_address_finder::Ipv6AddressFinder;
use kernel::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use kernel::ipv6_address::infrastructure::persistence::in_memory::in_memory_ipv6_address_repository::InMemoryIpv6AddressRepository;
use kernel::ipv6_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv6_address_repository::SqlxPostgresIpv6AddressRepository;
use kernel::mac_address::application::create_mac_address::create_mac_address_command_handler::CreateMacAddressCommandHandler;
use kernel::mac_address::application::create_mac_address::mac_address_creator::MacAddressCreator;
use kernel::mac_address::application::delete_mac_address::delete_mac_address_command_handler::DeleteMacAddressCommandHandler;
use kernel::mac_address::application::delete_mac_address::mac_address_deleter::MacAddressDeleter;
use kernel::mac_address::application::find_mac_address::find_mac_address_query_handler::FindMacAddressQueryHandler;
use kernel::mac_address::application::find_mac_address::mac_address_finder::MacAddressFinder;
use kernel::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use kernel::mac_address::infrastructure::persistence::in_memory::in_memory_mac_address_repository::InMemoryMacAddressRepository;
use kernel::mac_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_mac_address_repository::SqlxPostgresMacAddressRepository;
use kernel::ipv4_network::application::create_ipv4_network::create_ipv4_network_command_handler::CreateIpv4NetworkCommandHandler;
use kernel::ipv4_network::application::create_ipv4_network::ipv4_network_creator::Ipv4NetworkCreator;
use kernel::ipv4_network::application::delete_ipv4_network::delete_ipv4_network_command_handler::DeleteIpv4NetworkCommandHandler;
use kernel::ipv4_network::application::delete_ipv4_network::ipv4_network_deleter::Ipv4NetworkDeleter;
use kernel::ipv4_network::application::find_ipv4_network::find_ipv4_network_query_handler::FindIpv4NetworkQueryHandler;
use kernel::ipv4_network::application::find_ipv4_network::ipv4_network_finder::Ipv4NetworkFinder;
use kernel::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use kernel::ipv4_network::infrastructure::persistence::in_memory::in_memory_ipv4_network_repository::InMemoryIpv4NetworkRepository;
use kernel::ipv4_network::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv4_network_repository::SqlxPostgresIpv4NetworkRepository;
use kernel::ipv6_network::application::create_ipv6_network::create_ipv6_network_command_handler::CreateIpv6NetworkCommandHandler;
use kernel::ipv6_network::application::create_ipv6_network::ipv6_network_creator::Ipv6NetworkCreator;
use kernel::ipv6_network::application::delete_ipv6_network::delete_ipv6_network_command_handler::DeleteIpv6NetworkCommandHandler;
use kernel::ipv6_network::application::delete_ipv6_network::ipv6_network_deleter::Ipv6NetworkDeleter;
use kernel::ipv6_network::application::find_ipv6_network::find_ipv6_network_query_handler::FindIpv6NetworkQueryHandler;
use kernel::ipv6_network::application::find_ipv6_network::ipv6_network_finder::Ipv6NetworkFinder;
use kernel::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use kernel::ipv6_network::infrastructure::persistence::in_memory::in_memory_ipv6_network_repository::InMemoryIpv6NetworkRepository;
use kernel::ipv6_network::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv6_network_repository::SqlxPostgresIpv6NetworkRepository;
use kernel::sighting::application::create_sighting::create_sighting_command_handler::CreateSightingCommandHandler;
use kernel::sighting::application::create_sighting::sighting_creator::SightingCreator;
use kernel::sighting::application::delete_sighting::delete_sighting_command_handler::DeleteSightingCommandHandler;
use kernel::sighting::application::delete_sighting::sighting_deleter::SightingDeleter;
use kernel::sighting::application::find_sighting::find_sighting_query_handler::FindSightingQueryHandler;
use kernel::sighting::application::find_sighting::sighting_finder::SightingFinder;
use kernel::sighting::application::list_sightings_by_observable::list_sightings_by_observable_query_handler::ListSightingsByObservableQueryHandler;
use kernel::sighting::application::list_sightings_by_observable::sightings_by_observable_lister::SightingsByObservableLister;
use kernel::sighting::application::list_sightings_by_source::list_sightings_by_source_query_handler::ListSightingsBySourceQueryHandler;
use kernel::sighting::application::list_sightings_by_source::sightings_by_source_lister::SightingsBySourceLister;
use kernel::sighting::application::observe_sighting::observe_sighting_command_handler::ObserveSightingCommandHandler;
use kernel::sighting::application::observe_sighting::sighting_observer::SightingObserver;
use kernel::sighting::domain::repositories::sighting_repository::SightingRepository;
use kernel::sighting::infrastructure::persistence::in_memory::in_memory_sighting_repository::InMemorySightingRepository;
use kernel::sighting::infrastructure::persistence::sqlx_postgres::sqlx_postgres_sighting_repository::SqlxPostgresSightingRepository;
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_cqrs::query::domain::query_bus::QueryBus;
use shared_cqrs::query::infrastructure::in_memory::in_memory_query_bus::InMemoryQueryBus;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::infrastructure::in_memory::in_memory_event_bus::InMemoryEventBus;
use shared_persistence_postgres::infrastructure::postgres::outbox_event_bus::OutboxEventBus;
use shared_persistence_postgres::infrastructure::postgres::transactional_command_bus::TransactionalCommandBus;
use sqlx::PgPool;

pub mod domain_name;
pub mod email_address;
pub mod health;
pub mod hostname;
pub mod ipv4_address;
pub mod ipv4_network;
pub mod ipv6_address;
pub mod ipv6_network;
pub mod mac_address;
pub mod sighting;
pub mod source;
pub mod url_source;

/// Shared application state injected into every Actix-Web request handler.
pub struct AppState {
    pub command_bus: Arc<dyn CommandBus>,
    pub query_bus: Arc<dyn QueryBus>,
}

/// The repository of every kernel module, in memory or in PostgreSQL. There
/// is no mixed mode: a state is built from one family only.
struct Repositories {
    source: Arc<dyn SourceRepository>,
    url_source: Arc<dyn UrlSourceRepository>,
    ipv4_address: Arc<dyn Ipv4AddressRepository>,
    domain_name: Arc<dyn DomainNameRepository>,
    hostname: Arc<dyn HostnameRepository>,
    email_address: Arc<dyn EmailAddressRepository>,
    ipv6_address: Arc<dyn Ipv6AddressRepository>,
    mac_address: Arc<dyn MacAddressRepository>,
    ipv4_network: Arc<dyn Ipv4NetworkRepository>,
    ipv6_network: Arc<dyn Ipv6NetworkRepository>,
    sighting: Arc<dyn SightingRepository>,
}

/// Wires all repositories, services and buses together in memory and returns
/// the shared application state. It never reads the environment, and each
/// call produces isolated in-memory stores, making it safe to call once per
/// test.
pub fn build_state() -> web::Data<AppState> {
    let repositories = Repositories {
        source: Arc::new(InMemorySourceRepository::new()),
        url_source: Arc::new(InMemoryUrlSourceRepository::new()),
        ipv4_address: Arc::new(InMemoryIpv4AddressRepository::new()),
        domain_name: Arc::new(InMemoryDomainNameRepository::new()),
        hostname: Arc::new(InMemoryHostnameRepository::new()),
        email_address: Arc::new(InMemoryEmailAddressRepository::new()),
        ipv6_address: Arc::new(InMemoryIpv6AddressRepository::new()),
        mac_address: Arc::new(InMemoryMacAddressRepository::new()),
        ipv4_network: Arc::new(InMemoryIpv4NetworkRepository::new()),
        ipv6_network: Arc::new(InMemoryIpv6NetworkRepository::new()),
        sighting: Arc::new(InMemorySightingRepository::new()),
    };
    let (command_bus, query_bus) = build_buses(repositories, Arc::new(InMemoryEventBus::new()));

    web::Data::new(AppState {
        command_bus: Arc::new(command_bus),
        query_bus,
    })
}

/// Wires all repositories, services and buses together on PostgreSQL.
///
/// Services publish to the outbox event bus, and the command bus is wrapped
/// in the transactional decorator, so every command commits its state and
/// its events together (`docs/architecture/event-store.md`). The pool's
/// database must already be migrated.
pub fn build_postgres_state(pool: PgPool) -> web::Data<AppState> {
    let repositories = Repositories {
        source: Arc::new(SqlxPostgresSourceRepository::new(pool.clone())),
        url_source: Arc::new(SqlxPostgresUrlSourceRepository::new(pool.clone())),
        ipv4_address: Arc::new(SqlxPostgresIpv4AddressRepository::new(pool.clone())),
        domain_name: Arc::new(SqlxPostgresDomainNameRepository::new(pool.clone())),
        hostname: Arc::new(SqlxPostgresHostnameRepository::new(pool.clone())),
        email_address: Arc::new(SqlxPostgresEmailAddressRepository::new(pool.clone())),
        ipv6_address: Arc::new(SqlxPostgresIpv6AddressRepository::new(pool.clone())),
        mac_address: Arc::new(SqlxPostgresMacAddressRepository::new(pool.clone())),
        ipv4_network: Arc::new(SqlxPostgresIpv4NetworkRepository::new(pool.clone())),
        ipv6_network: Arc::new(SqlxPostgresIpv6NetworkRepository::new(pool.clone())),
        sighting: Arc::new(SqlxPostgresSightingRepository::new(pool.clone())),
    };
    let (command_bus, query_bus) = build_buses(repositories, Arc::new(OutboxEventBus::new()));
    let after_commit: Arc<dyn EventBus> = Arc::new(InMemoryEventBus::new());

    web::Data::new(AppState {
        command_bus: Arc::new(TransactionalCommandBus::new(
            Arc::new(command_bus),
            pool,
            after_commit,
        )),
        query_bus,
    })
}

/// Builds every service and registers every handler on fresh buses.
fn build_buses(
    repositories: Repositories,
    event_bus: Arc<dyn EventBus>,
) -> (InMemoryCommandBus, Arc<dyn QueryBus>) {
    // --- source (kernel) ---
    let source_repo = repositories.source;

    let source_creator = SourceCreator::new(Arc::clone(&source_repo), Arc::clone(&event_bus));
    let source_finder = SourceFinder::new(Arc::clone(&source_repo));
    let source_updater = SourceUpdater::new(Arc::clone(&source_repo), Arc::clone(&event_bus));
    let source_deleter = SourceDeleter::new(Arc::clone(&source_repo), Arc::clone(&event_bus));

    // --- url_source (kernel) ---
    let url_source_repo = repositories.url_source;

    let url_source_creator =
        UrlSourceCreator::new(Arc::clone(&url_source_repo), Arc::clone(&event_bus));
    let url_source_finder = UrlSourceFinder::new(Arc::clone(&url_source_repo));
    let url_source_updater =
        UrlSourceUpdater::new(Arc::clone(&url_source_repo), Arc::clone(&event_bus));
    let url_source_deleter =
        UrlSourceDeleter::new(Arc::clone(&url_source_repo), Arc::clone(&event_bus));

    // --- ipv4_address ---
    let ipv4_address_repo = repositories.ipv4_address;

    let ipv4_address_creator =
        Ipv4AddressCreator::new(Arc::clone(&ipv4_address_repo), Arc::clone(&event_bus));
    let ipv4_address_finder = Ipv4AddressFinder::new(Arc::clone(&ipv4_address_repo));
    let ipv4_address_deleter =
        Ipv4AddressDeleter::new(Arc::clone(&ipv4_address_repo), Arc::clone(&event_bus));

    // --- domain_name ---
    let domain_name_repo = repositories.domain_name;

    let domain_name_creator =
        DomainNameCreator::new(Arc::clone(&domain_name_repo), Arc::clone(&event_bus));
    let domain_name_finder = DomainNameFinder::new(Arc::clone(&domain_name_repo));
    let domain_name_deleter =
        DomainNameDeleter::new(Arc::clone(&domain_name_repo), Arc::clone(&event_bus));

    // --- hostname ---
    let hostname_repo = repositories.hostname;

    let hostname_creator = HostnameCreator::new(Arc::clone(&hostname_repo), Arc::clone(&event_bus));
    let hostname_finder = HostnameFinder::new(Arc::clone(&hostname_repo));
    let hostname_deleter = HostnameDeleter::new(Arc::clone(&hostname_repo), Arc::clone(&event_bus));

    // --- email_address ---
    let email_address_repo = repositories.email_address;

    let email_address_creator =
        EmailAddressCreator::new(Arc::clone(&email_address_repo), Arc::clone(&event_bus));
    let email_address_finder = EmailAddressFinder::new(Arc::clone(&email_address_repo));
    let email_address_deleter =
        EmailAddressDeleter::new(Arc::clone(&email_address_repo), Arc::clone(&event_bus));

    // --- ipv6_address ---
    let ipv6_address_repo = repositories.ipv6_address;

    let ipv6_address_creator =
        Ipv6AddressCreator::new(Arc::clone(&ipv6_address_repo), Arc::clone(&event_bus));
    let ipv6_address_finder = Ipv6AddressFinder::new(Arc::clone(&ipv6_address_repo));
    let ipv6_address_deleter =
        Ipv6AddressDeleter::new(Arc::clone(&ipv6_address_repo), Arc::clone(&event_bus));

    // --- mac_address ---
    let mac_address_repo = repositories.mac_address;

    let mac_address_creator =
        MacAddressCreator::new(Arc::clone(&mac_address_repo), Arc::clone(&event_bus));
    let mac_address_finder = MacAddressFinder::new(Arc::clone(&mac_address_repo));
    let mac_address_deleter =
        MacAddressDeleter::new(Arc::clone(&mac_address_repo), Arc::clone(&event_bus));

    // --- ipv4_network ---
    let ipv4_network_repo = repositories.ipv4_network;

    let ipv4_network_creator =
        Ipv4NetworkCreator::new(Arc::clone(&ipv4_network_repo), Arc::clone(&event_bus));
    let ipv4_network_finder = Ipv4NetworkFinder::new(Arc::clone(&ipv4_network_repo));
    let ipv4_network_deleter =
        Ipv4NetworkDeleter::new(Arc::clone(&ipv4_network_repo), Arc::clone(&event_bus));

    // --- ipv6_network ---
    let ipv6_network_repo = repositories.ipv6_network;

    let ipv6_network_creator =
        Ipv6NetworkCreator::new(Arc::clone(&ipv6_network_repo), Arc::clone(&event_bus));
    let ipv6_network_finder = Ipv6NetworkFinder::new(Arc::clone(&ipv6_network_repo));
    let ipv6_network_deleter =
        Ipv6NetworkDeleter::new(Arc::clone(&ipv6_network_repo), Arc::clone(&event_bus));

    // --- sighting ---
    let sighting_repo = repositories.sighting;

    let sighting_creator = SightingCreator::new(Arc::clone(&sighting_repo), Arc::clone(&event_bus));
    let sighting_observer =
        SightingObserver::new(Arc::clone(&sighting_repo), Arc::clone(&event_bus));
    let sighting_deleter = SightingDeleter::new(Arc::clone(&sighting_repo), Arc::clone(&event_bus));
    let sighting_finder = SightingFinder::new(Arc::clone(&sighting_repo));
    let sightings_by_observable_lister =
        SightingsByObservableLister::new(Arc::clone(&sighting_repo));
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
        .register(CreateIpv4AddressCommandHandler::new(ipv4_address_creator))
        .expect("Failed to register CreateIpv4AddressCommandHandler");
    command_bus
        .register(DeleteIpv4AddressCommandHandler::new(ipv4_address_deleter))
        .expect("Failed to register DeleteIpv4AddressCommandHandler");
    command_bus
        .register(CreateDomainNameCommandHandler::new(domain_name_creator))
        .expect("Failed to register CreateDomainNameCommandHandler");
    command_bus
        .register(DeleteDomainNameCommandHandler::new(domain_name_deleter))
        .expect("Failed to register DeleteDomainNameCommandHandler");
    command_bus
        .register(CreateHostnameCommandHandler::new(hostname_creator))
        .expect("Failed to register CreateHostnameCommandHandler");
    command_bus
        .register(DeleteHostnameCommandHandler::new(hostname_deleter))
        .expect("Failed to register DeleteHostnameCommandHandler");
    command_bus
        .register(CreateEmailAddressCommandHandler::new(email_address_creator))
        .expect("Failed to register CreateEmailAddressCommandHandler");
    command_bus
        .register(DeleteEmailAddressCommandHandler::new(email_address_deleter))
        .expect("Failed to register DeleteEmailAddressCommandHandler");
    command_bus
        .register(CreateIpv6AddressCommandHandler::new(ipv6_address_creator))
        .expect("Failed to register CreateIpv6AddressCommandHandler");
    command_bus
        .register(DeleteIpv6AddressCommandHandler::new(ipv6_address_deleter))
        .expect("Failed to register DeleteIpv6AddressCommandHandler");
    command_bus
        .register(CreateMacAddressCommandHandler::new(mac_address_creator))
        .expect("Failed to register CreateMacAddressCommandHandler");
    command_bus
        .register(DeleteMacAddressCommandHandler::new(mac_address_deleter))
        .expect("Failed to register DeleteMacAddressCommandHandler");
    command_bus
        .register(CreateIpv4NetworkCommandHandler::new(ipv4_network_creator))
        .expect("Failed to register CreateIpv4NetworkCommandHandler");
    command_bus
        .register(DeleteIpv4NetworkCommandHandler::new(ipv4_network_deleter))
        .expect("Failed to register DeleteIpv4NetworkCommandHandler");
    command_bus
        .register(CreateIpv6NetworkCommandHandler::new(ipv6_network_creator))
        .expect("Failed to register CreateIpv6NetworkCommandHandler");
    command_bus
        .register(DeleteIpv6NetworkCommandHandler::new(ipv6_network_deleter))
        .expect("Failed to register DeleteIpv6NetworkCommandHandler");
    command_bus
        .register(CreateSightingCommandHandler::new(sighting_creator))
        .expect("Failed to register CreateSightingCommandHandler");
    command_bus
        .register(ObserveSightingCommandHandler::new(sighting_observer))
        .expect("Failed to register ObserveSightingCommandHandler");
    command_bus
        .register(DeleteSightingCommandHandler::new(sighting_deleter))
        .expect("Failed to register DeleteSightingCommandHandler");

    // --- query bus ---
    let mut query_bus = InMemoryQueryBus::new();
    query_bus
        .register(FindSourceQueryHandler::new(source_finder))
        .expect("Failed to register FindSourceQueryHandler");
    query_bus
        .register(FindUrlSourceQueryHandler::new(url_source_finder))
        .expect("Failed to register FindUrlSourceQueryHandler");
    query_bus
        .register(FindIpv4AddressQueryHandler::new(ipv4_address_finder))
        .expect("Failed to register FindIpv4AddressQueryHandler");
    query_bus
        .register(FindDomainNameQueryHandler::new(domain_name_finder))
        .expect("Failed to register FindDomainNameQueryHandler");
    query_bus
        .register(FindHostnameQueryHandler::new(hostname_finder))
        .expect("Failed to register FindHostnameQueryHandler");
    query_bus
        .register(FindEmailAddressQueryHandler::new(email_address_finder))
        .expect("Failed to register FindEmailAddressQueryHandler");
    query_bus
        .register(FindIpv6AddressQueryHandler::new(ipv6_address_finder))
        .expect("Failed to register FindIpv6AddressQueryHandler");
    query_bus
        .register(FindMacAddressQueryHandler::new(mac_address_finder))
        .expect("Failed to register FindMacAddressQueryHandler");
    query_bus
        .register(FindIpv4NetworkQueryHandler::new(ipv4_network_finder))
        .expect("Failed to register FindIpv4NetworkQueryHandler");
    query_bus
        .register(FindIpv6NetworkQueryHandler::new(ipv6_network_finder))
        .expect("Failed to register FindIpv6NetworkQueryHandler");
    query_bus
        .register(FindSightingQueryHandler::new(sighting_finder))
        .expect("Failed to register FindSightingQueryHandler");
    query_bus
        .register(ListSightingsByObservableQueryHandler::new(
            sightings_by_observable_lister,
        ))
        .expect("Failed to register ListSightingsByObservableQueryHandler");
    query_bus
        .register(ListSightingsBySourceQueryHandler::new(
            sightings_by_source_lister,
        ))
        .expect("Failed to register ListSightingsBySourceQueryHandler");
    let query_bus: Arc<dyn QueryBus> = Arc::new(query_bus);

    (command_bus, query_bus)
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
        .service(ipv4_address::controllers::post::handler)
        .service(ipv4_address::controllers::get_by_value::handler)
        .service(ipv4_address::controllers::get::handler)
        .service(ipv4_address::controllers::delete::handler)
        .service(domain_name::controllers::post::handler)
        .service(domain_name::controllers::get_by_value::handler)
        .service(domain_name::controllers::get::handler)
        .service(domain_name::controllers::delete::handler)
        .service(hostname::controllers::post::handler)
        .service(hostname::controllers::get_by_value::handler)
        .service(hostname::controllers::get::handler)
        .service(hostname::controllers::delete::handler)
        .service(email_address::controllers::post::handler)
        .service(email_address::controllers::get_by_value::handler)
        .service(email_address::controllers::get::handler)
        .service(email_address::controllers::delete::handler)
        .service(ipv6_address::controllers::post::handler)
        .service(ipv6_address::controllers::get_by_value::handler)
        .service(ipv6_address::controllers::get::handler)
        .service(ipv6_address::controllers::delete::handler)
        .service(mac_address::controllers::post::handler)
        .service(mac_address::controllers::get_by_value::handler)
        .service(mac_address::controllers::get::handler)
        .service(mac_address::controllers::delete::handler)
        .service(ipv4_network::controllers::post::handler)
        .service(ipv4_network::controllers::get_by_value::handler)
        .service(ipv4_network::controllers::get::handler)
        .service(ipv4_network::controllers::delete::handler)
        .service(ipv6_network::controllers::post::handler)
        .service(ipv6_network::controllers::get_by_value::handler)
        .service(ipv6_network::controllers::get::handler)
        .service(ipv6_network::controllers::delete::handler)
        .service(crate::sighting::controllers::post::handler)
        .service(crate::sighting::controllers::post_observation::handler)
        .service(crate::sighting::controllers::get::handler)
        .service(crate::sighting::controllers::delete::handler)
        .service(crate::sighting::controllers::list_by_observable::handler)
        .service(crate::sighting::controllers::list_by_source::handler);
}
