// Enhanced network security for visualization
pub async fn run_server(port: u16) -> anyhow::Result<()> {
    let server_key = generate_tls_cert()?;
    
    HttpServer::new(|| {
        App::new()
            .wrap(ContentSecurityPolicy::default()
                .frame_ancestors(None)
                .default_src(Some(ContentSecurity::SameOrigin))
            )
            .route("/", web::get().to(serve_dashboard))
            .route("/data", web::get().to(network_data))
    })
    .bind_rustls(("0.0.0.0", port), server_key)?
    .run()
    .await?;
    
    Ok(())
}

fn generate_tls_cert() -> anyhow::Result<rustls::ServerConfig> {
    // Generate self-signed certificate with 24-hour validity
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let key = rustls::PrivateKey(cert.serialize_private_key_der());
    let cert_chain = vec![rustls::Certificate(cert.serialize_der()?)];
    
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)?;
        
    Ok(config)
} 