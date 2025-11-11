mod api;

#[tokio::main]
async fn main() {
    // If in debug mode, load environment variables from .env file
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    // Load JWK set from environment variables
    let jwk_set = jwk_set_from_env();

    // Create HTTP server
    let app = api::router(jwk_set);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // Run the server
    axum::serve(listener, app).await.unwrap();
}

// Fetch private keys from the environment and parse them into JSON Web Keys (JWK) with SHA256 thumbprint ids
fn jwk_set_from_env() -> jsonwebtoken::jwk::JwkSet {
    let keys = std::env::vars()
        .filter(|(key, _)| key.starts_with("PRIVATE_KEY"))
        .map(|(_, value)| {
            println!("{value}");
            let encoding_key = jsonwebtoken::EncodingKey::from_ec_pem(value.as_bytes())
                .expect("Encoding Key must be a valid EC PKCS#8 PEM Key");
            let mut jwk = jsonwebtoken::jwk::Jwk::from_encoding_key(
                &encoding_key,
                jsonwebtoken::Algorithm::ES384,
            )
            .unwrap();
            // from_encoding_key does not set the key ID so set it manually
            jwk.common.key_id = Some(jwk.thumbprint(jsonwebtoken::jwk::ThumbprintHash::SHA256));
            jwk
        });

    // Collect the keys into a JWK set
    jsonwebtoken::jwk::JwkSet {
        keys: keys.collect(),
    }
}
