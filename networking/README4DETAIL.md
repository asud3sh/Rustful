# Networking, Security, and Distributed Systems with Rust

The intent is not to enumerate terms but to build a coherent mental model of how a byte leaves a Rust function, traverses a network, is protected, and returns — and every place that can break in between. Depth is the objective, not coverage.


## Progression Map

```
LAYER 1  Network Foundations              the substrate
LAYER 2  Application Protocols            HTTP, REST, semantics
LAYER 3  Cryptographic Primitives         the toolbox
LAYER 4  Identity, TLS, and mTLS          turning keys into trust
LAYER 5  Modern Transport and RPC         HTTP/2, gRPC, QUIC
LAYER 6  Security Engineering             defending a running system
LAYER 7  Secure Distributed Services      integration under failure
```

Each layer assumes the previous. Every section ends with the failure modes it introduces.


# SECTION 1 — NETWORK FOUNDATIONS

## 1.1 The problem a network solves

Two independent computers must exchange information without sharing memory, or clock etc. Every networking abstraction exists to make that exchange tractable. The end-to-end principle (Saltzer, Reed, Clark, 1984) states it precisely: functionality that can be implemented at the endpoints should not be duplicated in the network. The network forwards; the endpoints reason.

The path between two hosts is not known to either host:

```
A ── switch ── router ── router ── router ── switch ── B
```

Neither endpoint is aware of every device in the path, and neither should be. That ignorance is what allows the Internet to scale.

## 1.2 Decomposition into layers

Layering is a design tool. Each layer offers a service contract to the layer above and consumes one from the layer below. The OSI model is a taxonomy; the TCP/IP model is what the Internet actually speaks.

```
OSI                      TCP/IP
─────────────            ─────────────
Application              Application
Presentation             (folded in)
Session                  (folded in)
Transport                Transport
Network                  Internet
Data Link                Link
Physical                 (physical)
```

When reasoning about a failure, ask: *at which layer was the contract broken?* A certificate error is an application-level trust failure; a checksum mismatch is a link-level integrity failure; they look unrelated but share the same diagnostic method.

## 1.3 Encapsulation

Data is wrapped as it descends and unwrapped as it ascends:

```
Application bytes
   └─▶ TCP segment     [TCP hdr][payload]
        └─▶ IP packet  [IP hdr][TCP hdr][payload]
             └─▶ Frame [Eth hdr][IP hdr][TCP hdr][payload][FCS]
                  └─▶ Signals
```

Each layer adds a header that is meaningful only to its peer at the far end. The receiver strips in reverse.

## 1.4 Addressing and forwarding — two distinct planes

The control plane decides *where* traffic should go. The data plane actually moves it. This separation is fundamental to every network.

- **MAC address** — a link-local identity. Meaningful only within one broadcast domain.
- **IP address** — a network-layer identity. Meaningful across the routed Internet.

A switch forwards frames within a LAN. A router forwards packets between LANs. Conceptually:

```
SWITCH   →  local link forwarding
ROUTER   →  network-to-network forwarding
```

## 1.5 ARP and Neighbor Discovery

IPv4 requires a translation from network-layer addresses to link-layer addresses. ARP resolves this by broadcast:

```
Host A: "Who has 192.168.1.20?"
Broadcast on LAN
Host B: "Me. My MAC is AA:BB:CC:DD:EE:FF"
```

The result is cached with a timeout. IPv6 generalizes this to Neighbor Discovery over ICMPv6.

## 1.6 IP — best-effort by design

IP provides no delivery, no ordering, no retransmission. That is deliberate. The network's job is to move packets; making it reliable at every hop would require per-hop state that destroys scalability. Reliability is moved to the endpoints (see §1.1).

The consequence:

```
IP        →  best-effort datagram delivery
TCP       →  reliable, ordered byte stream on top of IP
```

## 1.7 IPv4 addressing and subnets

An IPv4 address is 32 bits. A subnet mask defines the network prefix. `192.168.1.0/24` means the first 24 bits identify the network; the remaining 8 identify the host. Subnetting exists so a single allocation can be carved into separately routable segments.

The **default gateway** is the router a host uses when the destination is not in any directly attached subnet:

```
Host 192.168.1.10 wants 8.8.8.8
 → not local
 → send to default gateway 192.168.1.1
 → gateway forwards
```

## 1.8 Routing

A router holds a forwarding table:

```
DESTINATION PREFIX   →  NEXT HOP / INTERFACE
10.0.0.0/8           →  iface A
192.168.1.0/24       →  iface B
0.0.0.0/0 (default)  →  upstream
```

Longest-prefix match selects the entry. Interior gateway protocols (OSPF, IS-IS) compute intra-domain routes; exterior protocols (BGP) exchange reachability between autonomous systems. You will not implement these, but you will debug their consequences constantly.

## 1.9 ICMP

Control and diagnostic messages ride ICMP. `ping` uses echo request/reply; `traceroute` relies on TTL expiry and "time exceeded." ICMP is not a transport — it does not carry application data — but it is how the network reports its own failures.

## 1.10 NAT

Network Address Translation rewrites addresses at a boundary. Consumer routers translate many private addresses to one public address using port multiplexing. This is why private space (RFC 1918) coexists with a scarce public IPv4 space. It is also why peer-to-peer and inbound connections are hard: NAT is stateful and asymmetric.

## 1.11 TCP

TCP provides what IP does not:

```
connection establishment
reliable delivery
ordered byte stream
flow control
congestion control
full duplex
```

The critical mental correction: **TCP is a byte stream, not a message transport.** Boundaries you write are not boundaries the receiver reads.

### Handshake

```
CLIENT                     SERVER
  │──── SYN ─────────────▶ │
  │◀─── SYN+ACK ───────────│
  │──── ACK ─────────────▶ │
  │    ESTABLISHED         │
```

The handshake synchronizes sequence numbers in both directions.

### Sequence numbers and acknowledgements

TCP numbers every byte. A receiver acknowledges "I have everything up to N." A sender can then infer loss, duplication, and reordering without application help. Retransmission is triggered by timeouts or by repeated duplicate ACKs (fast retransmit).

### Flow control

The receive window tells the sender how much unacknowledged data the receiver can currently buffer. This protects the *receiver*.

### Congestion control

Separate from flow control. The congestion window adapts to network conditions (slow start, congestion avoidance, fast recovery). This protects the *network*. Conflating the two is a common conceptual error.

### Byte stream and framing

```rust
stream.write_all(b"HELLO")?;
stream.write_all(b"WORLD")?;
```

The peer may read `HELLOWORLD`, `HEL` then `LOWORLD`, or anything in between. Application protocols must therefore define framing:

```
fixed length
length prefix
delimiter
structured (e.g., HTTP, gRPC)
```

## 1.12 UDP

UDP preserves datagram boundaries and adds port multiplexing and a checksum. It provides no ordering, no retransmission, no connection state. Everything TCP gives you, UDP leaves to the application — which is why QUIC exists (§5).

## 1.13 DNS

A distributed, hierarchical, cached database keyed by name.

**Hierarchy & Structure:** Operates as an inverted tree starting from the root (`.`), branching down to Top-Level Domains (`.com`, `.org`), and resolving down to authoritative nameservers for specific domains.

**Core Record Types:**
- **A / AAAA:** Maps names to IPv4 / IPv6 addresses.
- **CNAME:** Canonical name alias targeting another domain name.
- **MX:** Mail exchange server priority list.
- **TXT:** Arbitrary text metadata (SPF, DKIM, domain verification).
- **NS:** Delegates a DNS zone to authoritative nameservers.
- **SRV:** Service locator defining host and port for specific protocols.


**Resolution Mechanics:**
1. **Iterative Lookup:** Client asks Local Resolver $\rightarrow$ Root Server ($\text{root-servers.net}$) $\rightarrow$ TLD Server ($\text{.com}$) $\rightarrow$ Authoritative Server.
2. **Caching & TTL:** Each node along the path caches responses for the duration set by the **Time To Live (TTL)** parameter to reduce lookup latency and authoritative load.


   **Failure Domain Impact:** Because resolution is the prerequisite for all application-layer transport (HTTP, SMTP, gRPC), a failure at the resolver or authoritative DNS layer acts as a total service outage—causing upstream applications to fail with network timeouts or connection refused errors even when underlying hosts are operational.

## 1.14 The socket API

Rust does not construct frames. It asks the OS for a socket, and the OS mediates transport:

```
RUST PROGRAM
    │
    ▼
SOCKET API  (tokio::net)
    │
    ▼
OPERATING SYSTEM   (TCP / UDP state)
    │
    ▼
NETWORK INTERFACE
```

Tokio exposes `TcpListener`, `TcpStream`, `UdpSocket` with async semantics over this interface.

## 1.15 Rust: TCP and UDP lab

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;

    tokio::spawn(async move {
        loop {
            let (mut socket, addr) = match tcp_listener.accept().await {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("accept failed: {error}");
                    continue;
                }
            };

            tokio::spawn(async move {
                let mut buf = [0u8; 1024];

                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            if let Err(error) = socket.write_all(&buf[..n]).await {
                                eprintln!("{addr}: write failed: {error}");
                                break;
                            }
                        }
                        Err(error) => {
                            eprintln!("{addr}: read failed: {error}");
                            break;
                        }
                    }
                }

                println!("{addr} disconnected");
            });
        }
    });

    let mut client = TcpStream::connect("127.0.0.1:8080").await?;
    client.write_all(b"Hello TCP").await?;

    let mut buf = [0u8; 1024];
    let n = client.read(&mut buf).await?;

    println!(
        "TCP response: {}",
        String::from_utf8_lossy(&buf[..n])
    );

    let udp_server = UdpSocket::bind("127.0.0.1:8081").await?;

    tokio::spawn(async move {
        let mut buf = [0u8; 1024];

        loop {
            let (n, addr) = match udp_server.recv_from(&mut buf).await {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("UDP receive failed: {error}");
                    continue;
                }
            };

            if let Err(error) = udp_server.send_to(&buf[..n], addr).await {
                eprintln!("UDP send failed: {error}");
            }
        }
    });

    let udp_client = UdpSocket::bind("127.0.0.1:0").await?;
    udp_client.connect("127.0.0.1:8081").await?;

    udp_client.send(b"Hello UDP").await?;

    let n = udp_client.recv(&mut buf).await?;

    println!(
        "UDP response: {}",
        String::from_utf8_lossy(&buf[..n])
    );

    Ok(())
}
```

## 1.16 Experimentation

Linux:

```bash
ss -lntp        # listening TCP
ss -lnup        # listening UDP
ip addr         # interfaces
ip route        # routing table
ip neigh        # ARP / ND cache
ping 127.0.0.1
```

Windows:

```powershell
Get-NetTCPConnection
Get-NetUDPEndpoint
ipconfig
route print
arp -a
ping 127.0.0.1
```

Answer, from evidence: which process owns port 8080? Which interface owns 127.0.0.1? What is the default route? What happens when nothing is listening?

### Failure modes introduced here

- **Silent truncation.** Stream reads returning partial data.
- **Head-of-line blocking.** A single lost segment stalls all multiplexed streams over TCP.
- **NAT state expiry.** Long-idle connections dropped without notification.
- **DNS as single point of failure.** A name that fails to resolve looks like everything else is broken.
- **Time.** Clock skew breaks certificate validity, TLS handshake, and logs.

---

# SECTION 2 — APPLICATION PROTOCOL LAYER

## 2.1 Why HTTP exists

TCP delivers bytes. It does not define what a request is, what a resource is, or how either should be named. HTTP defines:

```
METHOD     what is being attempted
TARGET     what it applies to
VERSION    which protocol rules apply
HEADERS    metadata
BODY       representation (optional)
```

This semantic layer is what makes HTTP cacheable, proxyable, and inspectable in a way that a raw byte stream is not.

## 2.2 Requests and responses

```http
GET /users/42 HTTP/1.1
Host: example.com
Accept: application/json
```

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 42,
  "name": "Alice"
}
```

The request-line and status-line are the framing; headers carry metadata; the body is optional.

## 2.3 Methods and their algebra

```
GET      retrieve         safe, idempotent
HEAD     headers only     safe, idempotent
POST     submit/create    neither
PUT      replace          idempotent
PATCH    partial update   idempotent only if designed so
DELETE   remove           idempotent
```

"Safe" means no intended side effects. "Idempotent" means N identical applications equal one. These properties are not decoration; they determine whether retries are legal. A network failure between sending a request and receiving a response leaves the client unsure whether the mutation occurred. Idempotent methods make retry safe.

## 2.4 Status codes

```
1xx  informational
2xx  success           200, 201, 202, 204
3xx  redirection       301, 302, 304, 307, 308
4xx  client error      400, 401, 403, 404, 405, 409, 422, 429
5xx  server error      500, 502, 503, 504
```

The category tells you *whose contract was broken*. This is far more useful than the specific code in most debugging.

## 2.5 Headers as metadata

Headers describe negotiation, authentication, caching, and correlation:

```
Accept, Content-Type          representation negotiation
Authorization, Cookie         credentials
Cache-Control, ETag           caching
User-Agent, X-Request-ID      correlation
```

`Accept` describes what the client will accept. `Content-Type` describes what the client (or server) has actually sent. Confusing the two is a common source of subtle bugs.

## 2.6 REST as an architectural style

REST (Fielding, 2000) prescribes:

```
resources with URIs
stateless interactions
uniform interface (methods + representations)
cacheable responses
layered system (proxies are transparent)
```

It is a *style*, not a specification. Real systems relax it (session cookies reintroduce state, RPC-over-HTTP ignores resource semantics). Understand why the constraints exist before abandoning them.

## 2.7 Statelessness and its real meaning

A stateless request carries everything needed to interpret it. This does not forbid server-side storage; it forbids requiring hidden conversational state at the protocol layer. Statelessness allows:

```
any replica to serve any request
horizontal scaling without session affinity
replays and retries to be semantically safe
```

## 2.8 Idempotency under failure

```
CLIENT ──PUT /users/1──▶ SERVER
                          │  applies update
                          │  response lost in transit
CLIENT: did it happen?
```

An idempotent operation is retry-safe. For non-idempotent operations, use idempotency keys (client-generated request identifiers that the server records).

## 2.9 Caching

Conditional requests and validators:

```
Server:   ETag: "abc123"
Client:   If-None-Match: "abc123"
Server:   304 Not Modified
```

Or time-based: `Last-Modified` / `If-Modified-Since`. `Cache-Control` sets freshness. Caching is not an optimization bolted on; it is part of HTTP's semantics and the reason the web scales.

## 2.10 Cookies

A server sets state on a client with `Set-Cookie`; the client echoes it on subsequent requests. Security attributes:

```
Secure     only sent over TLS
HttpOnly   not readable from JavaScript
SameSite   controls cross-site sending (CSRF mitigation)
Path/Domain scoping
Expires/Max-Age lifetime
```

Missing attributes are a common root cause of session theft and CSRF.

## 2.11 Proxies and reverse proxies

A *forward proxy* sits in front of clients (filtering, egress control, caching). A *reverse proxy* sits in front of servers (TLS termination, load balancing, authentication, rate limiting, observability). Reverse proxies are where cross-cutting concerns live, and understanding them clarifies why some security controls belong at the edge and not in each service.

## 2.12 Rust: a REST API with Axum

```toml
[dependencies]
axum = "..."
serde = { version = "...", features = ["derive"] }
serde_json = "..."
tokio = { version = "...", features = ["full"] }
```

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

type UserStore = Arc<Mutex<HashMap<u64, User>>>;

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(home))
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user).delete(delete_user),
        )
        .with_state(store);

    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "REST API Running!"
}
```

## 2.13 Error design

Returning a zeroed `User` for "not found" conflates absence with a valid empty record. Use a distinct error type:

```rust
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}
```

```rust
async fn get_user(
    State(store): State<UserStore>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let users = store.lock().unwrap();

    match users.get(&id) {
        Some(user) => {
            (StatusCode::OK, Json(user.clone()))
                .into_response()
        }

        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "user not found".to_string(),
            }),
        )
            .into_response(),
    }
}
```

## 2.14 Validation ordering

```
REQUEST
   │
   ▼
PARSE        syntactic shape
   │
   ▼
VALIDATE     field constraints, ranges, formats
   │
   ▼
AUTHORIZE    policy
   │
   ▼
BUSINESS LOGIC
```

Never trust the client; validate on the server even when you also validate on the client.

## 2.15 Experimentation

```bash
curl http://localhost:3000/
curl http://localhost:3000/users
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"id":1,"name":"Alice","email":"alice@example.com"}' \
  http://localhost:3000/users
```

Observe request line, headers, body, status, response.

### Failure modes introduced here

- **Mismatched `Content-Type` and `Accept`.** Silent client misinterpretation.
- **Missing `Cache-Control`.** Stale responses, or worse, cached authenticated responses served to the wrong user.
- **Insecure cookies.** Session theft.
- **Non-idempotent retry.** Duplicate side effects under transient failure.
- **Unbounded request bodies.** Memory exhaustion.
- **Trusting client-side validation.** Full attack surface exposed.

---

# SECTION 3 — CRYPTOGRAPHIC PRIMITIVES

## 3.1 What cryptography provides

Four guarantees, each a separate primitive class:

```
confidentiality    nobody else can read it
integrity          nobody else can modify it undetectably
authenticity       it comes from who it claims
non-repudiation    the author cannot later deny it
```

These are not interchangeable. Hashing gives integrity but not authenticity. Encryption gives confidentiality but not integrity (unless authenticated). Signatures give authenticity and non-repudiation. You will combine these, but you must know which one you are relying on at each point.

## 3.2 Threat model before primitives

Cryptography defends against a specified adversary. Enumerate:

```
What can the attacker observe?
What can the attacker modify?
What can the attacker inject or replay?
What secrets might eventually leak?
```

An unstated threat model yields unverifiable security.

## 3.3 Symmetric encryption

One key, both directions. Fast, used for bulk data.

```
plaintext ──E(k)──▶ ciphertext ──D(k)──▶ plaintext
```

Standards: AES (128/192/256), ChaCha20. Never construct modes manually.

## 3.4 AEAD — authenticated encryption with associated data

Modern encryption combines confidentiality and integrity in a single construction. AEAD takes:

```
plaintext
associated data (authenticated but not encrypted)
nonce
key
```

and yields ciphertext plus an authentication tag. Tampering with either ciphertext or associated data causes verification failure. AES-GCM and ChaCha20-Poly1305 are the widely deployed instances.

**Critical rule:** nonce reuse under the same key is catastrophic for GCM. Nonce management is a security-critical concern, not an implementation detail.

## 3.5 Asymmetric cryptography

Two mathematically related keys:

```
public key   shared freely
private key  kept secret
```

Uses:

```
signatures       prove possession of a private key
key agreement    derive a shared secret over a public channel
```

Asymmetric operations are expensive. In practice they establish a symmetric key; symmetric encryption carries the data.

## 3.6 Digital signatures

```
message + private key ──▶ signature
message + signature + public key ──▶ valid / invalid
```

Security notion: existentially unforgeable under chosen-message attack (EUF-CMA). Deployed schemes: Ed25519, ECDSA (P-256 etc.), RSA-PSS. Do not use RSA PKCS#1 v1.5 for new work.

## 3.7 Key agreement and forward secrecy

Diffie–Hellman lets two parties derive a shared secret over a public channel:

```
a (private)  A = g^a (public)  ──▶
b (private)  B = g^b (public)  ◀──
shared = B^a = A^b
```

Ephemeral keys per session yield **forward secrecy**: compromise of the long-term key does not decrypt past sessions. TLS 1.3 requires forward-secret key exchange.

## 3.8 Hash functions

Fixed-length output from arbitrary input. Required properties:

```
deterministic
preimage resistant
second-preimage resistant
collision resistant
```

SHA-256, SHA-3. Not for passwords (see below). Fast hashes are the wrong tool for password storage precisely because they are fast.

## 3.9 Password hashing

Password KDFs must be *slow* and ideally memory-hard:

```
Argon2id    recommended default
scrypt      memory-hard
bcrypt      legacy but acceptable
```

Salts must be per-user, random, and stored alongside the verifier. Password storage should be a verifier, not a decryptable ciphertext.

## 3.10 Randomness

Security depends on cryptographically secure randomness for:

```
session IDs
nonces
keys
salts
initialization vectors
```

Do not use general-purpose PRNGs (`rand::thread_rng` is fine for non-security uses; for keys, use `OsRng` or the platform CSPRNG). Predictable randomness invalidates every other primitive.

## 3.11 Rust: TLS server with rustls

```rust
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
};

use std::sync::Arc;

use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
```

```rust
use rcgen::generate_simple_self_signed;
use rustls::pki_types::{
    CertificateDer,
    PrivateKeyDer,
};

let certified_key =
    generate_simple_self_signed(
        vec!["localhost".to_string()]
    )?;

let cert_der =
    CertificateDer::from(
        certified_key.cert.der().to_vec()
    );

let key_der =
    PrivateKeyDer::Pkcs8(
        certified_key
            .signing_key
            .serialize_der()
            .into()
    );
```

```rust
let config =
    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            vec![cert_der],
            key_der
        )?;

let acceptor =
    TlsAcceptor::from(
        Arc::new(config)
    );
```

```rust
let listener =
    TcpListener::bind(
        "127.0.0.1:8443"
    ).await?;

loop {
    let (tcp, addr) =
        listener.accept().await?;

    let acceptor =
        acceptor.clone();

    tokio::spawn(async move {
        match acceptor.accept(tcp).await {
            Ok(_tls) => {
                println!(
                    "TLS established: {addr}"
                );
            }

            Err(error) => {
                eprintln!(
                    "TLS failed: {error}"
                );
            }
        }
    });
}
```

### Failure modes introduced here

- **Nonce reuse.** Total loss of confidentiality and integrity under GCM.
- **Weak password hashing.** Offline cracking.
- **Predictable randomness.** Key and token compromise.
- **Ignoring verification results.** "TLS enabled" is not the same as "TLS verified."
- **Rolling your own primitive.** Catastrophic, and never necessary.

---

# SECTION 4 — IDENTITY, PKI, AND MUTUAL AUTHENTICATION

## 4.1 From keys to identity

A public key is not an identity; it is a number. Certificates bind a public key to a subject. A PKI provides the machinery for deciding whether that binding is trustworthy.

## 4.2 X.509 certificates

A certificate contains:

```
subject               who this describes
public key            bound to the subject
issuer                who signed
validity              not-before, not-after
serial                unique within issuer
extensions            constrained usage
signature             over all of the above
```

Extensions carry the semantics that make certificates useful in practice: SAN (what names are valid), keyUsage (what operations are allowed), extendedKeyUsage (serverAuth / clientAuth).

## 4.3 Chains of trust

```
ROOT CA  (self-signed, in trust store)
   │ signs
INTERMEDIATE CA
   │ signs
LEAF CERTIFICATE
```

Validation walks the chain to a trusted root, checking at each step: signature validity, validity period, constraint adherence, revocation (if applicable), and finally that the leaf's identity matches the name being used.

## 4.4 SAN and identity matching

`Subject Alternative Name` is where the identity lives. A certificate for `localhost` must include `DNS:localhost`; a certificate for `127.0.0.1` must include `IP:127.0.0.1`. Modern validators ignore the legacy Common Name entirely.

## 4.5 TLS handshake (1.3)

Conceptually:

```
CLIENT                              SERVER
ClientHello  ─────────────────────▶
             ◀──────────────────── ServerHello
             ◀──────────────────── EncryptedExtensions
             ◀──────────────────── Certificate
             ◀──────────────────── CertificateVerify
             ◀──────────────────── Finished
Finished     ─────────────────────▶
Application Data  ◀── encrypted ──▶
```

Purpose of each step: negotiate parameters, perform ephemeral key exchange, prove the server holds the private key corresponding to its certificate, verify the transcript is unmodified, derive traffic keys, and begin protecting records.

## 4.6 Record layer

After the handshake, application bytes are protected in records:

```
application bytes
     │
     ▼
AEAD protect (with negotiated keys)
     │
     ▼
ciphertext + tag
     │
     ▼
network
```

Above the record layer the application sees a byte stream; below it, cryptography.

## 4.7 Mutual TLS (mTLS)

Standard TLS authenticates the server to the client. mTLS extends this so the server also authenticates the client via a certificate:

```
CLIENT                              SERVER
ClientHello ───────────────────────▶
             ◀────────────────────── Certificate (server)
             ◀────────────────────── CertificateRequest
Client Certificate + CertificateVerify ──▶
             ◀────────────────────── Finished
```

The result is *mutual* authentication with *machine identity* — no passwords, no API keys in headers. This is the foundation of Zero Trust between services.

## 4.8 PKI layout for a small service mesh

```
                    ROOT CA
             ┌────────┴────────┐
             │                 │
       SERVER CERT         CLIENT CERT
             │                 │
        server.key         client.key
             │                 │
             └──── mutual TLS ─┘
```

The CA's *private key* must be protected far more carefully than any leaf key. Compromise of the CA allows issuing certificates trusted by anyone who trusts the CA.

## 4.9 CSRs

A Certificate Signing Request carries the public key and requested subject, signed by the corresponding private key. The CA never sees the private key. Best practice: generate the keypair on the machine that will use it, submit a CSR, receive a signed certificate.

## 4.10 Trust stores

The client trusts the CA (and thus the server certificate). The server trusts the same CA (and thus client certificates). Same trust anchor, different purposes. Keeping those purposes distinct is what keyUsage and extendedKeyUsage enforce.

## 4.11 Rust: mTLS

Server:

```rust
let mut root_store =
    RootCertStore::empty();

for cert in ca_certs {
    root_store.add(cert)?;
}

let client_verifier =
    WebPkiClientVerifier::builder(
        Arc::new(root_store.clone())
    )
    .build()?;

let server_config =
    ServerConfig::builder()
        .with_client_cert_verifier(
            client_verifier
        )
        .with_single_cert(
            server_certs,
            server_key
        )?;
```

Client:

```rust
let client_config =
    ClientConfig::builder()
        .with_root_certificates(
            root_store
        )
        .with_client_auth_cert(
            client_certs,
            client_key
        )?;
```

Connection:

```rust
let tcp =
    TcpStream::connect(
        "127.0.0.1:9443"
    ).await?;

let server_name =
    "localhost".try_into()?;

let tls =
    connector
        .connect(
            server_name,
            tcp
        )
        .await?;
```

## 4.12 Authentication vs. authorization

```
AUTHENTICATION  — Who are you?
AUTHORIZATION   — What are you allowed to do?
```

A valid client certificate proves a machine identity. It says nothing about what that machine may do. Policy is a separate decision.

## 4.13 RBAC and ABAC

Role-Based Access Control:

```
identity → role(s) → permissions
```

Attribute-Based Access Control evaluates arbitrary attributes (subject, resource, action, context) against policy. RBAC scales for small systems; ABAC scales for heterogeneous ones. Both require explicit policy; neither falls out of authentication.

## 4.14 Zero Trust

The premise: the network perimeter is not a trust boundary. Instead:

```
every request    → authenticated
every identity   → authorized
every action     → audited
```

Internal service-to-service calls carry the same scrutiny as external ones. mTLS is a mechanism; Zero Trust is a policy stance.

## 4.15 Failure lab

Break each of:

```
wrong CA
wrong hostname / SAN
expired certificate
missing client certificate
wrong EKU (server cert used as client)
mismatched key/cert pair
```

For each, record:

```
who detected it
at what stage
what validation failed
what property was protected
```

This is the single most valuable exercise in the entire section.

### Failure modes introduced here

- **Certificate expiry.** A silent operational failure across the fleet.
- **Weak key sizes.** Silent but fatal.
- **Overbroad EKU.** A server cert being accepted for client use, or vice versa.
- **CA private key exposure.** Complete trust collapse.
- **Name mismatch ignored.** The certificate validates but is for a different service.

---

# SECTION 5 — MODERN TRANSPORT AND RPC

## 5.1 Why HTTP/1.1 was insufficient

HTTP/1.1 serializes requests per connection. Pipelining was never reliably deployed. The browser workaround (many parallel connections) is wasteful. Every request carries textual headers, repeated for each request on the connection. For modern, chatty, high-fan-out systems this is expensive.

## 5.2 HTTP/2

Introduces:

```
binary framing
streams within one connection
multiplexing
HPACK header compression
stream- and connection-level flow control
server push (deprecated in practice)
```

A connection carries many independent streams:

```
CONNECTION
├── stream 1
├── stream 3
├── stream 5
└── stream 7
```

Frames: `HEADERS`, `DATA`, `WINDOW_UPDATE`, `PING`, `RST_STREAM`, `GOAWAY`.

## 5.3 TCP head-of-line blocking

HTTP/2 multiplexing is logical. Underneath, all streams share one TCP byte stream. If a segment is lost, every stream stalls until retransmission completes — *transport* head-of-line blocking. This is why QUIC exists.

## 5.4 gRPC

RPC semantics over HTTP/2 with Protobuf as the default encoding. Four call shapes:

```
unary                  request → response
server streaming       request → stream of responses
client streaming       stream of requests → response
bidirectional          stream ⇄ stream
```

gRPC adds deadlines, cancellation, and status codes on top of HTTP/2 semantics.

## 5.5 Protocol Buffers

Schema-driven serialization. Field numbers are the wire identity; field names are for humans:

```proto
syntax = "proto3";

message GetUserRequest {
    uint64 id = 1;
}

message UserResponse {
    uint64 id = 1;
    string name = 2;
    string email = 3;
}
```

**Never renumber existing fields in a deployed message.** Doing so silently breaks binary compatibility.

## 5.6 Rust: gRPC with Tonic

```toml
[dependencies]
tokio = "..."
tonic = "..."
prost = "..."
tokio-stream = "..."

[build-dependencies]
tonic-build = "..."
```

```rust
fn main() {
    tonic_build::compile_protos(
        "proto/service.proto"
    )
    .unwrap();
}
```

```rust
use tonic::{
    transport::Server,
    Request,
    Response,
    Status,
};

pub mod service {
    tonic::include_proto!("service");
}

use service::{
    GetUserRequest,
    UserResponse,
};

use service::user_service_server::{
    UserService,
    UserServiceServer,
};

#[derive(Default)]
struct UserServiceImpl;

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {

        let request = request.into_inner();

        if request.id == 0 {
            return Err(
                Status::invalid_argument(
                    "id must be non-zero"
                )
            );
        }

        let response = UserResponse {
            id: request.id,
            name: "Alice".into(),
            email: "alice@example.com".into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    Server::builder()
        .add_service(
            UserServiceServer::new(
                UserServiceImpl::default()
            )
        )
        .serve(addr)
        .await?;

    Ok(())
}
```

## 5.7 QUIC

QUIC runs over UDP and reimplements everything TCP gives you — plus more:

```
reliable streams
per-stream flow control
connection-level flow control
congestion control
TLS 1.3 integrated into the handshake
connection migration
no cross-stream head-of-line blocking
```

Because stream state is per-stream, loss on one stream does not stall others. Because TLS is inside QUIC, the handshake and key exchange proceed in one round trip.

## 5.8 HTTP/3

```
HTTP/1.1  → TCP   → IP
HTTP/2    → TCP   → IP
HTTP/3    → QUIC  → UDP → IP
```

Same application semantics (methods, headers, status codes) over a different transport.

## 5.9 Comparison experiment

Build the same operation as REST/JSON and as gRPC/Protobuf. Measure:

```
payload size
serialization/deserialization cost
latency distribution
streaming behavior
schema enforcement
```

The result is usually less dramatic than advertised and more informative for that reason.

### Failure modes introduced here

- **Stream exhaustion.** Opening unlimited HTTP/2 streams without backpressure.
- **Silent schema drift.** Renumbered fields breaking consumers.
- **Unbounded streaming responses.** Server-side resource exhaustion.
- **QUIC on UDP-blocking networks.** A transport blocked by policy.
- **Treating gRPC as local.** Deadlines and cancellation are not optional.

---

# SECTION 6 — SECURITY ENGINEERING

## 6.1 Security is layered, not a feature

```
NETWORK
  │
TLS
  │
IDENTITY
  │
AUTHENTICATION
  │
AUTHORIZATION
  │
INPUT VALIDATION
  │
BUSINESS LOGIC
  │
DATA SECURITY
  │
OBSERVABILITY
```

Each layer has its own failure mode. A correctly configured TLS layer over a broken authorization layer is not "secure"; it is only encrypted.

## 6.2 Threat modeling

For each asset ask:

```
What are we protecting?
Who is the adversary?
What can they reach?
What can they modify?
What happens if they succeed?
What trust assumptions are we making?
```

STRIDE (Spoofing, Tampering, Repudiation, Information disclosure, Denial of service, Elevation of privilege) is one framework; the questions above are the substance.

## 6.3 Attack surface

Every exposed interface is a surface:

```
HTTP port
gRPC port
admin interface
database
filesystem
config
logs and metrics
dependencies
```

Reducing surface is more reliable than hardening it.

## 6.4 Authentication methods

```
password + KDF      interactive users
API key             programmatic, simple
session cookie      interactive, revocable
JWT                 self-contained claims
OAuth token         delegated authorization
mTLS certificate    machine identity
SSH key             operator access
```

Each carries different properties for lifecycle, revocation, and operational cost.

## 6.5 Sessions

```
LOGIN  → verify credentials
       → issue session id (random, high entropy)
       → client stores (Secure, HttpOnly, SameSite cookie)
       → subsequent requests include it
```

Revocation is immediate because state is server-side. This is a strength sessions have and JWTs do not.

## 6.6 JWT — what it is and what it is not

A JWT is a signed (or encrypted) JSON document:

```
HEADER.PAYLOAD.SIGNATURE
```

A normal signed JWT is not encrypted. Anyone with the token can read the payload. The signature protects integrity, not confidentiality.

Claims to validate carefully:

```
iss   issuer
aud   audience
exp   expiration
nbf   not-before
sub   subject
jti   token identifier (for replay tracking)
```

The audience check is commonly forgotten and commonly exploitable.

## 6.7 OAuth in one paragraph

OAuth is a delegation framework. A resource owner grants a client access to a resource server via an authorization server. The client receives an access token (opaque or JWT). OAuth is not authentication — that is OIDC. And OAuth does not imply JWT, nor JWT OAuth.

## 6.8 RBAC and least privilege

```
identity → role → permissions
```

Least privilege: grant only what the identity needs. Compromise of a scoped identity is survivable; compromise of a god-identity is not.

## 6.9 Rate limiting

Algorithms:

```
fixed window     simple, bursty at boundaries
sliding window   smooths boundaries
token bucket     allows controlled bursts
leaky bucket     enforces smooth output
```

Token bucket model:

```
tokens refill at rate r
request consumes 1
if tokens >= 1 → allow, else reject
```

Per-identity limiting is preferred to per-IP; IPs are shared and spoofable.

## 6.10 Input validation

```
raw input
   │
   ▼
parse (reject malformed)
   │
   ▼
validate syntax
   │
   ▼
validate semantics
   │
   ▼
authorize
   │
   ▼
business logic
```

Client-side validation improves UX; server-side validation provides security.

## 6.11 Secure error handling

Return generic errors to clients; keep details in logs. A helpful error message is often a map of the internals.

## 6.12 Secrets

Never commit to source. Never log. Load from environment or a secret store. Rotate. Assume any static secret will eventually be exposed.

## 6.13 Logging, metrics, tracing

```
log fields:  timestamp, request_id, trace_id, service, route, status,
             latency, identity, error category
metric types: counters, gauges, histograms
tracing:     trace_id propagated across services
```

Never log passwords, tokens, keys, or session cookies. This includes logs from frameworks that happily serialize request bodies.

## 6.14 Supply chain

Your dependency graph is part of your attack surface. Lockfiles, vulnerability scans, minimal dependencies, and vendor review for critical crates are all part of "application security."

## 6.15 Do not build primitives

Use audited libraries (rustls, ring, aws-lc-rs, argon2, etc.). Implementing AES, TLS, or a KDF from scratch is a legitimate cryptographic study and a career-limiting production choice.

### Failure modes introduced here

- **JWT audience not verified.** Tokens for one service accepted by another.
- **Rate limit only at the edge.** Internal callers bypass it.
- **Excessive error detail.** Information disclosure.
- **Secret in a log line.** The most common breach vector in real systems.
- **Trusting dependency names.** Typosquatting and dependency confusion.

---

# SECTION 7 — SECURE DISTRIBUTED SERVICES

## 7.1 The integration problem

Every previous section addressed one concern in isolation. The real work is composing them without letting one layer's assumptions break another's. This is where most production incidents originate: not a broken primitive, but a broken *interface* between primitives.

## 7.2 Target architecture

```
                    CLIENT
                      │
                      ▼
               ┌─────────────┐
               │   mTLS/TLS  │
               └──────┬──────┘
                      │
         ┌────────────┴────────────┐
         │                         │
         ▼                         ▼
    REST / Axum              gRPC / Tonic
         │                         │
         └────────────┬────────────┘
                      │
                      ▼
               AUTHENTICATION
                      │
                      ▼
               AUTHORIZATION
                      │
                      ▼
                RATE LIMITER
                      │
                      ▼
                 VALIDATION
                      │
                      ▼
               BUSINESS LOGIC
                      │
         ┌────────────┼────────────┐
         ▼            ▼            ▼
       USERS       METRICS      LOGGING
```

## 7.3 Project layout

```
secure-service/
├── Cargo.toml
├── build.rs
├── proto/
│   └── user.proto
├── certs/
│   ├── ca.crt
│   ├── server.crt
│   ├── server.key
│   ├── client.crt
│   └── client.key
└── src/
    ├── main.rs
    ├── state.rs
    ├── models.rs
    ├── rest.rs
    ├── grpc.rs
    ├── auth.rs
    ├── tls.rs
    ├── rate_limit.rs
    └── metrics.rs
```

The layout is not cosmetic: it enforces that each cross-cutting concern is a distinct module with a distinct test surface.

## 7.4 State and models

```rust
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<HashMap<String, User>>>,
    metrics: Arc<RwLock<Metrics>>,
}
```

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
    role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum Role {
    Admin,
    User,
    Guest,
}
```

```rust
#[derive(Debug, Default, Clone, Serialize)]
struct Metrics {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    auth_failures: u64,
    authorization_failures: u64,
    rate_limited: u64,
}
```

## 7.5 REST endpoints

```
GET    /health
GET    /users
GET    /users/{id}
POST   /users
DELETE /users/{id}
GET    /metrics
```

## 7.6 gRPC surface

```proto
syntax = "proto3";

package secure_service;

service UserService {
    rpc GetUser(GetUserRequest)
        returns (UserResponse);

    rpc ListUsers(ListUsersRequest)
        returns (stream UserResponse);
}

message GetUserRequest {
    string id = 1;
}

message ListUsersRequest {
}

message UserResponse {
    string id = 1;
    string name = 2;
    string email = 3;
    string role = 4;
}
```

## 7.7 Security pipeline

Every protected request traverses:

```
REQUEST
   │
   ▼
TLS / mTLS            transport identity
   │
   ▼
IDENTIFY CLIENT       extract certificate subject
   │
   ▼
AUTHENTICATE          verify against policy
   │
   ▼
AUTHORIZE             role / attribute decision
   │
   ▼
RATE LIMIT            per-identity budget
   │
   ▼
VALIDATE              syntactic and semantic
   │
   ▼
BUSINESS LOGIC
   │
   ▼
RESPONSE
```

Ordering matters. You cannot rate-limit an unauthenticated identity meaningfully. You cannot authorize a request you have not validated structurally.

## 7.8 Authorization

```rust
fn allowed(role: &Role, action: &str) -> bool {
    match role {
        Role::Admin => true,

        Role::User =>
            matches!(
                action,
                "read_user"
            ),

        Role::Guest =>
            matches!(
                action,
                "read_public"
            ),
    }
}
```

The identity that flows into this function comes from the mTLS peer, not from a header. Headers can be forged by anything that can reach the port; certificates cannot (assuming the CA is not compromised).

## 7.9 Rate limiter

```rust
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::RwLock;

#[derive(Clone)]
struct RateLimiter {
    requests: Arc<RwLock<Vec<Instant>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(
        max_requests: usize,
        window: Duration,
    ) -> Self {
        Self {
            requests: Arc::new(
                RwLock::new(Vec::new())
            ),
            max_requests,
            window,
        }
    }

    async fn check(&self) -> bool {
        let now = Instant::now();

        let mut requests =
            self.requests.write().await;

        requests.retain(|timestamp| {
            now.duration_since(*timestamp)
                < self.window
        });

        if requests.len() >= self.max_requests {
            return false;
        }

        requests.push(now);
        true
    }
}
```

This is an in-process limiter for study. In a multi-instance deployment the counter must live in shared state (Redis, a sidecar, or a service mesh filter) or the effective limit is `limit × replicas`.

## 7.10 Health: liveness vs. readiness

```
liveness   process is running
readiness  process can serve traffic
```

Conflating them causes cascading restarts: a slow dependency marks a process unready, the orchestrator kills it, and the service never stabilizes.

## 7.11 Observability flow

```
REQUEST
   ├── trace_id assigned or propagated
   ├── request start (metric counter)
   ├── authentication result (log + metric)
   ├── authorization result (log + metric)
   ├── handler execution (span)
   ├── latency (histogram)
   └── response status (metric)
```

The three signals — logs, metrics, traces — answer different questions. Logs answer *what happened to this request*. Metrics answer *what is happening in aggregate*. Traces answer *where did time go*.

## 7.12 Failure matrix

```
┌────────────────────────────┬───────────────┐
│ FAILURE                    │ EXPECTED      │
├────────────────────────────┼───────────────┤
│ missing certificate        │ TLS failure   │
│ wrong CA                   │ TLS failure   │
│ wrong hostname             │ TLS failure   │
│ wrong EKU                  │ TLS failure   │
│ expired certificate        │ TLS failure   │
│ invalid token              │ 401           │
│ insufficient role          │ 403           │
│ missing resource           │ 404           │
│ malformed JSON             │ 4xx           │
│ rate limit exceeded        │ 429           │
│ internal failure           │ 5xx           │
│ upstream timeout           │ timeout/5xx   │
└────────────────────────────┴───────────────┘
```

The value is in verifying each row *against your actual implementation*, and in ensuring the response body does not leak details the client is not entitled to.

## 7.13 The packet mental model, assembled

A `GET https://api.example.com/users/42`:

```
APPLICATION
  GET /users/42
        │
        ▼
HTTP
        │
        ▼
TLS
        │
        ▼
TCP
        │
        ▼
IP
        │
        ▼
Ethernet / Wi-Fi
        │
        ▼
PHYSICAL NETWORK
```

A gRPC call:

```
RPC method
    │
    ▼
Protobuf
    │
    ▼
HTTP/2
    │
    ▼
TLS
    │
    ▼
TCP
    │
    ▼
IP
```

An HTTP/3 call:

```
HTTP/3
   │
   ▼
QUIC
   │
   ▼
UDP
   │
   ▼
IP
```

An mTLS call:

```
APPLICATION
    │
    ▼
mTLS
    ├── authenticate server
    ├── authenticate client
    ├── establish keys
    └── protect traffic
```

## 7.14 The security mental model

```
                    TRUST
                      │
                      ▼
                  IDENTITY
                      │
                      ▼
                AUTHENTICATION
                      │
                      ▼
                AUTHORIZATION
                      │
                      ▼
                  RESOURCE
                      │
                      ▼
                  AUDITING
```

Distinctions that must remain distinct in the mind:

```
certificate        ≠ permission
authentication     ≠ authorization
encryption         ≠ authentication
hash               ≠ encryption
TLS                ≠ HTTP
mTLS               ≠ Zero Trust
JWT                ≠ OAuth
UDP                ≠ QUIC
```

These look trivial written down. Most production security incidents are one of these collapsed into another.

## 7.15 Distributed systems: designing for failure

Every arrow in an architecture diagram is a place failure can occur:

```
DNS failure
TCP timeout / connection refused
TLS failure (cert, chain, hostname, EKU)
authentication failure
authorization denial
rate-limit rejection
validation failure
database unavailable
response timeout
connection reset
process crash
partial network partition
```

Robust design anticipates each. Some techniques:

- **Timeouts everywhere.** Every remote call has a deadline.
- **Retries with backoff and jitter, only on idempotent operations.**
- **Circuit breakers.** Stop calling a dependency that is failing fast.
- **Bulkheads.** Isolate thread/connection pools per dependency.
- **Backpressure.** Reject work rather than queue unboundedly.
- **Graceful shutdown.** Drain before exit.

## 7.16 Checklist

**Networking**

```
[ ] OSI / TCP-IP models
[ ] Ethernet, MAC, ARP
[ ] IPv4, IPv6 concepts
[ ] subnetting, default gateway, routing
[ ] ICMP, NAT
[ ] DNS
[ ] ports, sockets
[ ] TCP: handshake, sequence, ACK, retransmit, flow, congestion
[ ] UDP
[ ] framing
```

**HTTP**

```
[ ] request/response shape
[ ] methods, status codes, headers
[ ] cookies, caching, ETag
[ ] REST, idempotency
[ ] proxies, reverse proxies
[ ] validation
[ ] Axum
```

**Cryptography**

```
[ ] confidentiality / integrity / authenticity
[ ] symmetric, AEAD
[ ] hashing, salts, password KDFs
[ ] public/private keys, signatures
[ ] key agreement, forward secrecy
[ ] randomness, nonces
```

**TLS / PKI**

```
[ ] TLS 1.3 handshake
[ ] record layer
[ ] certificates, CA, CSR, chains
[ ] trust stores
[ ] SAN, keyUsage, EKU
[ ] serverAuth, clientAuth
[ ] HTTPS
[ ] rustls / tokio-rustls
```

**mTLS / Zero Trust**

```
[ ] mutual TLS
[ ] client and server auth
[ ] machine identity
[ ] authentication vs authorization
[ ] RBAC, ABAC, least privilege
[ ] microsegmentation
[ ] Zero Trust
```

**Modern protocols**

```
[ ] HTTP/2 framing, streams, multiplexing
[ ] gRPC, Protobuf, streaming shapes
[ ] QUIC
[ ] HTTP/3
```

**Security engineering**

```
[ ] threat modeling
[ ] attack surfaces
[ ] sessions, API keys, JWT, OAuth concepts
[ ] password security
[ ] rate limiting
[ ] input validation
[ ] secure errors, secrets
[ ] logging, metrics, tracing
[ ] supply chain
```

**Rust**

```
[ ] Tokio, async/await
[ ] TcpListener / TcpStream / UdpSocket
[ ] Axum, Serde
[ ] rustls, tokio-rustls, rcgen
[ ] Tonic, Prost
[ ] shared state, synchronization
[ ] Result / error handling
[ ] graceful shutdown
```

## 7.17 The standard

Do not measure success by compilation. Measure it by whether you can reconstruct, from memory, without notes, the following narrative:

> A Rust client calls a secure service. DNS resolves the name. A socket is created. TCP establishes a connection — or QUIC establishes a QUIC connection. TLS negotiates parameters, the server presents a certificate, the client validates the chain and the identity, and in mTLS the client presents its own certificate for the server to validate. Both sides derive traffic keys. HTTP or HTTP/2 (or HTTP/3) carries the application protocol; REST or gRPC defines its semantics. The service authenticates the caller, authorizes the action, enforces a rate limit, validates the input, executes business logic, records metrics and logs, and returns a response that travels back through every layer it descended.

And then, for each sentence, be able to say which RFC, which cryptographic property, and which failure mode it depends on.

That is the objective. Not the libraries. The chain of reasoning from a byte in a Rust buffer to a byte on the wire — and every place it can break in between.