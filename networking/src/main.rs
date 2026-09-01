use rcgen::generate_simple_self_signed;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Generate self-signed certificate using rcgen
    let certified_key = generate_simple_self_signed(vec!["localhost".to_string()])?;

    // 2. Extract DER bytes into modern rustls::pki_types
    let cert_der = CertificateDer::from(certified_key.cert.der().to_vec());
    
    // signing_key IS the KeyPair struct, so call serialize_der() directly on it
    let key_der = PrivateKeyDer::Pkcs8(
        certified_key.signing_key.serialize_der().into()
    );

    let certs = vec![cert_der];

    // 3. Configure TLS Server
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key_der)?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    // 4. Start TLS Listener
    let listener = TcpListener::bind("127.0.0.1:8443").await?;
    println!("HTTPS / TLS Server listening on https://localhost:8443");

    loop {
        let (tcp_stream, addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            match acceptor.accept(tcp_stream).await {
                Ok(_tls_stream) => {
                    println!("TLS connection established with {}", addr);
                }
                Err(e) => {
                    eprintln!("TLS handshake failed with {}: {}", addr, e);
                }
            }
        });
    }
}