// Code generation module for Express and Fastify middleware

pub mod express;
pub mod fastify;

pub use express::generate_express_middleware;
pub use fastify::generate_fastify_plugin;
