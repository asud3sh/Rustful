use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, TlsConnector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------
    // Load certificates and private keys
    // ---------------------------------------------------------

    let server_cert = load_certs("server.crt")?;
    let server_key = load_key("server.key")?;

    let client_cert = load_certs("client.crt")?;
    let client_key = load_key("client.key")?;

    let ca_cert = load_certs("ca.crt")?;

    // ---------------------------------------------------------
    // Root CA / Trust Store
    // ---------------------------------------------------------

    let mut root_store = RootCertStore::empty();

    for cert in ca_cert {
        root_store.add(cert)?;
    }

    // ---------------------------------------------------------
    // SERVER
    // ---------------------------------------------------------

    // The server trusts clients whose certificates
    // are signed by our CA.
    let client_verifier =
        WebPkiClientVerifier::builder(Arc::new(root_store.clone()))
            .build()?;

    let server_config = ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(server_cert, server_key)?;

    let acceptor = TlsAcceptor::from(Arc::new(server_config));

    // ---------------------------------------------------------
    // CLIENT
    // ---------------------------------------------------------

    // The client trusts servers whose certificates
    // are signed by our CA.
    //
    // It also presents client.crt + client.key to the server.
    let client_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_client_auth_cert(client_cert, client_key)?;

    let connector = TlsConnector::from(Arc::new(client_config));

    // ---------------------------------------------------------
    // TCP SERVER
    // ---------------------------------------------------------

    let listener = TcpListener::bind("127.0.0.1:9443").await?;

    println!("mTLS server listening on 127.0.0.1:9443");

    // ---------------------------------------------------------
    // Start server task
    // ---------------------------------------------------------

    let server_task = tokio::spawn(async move {
        let (tcp, addr) = match listener.accept().await {
            Ok(connection) => connection,
            Err(e) => {
                eprintln!("TCP accept failed: {e}");
                return;
            }
        };

        println!("TCP connection from {addr}");

        match acceptor.accept(tcp).await {
            Ok(_tls) => {
                println!("mTLS handshake successful");
                println!("Server verified the client certificate");
            }

            Err(e) => {
                eprintln!("mTLS handshake failed: {e}");
            }
        }
    });

    // ---------------------------------------------------------
    // TCP CLIENT
    // ---------------------------------------------------------

    let tcp = TcpStream::connect("127.0.0.1:9443").await?;

    println!("TCP connection established");

    // This name MUST match a SAN in server.crt.
    //
    // Our certificate contains:
    //
    // DNS:localhost
    //
    let server_name = "localhost".try_into()?;

    // TLS handshake happens here.
    let _tls = connector.connect(server_name, tcp).await?;

    println!("mTLS connection established");
    println!("Client verified the server certificate");

    // Wait for the server to finish processing the connection.
    server_task.await?;

    Ok(())
}

// -------------------------------------------------------------
// Certificate loading
// -------------------------------------------------------------

fn load_certs(
    path: &str,
) -> Result<Vec<CertificateDer<'static>>, Box<dyn std::error::Error>> {
    let certfile = std::fs::File::open(path)?;

    let mut reader = std::io::BufReader::new(certfile);

    let certs = rustls_pemfile::certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(certs)
}

// -------------------------------------------------------------
// Private key loading
// -------------------------------------------------------------

fn load_key(
    path: &str,
) -> Result<PrivateKeyDer<'static>, Box<dyn std::error::Error>> {
    let keyfile = std::fs::File::open(path)?;

    let mut reader = std::io::BufReader::new(keyfile);

    let key = rustls_pemfile::private_key(&mut reader)?
        .ok_or("no private key found")?;

    Ok(key)
}
