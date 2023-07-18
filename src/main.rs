use std::env;
use std::future::ready;
use std::net::SocketAddr;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::GraphQLSubscription;
use axum::{extract::Extension, middleware, Router, routing::get, Server};
use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tracing_subscriber::util::SubscriberInitExt;

use crate::models::graphql::{MutationRoot, QueryRoot};
use crate::observability::metrics::{create_prometheus_recorder, track_metrics};
use crate::observability::tracing::create_tracer_from_env;
use crate::routes::graphql::{graphql_handler, graphql_playground_handler};
use crate::routes::health::health_handler;
use crate::utils::shutdown_signal::shutdown_signal;

mod mock;
mod models;
mod observability;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let prometheus_recorder = create_prometheus_recorder();
    let registry = Registry::default().with(tracing_subscriber::fmt::layer().pretty());

    match create_tracer_from_env() {
        Some(tracer) => registry
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .try_init()
            .expect("Failed to register tracer with registry"),
        None => registry
            .try_init()
            .expect("Failed to register tracer with registry"),
    };

    let app = Router::new()
        .route(
            "/graphql",
            get(graphql_playground_handler).post(graphql_handler),
        )
        .route_service("/graphql/ws", GraphQLSubscription::new(schema.clone()))
        .route("/health", get(health_handler))
        .route("/metrics", get(move || ready(prometheus_recorder.render())))
        .route_layer(middleware::from_fn(track_metrics))
        .layer(Extension(schema));

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("GraphQL:  http://{}/graphql", addr);
    println!("Health:   http://{}/health", addr);
    println!("Metrics:  http://{}/metrics", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
