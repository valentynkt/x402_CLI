// x402-dev TypeScript Runtime
//
// This runtime will be embedded into the Rust binary via deno_core
// and provide integration with the Corbits SDK and other TypeScript libraries.
//
// Future stories will add:
// - Corbits SDK integration
// - Express server for mock server
// - Policy middleware
// - Utility functions

export const version = "0.1.0";

export function initialize() {
    console.log("x402-dev runtime initialized");
}
