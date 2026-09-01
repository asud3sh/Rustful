
### NETWORKING, SECURITY AND DISTRIBUTED SYSTEMS WITH RUST ( 7 Hours/Day )
> 7-DAY DEEP-DIVE CURRICULUM

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│                    NETWORKING + SECURITY MASTERY                             │
├────────┬─────────────────────┬───────────────────────────────────────────────┤
│ DAY 1  │ NETWORK FOUNDATIONS │ OSI/TCP-IP, Ethernet, ARP, IP, ICMP, routing  │
│        │                     │ TCP, UDP, sockets, DNS, NAT, packet flow      │
├────────┼─────────────────────┼───────────────────────────────────────────────┤
│ DAY 2  │ APPLICATION NETWORK │ HTTP/1.1, REST, JSON, caching, cookies,       │
│        │ PROTOCOLS           │ proxies, reverse proxies, Axum                │
├────────┼─────────────────────┼───────────────────────────────────────────────┤
│ DAY 3  │ CRYPTO + TLS        │ cryptographic primitives, PKI, TLS 1.3,       │
│        │                     │ certificate validation, HTTPS, rustls         │
├────────┼─────────────────────┼───────────────────────────────────────────────┤
│ DAY 4  │ mTLS + IDENTITY     │ CA, CSR, SAN, EKU, trust, client identity,    │
│        │ + ZERO TRUST        │ authentication, authorization, Zero Trust     │
├────────┼─────────────────────┼───────────────────────────────────────────────┤
│ DAY 5  │ MODERN PROTOCOLS    │ HTTP/2, gRPC, Protobuf, streams, QUIC, HTTP/3 │
├────────┼─────────────────────┼───────────────────────────────────────────────┤
│ DAY 6  │ SECURITY ENGINEERING│ password security, JWT, OAuth concepts,       │
│        │                     │ RBAC, rate limiting, threat modeling, logs    │
├────────┼─────────────────────┼───────────────────────────────────────────────┤
│ DAY 7  │ DISTRIBUTED SYSTEM  │ secure microservice, REST + gRPC + mTLS,      │
│        │ + FINAL PROJECT      │ observability, failure handling, testing     │
└────────┴─────────────────────┴───────────────────────────────────────────────┘
```

Planned daily split:

```text
4–6 HOURS

┌─────────────────────┐
│ THEORY      40–45%  │
├─────────────────────┤
│ RUST        35–40%  │
├─────────────────────┤
│ EXPERIMENTS 15–20%  │
└─────────────────────┘
```

The objective is not:

```text
"I know the names of networking protocols."
```

The objective is:

```text
I understand the problem.
I understand the protocol.
I understand the packet/message flow.
I understand the security model.
I understand failure.
I can observe it.
I can implement it in Rust.
I can debug it.
```

---

----------------------------------------------
### DAY 1 NETWORK FOUNDATIONS
----------------------------------------------

### DAY 1 BIG PICTURE

```text
APPLICATION
     │
     ▼
TRANSPORT
     │
     ▼
NETWORK
     │
     ▼
LINK
     │
     ▼
PHYSICAL
```

We will understand the journey:

```text
Rust function
    ↓
socket
    ↓
TCP / UDP
    ↓
IP
    ↓
Ethernet / Wi-Fi
    ↓
network
```

---

### 1. WHAT IS A NETWORK?

A network allows independent computing systems to exchange information.

At the simplest level:

```text
A ─────────────────────────────── B
```

But the actual path may contain:

```text
A
 │
 ▼
switch
 │
 ▼
router
 │
 ▼
router
 │
 ▼
router
 │
 ▼
switch
 │
 ▼
B
```

The two applications do not need to know every device in the path.

That is a key networking abstraction.

The application says:

```text
"I want to send data to destination X."
```

The lower layers handle the mechanics of delivery.

---

### 2. NETWORKING PROBLEMS

A complete networking system must solve different problems.

```text
IDENTITY
    Who is this host?

ADDRESSING
    Where is the destination?

ROUTING
    Which path should packets take?

DELIVERY
    How do packets reach the destination?

MULTIPLEXING
    Which application receives the data?

RELIABILITY
    What happens when data is lost?

ORDERING
    What happens when packets arrive out of order?

FLOW CONTROL
    What if receiver is slower?

CONGESTION CONTROL
    What if network is overloaded?

SECURITY
    Can traffic be read or modified?

APPLICATION SEMANTICS
    What does the data mean?
```

No single protocol has to solve all of these.

---

### 3. OSI MODEL

```text
7  APPLICATION
   HTTP
   DNS
   SMTP
   FTP

6  PRESENTATION
   representation
   encoding
   compression
   encryption concepts

5  SESSION
   session management
   RPC/session concepts

4  TRANSPORT
   TCP
   UDP
   QUIC

3  NETWORK
   IPv4
   IPv6
   ICMP
   routing

2  DATA LINK
   Ethernet
   Wi-Fi
   MAC
   ARP

1  PHYSICAL
   electrical
   optical
   radio
```

The important thing is not memorization.

Ask:

```text
What responsibility belongs here?
```

---

### 4. TCP/IP MODEL

A more practical Internet model is:

```text
APPLICATION
    │
    ├── HTTP
    ├── DNS
    ├── SSH
    └── gRPC

TRANSPORT
    │
    ├── TCP
    ├── UDP
    └── QUIC

INTERNET
    │
    ├── IPv4
    ├── IPv6
    └── ICMP

LINK
    │
    ├── Ethernet
    └── Wi-Fi
```

The OSI model is useful for reasoning.

The TCP/IP model is useful for understanding actual Internet protocol families.

---

### 5. ENCAPSULATION

Application sends:

```text
Hello
```

TCP wraps it:

```text
[TCP HEADER][Hello]
```

IP wraps the TCP segment:

```text
[IP HEADER][TCP HEADER][Hello]
```

Ethernet wraps the packet:

```text
[ETHERNET HEADER][IP HEADER][TCP HEADER][Hello][TRAILER]
```

Conceptually:

```text
Application data
      ↓
TCP segment
      ↓
IP packet
      ↓
Ethernet frame
      ↓
Bits/signals
```

At the receiver:

```text
Bits
 ↓
Frame
 ↓
Packet
 ↓
Segment
 ↓
Application data
```

---

### 6. ETHERNET AND MAC ADDRESSES

IP is not the only addressing system.

On a local Ethernet network, devices use MAC addresses.

Conceptually:

```text
IP address
    │
    ▼
network-layer identity

MAC address
    │
    ▼
local-link identity
```

Example:

```text
IP:
    192.168.1.10

MAC:
    08:00:27:12:34:56
```

An Ethernet frame contains source and destination MAC addresses.

---

### 7. SWITCH VS ROUTER

A switch primarily forwards frames within a local network.

```text
Host A
   │
   ▼
 SWITCH
  /   \
 ▼     ▼
B       C
```

A router forwards IP packets between networks.

```text
NETWORK A
     │
     ▼
   ROUTER
     │
     ▼
NETWORK B
```

Mental model:

```text
SWITCH
    local link forwarding

ROUTER
    network-to-network forwarding
```

---

### 8. ARP

IPv4 hosts often need to discover:

```text
Which MAC address corresponds to this IPv4 address
on my local network?
```

ARP solves this on IPv4 Ethernet networks.

Conceptually:

```text
HOST A

"I need 192.168.1.20.
 Who has that IP?"
          │
          ▼
     broadcast
          │
          ▼
HOST B

"192.168.1.20 is me.
 My MAC is AA:BB:CC:DD:EE:FF"
```

The result is cached.

Conceptually:

```text
ARP CACHE

192.168.1.20
      ↓
AA:BB:CC:DD:EE:FF
```

IPv6 uses Neighbor Discovery rather than ARP.

---

### 9. IP

IP provides network-layer packet delivery.

IP is fundamentally best-effort.

It does not promise:

```text
delivery
ordering
retransmission
```

Therefore:

```text
IP
    ↓
best-effort packet delivery

TCP
    ↓
reliable ordered byte stream
```

This distinction is fundamental.

---

### 10. IPv4

IPv4 addresses are 32 bits.

Example:

```text
192.168.1.10
```

Conceptually:

```text
11000000.10101000.00000001.00001010
```

IPv4 addressing is normally expressed using four decimal octets.

---

### 11. SUBNETS

Suppose:

```text
192.168.1.0/24
```

The `/24` means:

```text
first 24 bits = network prefix
remaining 8 bits = host portion
```

Conceptually:

```text
192.168.1
     │
     └── network

last octet
     │
     └── host
```

Subnetting allows networks to be divided into smaller logical networks.

---

### 12. DEFAULT GATEWAY

Suppose your host is:

```text
192.168.1.10
```

and wants to reach:

```text
8.8.8.8
```

The destination is not local.

The host sends the packet toward its default gateway.

```text
HOST
192.168.1.10
     │
     ▼
DEFAULT GATEWAY
192.168.1.1
     │
     ▼
OTHER NETWORKS
```

---

### 13. ROUTING

Routers maintain routing information.

Conceptually:

```text
DESTINATION PREFIX
       ↓
NEXT HOP / INTERFACE
```

Example:

```text
10.0.0.0/8
    → interface A

192.168.1.0/24
    → interface B

default
    → upstream router
```

The router examines the destination IP and chooses a route.

---

### 14. ICMP

ICMP is used for network control and diagnostic messages.

Examples include:

```text
echo request
echo reply
destination unreachable
time exceeded
```

`ping` uses ICMP echo messages in common implementations.

Conceptually:

```text
HOST A
  │
  │ ICMP Echo Request
  ▼
HOST B
  │
  │ ICMP Echo Reply
  ▼
HOST A
```

---

### 15. NAT

NAT:

```text
Network Address Translation
```

A common use is translating private internal addresses to a public address.

```text
PRIVATE NETWORK

192.168.1.10
192.168.1.11
192.168.1.12
       │
       ▼
      NAT
       │
       ▼
PUBLIC IP
```

This is a major reason private addresses can coexist behind one public IPv4 address.

---

### 16. TCP

TCP gives the application:

```text
connection
reliable delivery
ordered byte stream
flow control
congestion control
full duplex
```

TCP is not:

```text
message transport
```

It is:

```text
BYTE STREAM
```

---

### 17. TCP HANDSHAKE

```text
CLIENT                         SERVER

   │                             │
   │──────── SYN ──────────────►│
   │                             │
   │◄────── SYN + ACK ──────────│
   │                             │
   │──────── ACK ──────────────►│
   │                             │
   │       ESTABLISHED           │
```

The connection begins with synchronized protocol state.

---

### 18. TCP SEQUENCE NUMBERS

Suppose the conceptual stream is:

```text
ABCDEFGHIJ
```

TCP associates sequence information with transmitted bytes.

This allows it to reason about:

```text
missing data
duplicate data
out-of-order data
acknowledged data
```

Example:

```text
Packet A ─────────►
Packet B ── X

Packet C ─────────►
```

TCP can recognize that part of the sequence is missing and recover using retransmission.

---

### 19. TCP ACKNOWLEDGEMENTS

The receiver acknowledges received data.

```text
CLIENT                         SERVER

data ────────────────────────►
       ◄────────────────────── ACK
```

Acknowledgements allow the sender to determine what the receiver has successfully received.

---

### 20. TCP RETRANSMISSION

If packets are lost:

```text
Packet 1 ─────────────►

Packet 2 ───── X

Packet 3 ─────────────►
```

The protocol can retransmit missing data.

The application normally does not implement this itself.

---

### 21. FLOW CONTROL

Flow control protects the receiver.

Imagine:

```text
sender:
    1 GB/s

receiver:
    100 MB/s
```

Without control, receiver buffers could overflow.

TCP uses a receive window and related mechanisms to communicate how much unacknowledged data the receiver can currently accommodate.

---

### 22. CONGESTION CONTROL

Congestion control protects the network.

Example:

```text
Sender ──► Router ──► Router ──► Receiver
               │
               │ overloaded
               ▼
             queue
```

TCP adapts its transmission rate based on network conditions.

This is different from flow control.

```text
FLOW CONTROL
    protects receiver

CONGESTION CONTROL
    responds to network congestion
```

---

### 23. TCP BYTE STREAM AND FRAMING

This is a critical concept.

Suppose:

```rust
stream.write_all(b"HELLO")?;
stream.write_all(b"WORLD")?;
```

The receiving application is not guaranteed to receive:

```text
HELLO
WORLD
```

as two reads.

It might observe:

```text
HELLOWORLD
```

or:

```text
HEL
LOWORLD
```

or:

```text
HELLOW
ORLD
```

Therefore application protocols need framing.

Possible framing designs:

```text
fixed length

length prefix

delimiter

structured protocol
```

HTTP, gRPC, and other protocols define their own framing rules.

---

### 24. UDP

UDP is datagram oriented.

```text
DATAGRAM 1 ─────────►
DATAGRAM 2 ─────────►
DATAGRAM 3 ─────────►
```

The datagram boundary is preserved.

UDP itself does not provide TCP-style:

```text
retransmission
ordering
connection establishment
flow control
congestion control
```

Protocols built on UDP can provide additional mechanisms.

QUIC is a major example.

---

### 25. TCP VS UDP

```text
┌────────────────────┬─────────────────────────┐
│ TCP                │ UDP                     │
├────────────────────┼─────────────────────────┤
│ Byte stream        │ Datagram                │
│ Connection         │ No connection handshake │
│ Reliable           │ Best effort             │
│ Ordered            │ No ordering guarantee   │
│ Retransmission     │ Application-defined     │
│ Flow control       │ Not TCP-style           │
│ Congestion control │ Built into TCP          │
└────────────────────┴─────────────────────────┘
```

---

### 26. DNS

DNS:

```text
Domain Name System
```

maps names to information such as IP addresses.

Example:

```text
example.com
     │
     ▼
DNS
     │
     ▼
93.184.216.34
```

But DNS is more than hostname-to-IP mapping.

It can provide:

```text
A
AAAA
CNAME
MX
TXT
NS
SRV
```

records.

---

### 27. DNS LOOKUP

A simplified lookup:

```text
APPLICATION
    │
    │ "api.example.com?"
    ▼
LOCAL DNS RESOLVER
    │
    ▼
DNS INFRASTRUCTURE
    │
    ▼
ANSWER
    │
    ▼
IP ADDRESS
```

Resolvers cache results according to TTLs.

This means DNS is also a distributed caching system.

---

### 28. SOCKET API

Your Rust program does not usually build Ethernet frames manually.

It asks the OS for a socket.

```text
RUST
 │
 ▼
SOCKET API
 │
 ▼
OPERATING SYSTEM
 │
 ▼
TCP / UDP
 │
 ▼
NETWORK INTERFACE
```

Tokio provides:

```rust
TcpListener
TcpStream
UdpSocket
```

---

### 29. DAY 1 RUST CODE

#### Cargo.toml

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

#### TCP + UDP lab

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

---

### 30. DAY 1 DEBUGGING LAB

Use OS tools.

Linux:

```bash
ss -lntp
ss -lnup
ip addr
ip route
ip neigh
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

Questions:

```text
Which process owns port 8080?

Which interface owns 127.0.0.1?

What is the default route?

What is the ARP cache?

What happens when no process is listening?
```

---

----------------------------------------------
### DAY 2 HTTP, REST AND WEB NETWORKING
----------------------------------------------

### 1. WHY HTTP EXISTS

TCP gives:

```text
BYTE STREAM
```

It doesn't give:

```text
GET
POST
URL
headers
content type
status code
```

HTTP supplies application-level semantics.

```text
HTTP
  │
  ▼
TCP
  │
  ▼
IP
```

---

### 2. HTTP REQUEST

```http
GET /users/42 HTTP/1.1
Host: example.com
Accept: application/json
```

A request contains conceptually:

```text
METHOD
TARGET
VERSION
HEADERS
BODY
```

Not every request has a body.

---

### 3. HTTP RESPONSE

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "id": 42,
  "name": "Alice"
}
```

Conceptually:

```text
VERSION
STATUS
HEADERS
BODY
```

---

### 4. HTTP METHODS

```text
GET
    retrieve

POST
    submit/create/process

PUT
    replace

PATCH
    partial modification

DELETE
    remove
```

Important properties to study:

```text
safe
idempotent
cacheable
```

Do not assume these properties are identical.

---

### 5. HTTP STATUS CODES

```text
1xx informational

2xx success

3xx redirection

4xx request/client errors

5xx server errors
```

Important examples:

```text
200 OK
201 Created
202 Accepted
204 No Content

400 Bad Request
401 Unauthorized
403 Forbidden
404 Not Found
405 Method Not Allowed
409 Conflict
422 Unprocessable Content
429 Too Many Requests

500 Internal Server Error
502 Bad Gateway
503 Service Unavailable
504 Gateway Timeout
```

---

### 6. HEADERS

Headers carry metadata.

Examples:

```text
Host
Accept
Content-Type
Content-Length
Authorization
Cookie
Set-Cookie
Cache-Control
ETag
If-None-Match
User-Agent
X-Request-ID
```

Headers can describe:

```text
what the client accepts
what the body contains
authentication
caching
content negotiation
cookies
request correlation
```

---

### 7. CONTENT-TYPE

These are not equivalent:

```text
application/json
text/plain
text/html
application/octet-stream
```

`Content-Type` describes the representation being sent.

Example:

```http
Content-Type: application/json
```

means:

```text
"The body is JSON."
```

---

### 8. ACCEPT

The client can say:

```http
Accept: application/json
```

meaning approximately:

```text
"I prefer a JSON representation."
```

This is different from:

```http
Content-Type
```

which describes what is actually being sent.

---

### 9. REST

REST is an architectural style.

A resource model might look like:

```text
/users
/users/1
/users/2

/orders
/orders/100
```

Operations use HTTP semantics.

```text
GET /users/1
POST /users
DELETE /users/1
```

---

### 10. STATELESSNESS

A traditional REST-style architecture emphasizes stateless requests.

Conceptually:

```text
REQUEST N
    contains the context needed

REQUEST N+1
    contains its own context
```

The server should not have to infer the meaning of the current request solely from undocumented connection state.

This does not mean the server cannot have application state.

It means the protocol interaction should not require hidden conversational state in the way some stateful protocols do.

---

### 11. IDEMPOTENCY

Suppose:

```text
PUT /users/1
```

sets user 1 to:

```json
{
  "name": "Alice"
}
```

Repeating the same operation should result in the same intended final state.

This is useful because networks fail.

Consider:

```text
CLIENT
   │
   │ PUT request
   ▼
SERVER
   │
   ├── applies update
   │
   └── response gets lost
   X
```

Client cannot tell whether:

```text
request failed
```

or:

```text
request succeeded
response was lost
```

Idempotent semantics make retry safer.

---

### 12. CACHING

HTTP supports caching semantics.

Conceptually:

```text
CLIENT
   │
   ▼
CACHE
   │
   ├── hit → return cached representation
   │
   └── miss → request server
```

Important headers:

```text
Cache-Control
ETag
Last-Modified
If-None-Match
If-Modified-Since
```

An ETag allows conditional requests.

```text
Client:
    If-None-Match: "abc123"

Server:
    304 Not Modified
```

---

### 13. COOKIES

A server can send:

```http
Set-Cookie: session=abc123
```

The client can later send:

```http
Cookie: session=abc123
```

Cookie security attributes include:

```text
Secure
HttpOnly
SameSite
Path
Domain
Expires
Max-Age
```

Important:

```text
HttpOnly
    JavaScript cannot directly read the cookie

Secure
    cookie should only be sent over secure transport

SameSite
    controls cross-site sending behavior
```

---

### 14. PROXY

A proxy sits between client and destination.

```text
CLIENT
   │
   ▼
PROXY
   │
   ▼
SERVER
```

Possible functions:

```text
filtering
caching
access control
traffic inspection
routing
```

---

### 15. REVERSE PROXY

A reverse proxy sits in front of servers.

```text
                 ┌────────────── SERVER A
CLIENT ──► PROXY│
                 └────────────── SERVER B
```

Examples of responsibilities:

```text
TLS termination
load balancing
routing
authentication
rate limiting
compression
observability
```

---

### 16. AXUM

Axum maps HTTP requests to Rust handlers.

```text
HTTP REQUEST
     │
     ▼
Axum Router
     │
     ▼
Extractor
     │
     ▼
Handler
     │
     ▼
Application State
     │
     ▼
HTTP RESPONSE
```

---

### 17. RUST REST API

```toml
[dependencies]
axum = "..."
serde = { version = "...", features = ["derive"] }
serde_json = "..."
tokio = { version = "...", features = ["full"] }
```

Core application:

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

---

### 18. BETTER 404 DESIGN

Avoid:

```json
{
  "id": 0,
  "name": "",
  "email": ""
}
```

because that looks like a valid `User`.

Use an error type.

```rust
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}
```

Then:

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

---

### 19. REQUEST VALIDATION

Bad:

```rust
Json(user)
```

followed immediately by:

```rust
users.insert(user.id, user);
```

Better:

```text
REQUEST
   │
   ▼
PARSE
   │
   ▼
VALIDATE
   │
   ▼
AUTHORIZE
   │
   ▼
BUSINESS LOGIC
```

Validate things such as:

```text
required values
length
format
allowed enum values
numeric ranges
relationships
business rules
```

---

### 20. DAY 2 LAB

Use:

```bash
curl
```

Examples:

```bash
curl http://localhost:3000/
```

```bash
curl http://localhost:3000/users
```

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"id":1,"name":"Alice","email":"alice@example.com"}' \
  http://localhost:3000/users
```

Observe:

```text
request
headers
body
status
response
```

---

----------------------------------------------
### DAY 3 CRYPTOGRAPHY
----------------------------------------------

### 1. WHY CRYPTOGRAPHY EXISTS

Cryptography gives us mechanisms for:

```text
confidentiality
integrity
authentication
non-repudiation concepts
key establishment
```

Cryptography is not one algorithm.

It is a toolbox.

---

### 2. THREAT MODEL

Imagine an attacker:

```text
CLIENT
   │
   │
   ▼
ATTACKER
   │
   ▼
SERVER
```

The attacker may:

```text
observe traffic
modify traffic
inject traffic
replay messages
impersonate endpoints
steal secrets
```

Security mechanisms are designed against specific threats.

---

### 3. CONFIDENTIALITY

Goal:

```text
Attacker sees ciphertext
but cannot recover plaintext.
```

```text
PLAINTEXT
   │
   ▼
ENCRYPT
   │
   ▼
CIPHERTEXT
   │
   ▼
NETWORK
```

---

### 4. INTEGRITY

Goal:

```text
Detect unauthorized modification.
```

```text
MESSAGE
   │
   ▼
AUTHENTICATOR
   │
   ▼
NETWORK
```

Modern authenticated encryption generally combines encryption and integrity protection.

---

### 5. AUTHENTICATION

Goal:

```text
Determine who/what is communicating.
```

This can be achieved through:

```text
password
certificate
public key
token
digital signature
shared secret
```

---

### 6. ENCRYPTION VS HASHING

Encryption:

```text
plaintext
   │
   │ key
   ▼
ciphertext
   │
   │ key
   ▼
plaintext
```

Hashing:

```text
message
   │
   ▼
hash
```

Hashing is not designed to be reversed.

---

### 7. SYMMETRIC CRYPTOGRAPHY

One shared secret key:

```text
             KEY
              │
              ▼
PLAINTEXT → ENCRYPT → CIPHERTEXT
                          │
                          ▼
                       DECRYPT
                          │
                          ▼
                       PLAINTEXT
```

Examples:

```text
AES
ChaCha20
```

Symmetric algorithms are efficient.

---

### 8. AUTHENTICATED ENCRYPTION

Modern protocols generally need:

```text
confidentiality
+
integrity/authentication
```

An AEAD construction provides both.

Examples include concepts such as:

```text
AES-GCM
ChaCha20-Poly1305
```

Mental model:

```text
plaintext
   +
associated authenticated data
   │
   ▼
AEAD
   │
   ▼
ciphertext + authentication tag
```

Associated authenticated data is not encrypted but is integrity-protected.

---

### 9. ASYMMETRIC CRYPTOGRAPHY

Two related keys:

```text
PUBLIC KEY
PRIVATE KEY
```

The private key remains secret.

Uses include:

```text
digital signatures
authentication
key agreement
```

Do not think:

```text
"RSA is encryption, therefore all TLS data uses RSA."
```

Modern TLS uses asymmetric mechanisms primarily for authentication/key establishment and efficient symmetric encryption for application data.

---

### 10. DIGITAL SIGNATURE

```text
MESSAGE
   │
   ▼
SIGN WITH PRIVATE KEY
   │
   ▼
SIGNATURE
```

Verifier:

```text
MESSAGE
   +
SIGNATURE
   +
PUBLIC KEY
   │
   ▼
VERIFY
```

A successful signature verification provides cryptographic evidence linked to the private key.

---

### 11. KEY EXCHANGE

Client and server need shared secret material.

Conceptually:

```text
CLIENT                         SERVER

private material               private material

     │                               │
     │── public information ───────►│
     │◄─ public information ────────│
     │                               │

        DERIVE SHARED SECRET
```

Ephemeral key agreement gives important security properties such as forward secrecy when used appropriately.

---

### 12. FORWARD SECRECY

Suppose an attacker records encrypted traffic today.

Later:

```text
attacker obtains long-term server key
```

With forward-secret key establishment, compromise of the long-term authentication key should not by itself reveal previously recorded session plaintext.

Conceptually:

```text
SESSION 1 → ephemeral secret 1
SESSION 2 → ephemeral secret 2
SESSION 3 → ephemeral secret 3
```

rather than:

```text
everything → one permanent encryption key
```

---

### 13. HASH FUNCTIONS

A hash function maps arbitrary input to a fixed-size digest.

```text
"hello"
   │
   ▼
SHA-256
   │
   ▼
256-bit digest
```

Important properties:

```text
deterministic
fixed output length
preimage resistance
second-preimage resistance
collision resistance
```

---

### 14. PASSWORD HASHING

Password storage should not use ordinary fast hashes such as:

```text
SHA-256(password)
```

Password hashing should use deliberately expensive, memory-hard password hashing mechanisms.

Examples:

```text
Argon2
scrypt
bcrypt
```

Conceptually:

```text
password
   │
   ▼
salt
   +
slow password KDF
   │
   ▼
stored verifier
```

Never store plaintext passwords.

Never encrypt passwords simply because you need to "decrypt them later."

A password verifier normally needs only to determine whether the supplied password matches the stored verifier.

---

### 15. SALTS

A salt is a unique random value used with password hashing.

Without salts:

```text
password "password123"
     │
     ▼
same hash everywhere
```

With unique salts:

```text
password
   +
random salt A
   ▼
verifier A

password
   +
random salt B
   ▼
verifier B
```

This prevents identical passwords from automatically producing identical stored outputs.

---

### 16. RANDOMNESS

Security depends heavily on cryptographically secure randomness.

Examples:

```text
session tokens
nonces
keys
salts
certificate keys
```

Do not use predictable general-purpose randomness for security-sensitive values.

---

### 17. NONCE

Nonce:

```text
number used once
```

Many cryptographic constructions require nonce uniqueness under a given key.

A nonce is not automatically secret.

But misuse can be catastrophic for some algorithms.

Therefore:

```text
crypto API
    +
correct nonce management
```

is critical.

---

### 18. TLS

TLS combines multiple cryptographic ideas.

Conceptually:

```text
               TLS
                │
       ┌────────┼─────────┐
       │        │         │
       ▼        ▼         ▼
authentication key setup encryption
       │        │         │
       └────────┴─────────┘
                │
                ▼
       protected application data
```

---

### 19. TLS HANDSHAKE

Conceptual TLS 1.3 flow:

```text
CLIENT                              SERVER

ClientHello ─────────────────────►
              key/share information

             ◄──────────────────── ServerHello
             ◄──────────────────── Certificate
             ◄──────────────────── CertificateVerify
             ◄──────────────────── Finished

Finished ─────────────────────────►

Encrypted Application Data ◄──────►
```

The exact wire protocol is more detailed, but the conceptual tasks are:

```text
negotiate
establish shared secret
authenticate
prove possession
derive traffic keys
protect application traffic
```

---

### 20. TLS RECORD LAYER

After the handshake, application data is carried in TLS records.

Conceptually:

```text
APPLICATION BYTES
       │
       ▼
TLS RECORD PROTECTION
       │
       ▼
CIPHERTEXT + AUTHENTICATION
       │
       ▼
NETWORK
```

The application does not see raw network packets after decryption.

It sees the resulting protected byte stream.

---

### 21. CERTIFICATES

A certificate is approximately:

```text
IDENTITY
   +
PUBLIC KEY
   +
VALIDITY
   +
EXTENSIONS
   +
ISSUER
   +
DIGITAL SIGNATURE
```

Example:

```text
Subject:
    localhost

Public Key:
    server public key

Issuer:
    Rustful Test CA

Validity:
    start → end

SAN:
    DNS:localhost
    IP:127.0.0.1
```

---

### 22. PKI

PKI provides the machinery for trust relationships.

```text
ROOT CA
   │
   ▼
INTERMEDIATE CA
   │
   ▼
LEAF CERTIFICATE
```

The system trusts a root CA.

The leaf certificate is validated through its chain.

---

### 23. CERTIFICATE CHAIN VALIDATION

Conceptually:

```text
server.crt
    │
    ▼
Issuer signature valid?
    │
    ▼
issuer trusted?
    │
    ▼
validity period?
    │
    ▼
key usage?
    │
    ▼
hostname/SAN?
    │
    ▼
ACCEPT
```

A failure at any relevant step can result in rejection.

---

### 24. SAN

Subject Alternative Name contains identities.

Example:

```text
DNS:localhost
IP:127.0.0.1
```

Suppose client connects to:

```text
localhost
```

The certificate needs an appropriate identity for:

```text
localhost
```

Suppose the client connects to:

```text
127.0.0.1
```

The certificate needs an appropriate IP identity.

---

### 25. TLS VS HTTPS

TLS:

```text
security protocol
```

HTTP:

```text
application protocol
```

HTTPS:

```text
HTTP
  +
TLS
  +
TCP
```

Your current TLS server proves TLS, not a complete HTTP application server.

---

### 26. DAY 3 RUST TLS CODE

Your current approach uses modern `rustls::pki_types`.

Core imports:

```rust
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
};

use std::sync::Arc;

use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
```

Certificate generation:

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

Server configuration:

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

Listener:

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

---

----------------------------------------------
### DAY 4 mTLS, PKI, MACHINE IDENTITY AND ZERO TRUST
----------------------------------------------

### 1. ONE-WAY TLS

```text
CLIENT                         SERVER

   │                             │
   │──── ClientHello ──────────►│
   │                             │
   │◄──── Server Certificate ───│
   │                             │
   │      verify server          │
   │                             │
   │◄════ encrypted session ═══►│
```

---

### 2. mTLS

```text
CLIENT                         SERVER

   │                             │
   │──── ClientHello ──────────►│
   │                             │
   │◄──── Server Certificate ───│
   │                             │
   │      verify server          │
   │                             │
   │◄──── CertificateRequest ───│
   │                             │
   │──── Client Certificate ───►│
   │                             │
   │                     verify client
   │                             │
   │◄════ encrypted session ═══►│
```

This gives:

```text
server authentication
+
client authentication
```

---

### 3. OUR PKI

```text
                  RUSTFUL TEST CA
                         │
             ┌───────────┴───────────┐
             │                       │
             ▼                       ▼
        SERVER CERT              CLIENT CERT
             │                       │
         server.key              client.key
             │                       │
             └────── mutual TLS ─────┘
```

---

### 4. CA CERTIFICATE

```text
ca.crt
```

is distributed to systems that trust this CA.

The CA private key:

```text
ca.key
```

must be protected extremely carefully.

Conceptually:

```text
CA private key compromised
          │
          ▼
attacker may issue
certificates trusted
by systems trusting CA
```

---

### 5. CSR

Certificate Signing Request:

```text
private key
    │
    ├── stays private
    │
    ▼
CSR
    │
    ▼
CA
    │
    ▼
signed certificate
```

The private key is not supposed to be transferred to the CA.

---

### 6. SERVER CERTIFICATE

```text
basicConstraints = CA:FALSE

keyUsage =
    digitalSignature,
    key-related usage appropriate to certificate

extendedKeyUsage =
    serverAuth

subjectAltName =
    DNS:localhost,
    IP:127.0.0.1
```

---

### 7. CLIENT CERTIFICATE

```text
basicConstraints = CA:FALSE

extendedKeyUsage =
    clientAuth
```

The distinction matters.

```text
server.crt
    intended server authentication

client.crt
    intended client authentication
```

---

### 8. TRUST STORE

The client trusts:

```text
ca.crt
```

The server also trusts:

```text
ca.crt
```

But they use that trust for different purposes.

```text
CLIENT

trust CA
   │
   ▼
validate SERVER certificate


SERVER

trust CA
   │
   ▼
validate CLIENT certificate
```

---

### 9. RUST mTLS CONFIGURATION

Your modern code uses:

```rust
ClientConfig
ServerConfig
RootCertStore
WebPkiClientVerifier
CertificateDer
PrivateKeyDer
TlsAcceptor
TlsConnector
```

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

---

### 10. CLIENT CONNECTION

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

Notice:

```text
TCP connection
     │
     ▼
TLS connector
     │
     ▼
server verification
     │
     ▼
client certificate presentation
     │
     ▼
mTLS connection
```

---

### 11. AUTHENTICATION VS AUTHORIZATION

Authentication:

```text
WHO ARE YOU?
```

Authorization:

```text
WHAT MAY YOU DO?
```

Example:

```text
Certificate:
    service-a
```

does not automatically mean:

```text
service-a may delete users
```

We need policy.

---

### 12. RBAC

Role-Based Access Control:

```text
USER
 │
 ▼
ROLE
 │
 ▼
PERMISSIONS
```

Example:

```text
alice
  │
  ▼
admin
  │
  ├── user:read
  ├── user:create
  ├── user:update
  └── user:delete
```

---

### 13. ABAC

Attribute-Based Access Control can evaluate attributes.

Conceptually:

```text
SUBJECT
    identity = service-a

RESOURCE
    type = user

ACTION
    read

CONTEXT
    environment = production
```

Policy:

```text
ALLOW
if service-a
and action=read
and resource=user
```

This becomes useful in larger systems.

---

### 14. ZERO TRUST

Do not assume:

```text
internal network
    =
trusted
```

Instead:

```text
REQUEST
  │
  ▼
AUTHENTICATE
  │
  ▼
AUTHORIZE
  │
  ▼
ENFORCE POLICY
  │
  ▼
RESOURCE
```

---

### 15. MICROSERVICE IDENTITY

```text
                CA
             /      \
            ▼        ▼
        service-a  service-b
            │          │
            └── mTLS ──┘
```

The certificate can provide machine identity.

Authorization still determines permissions.

---

### 16. ZERO TRUST MICROSEGMENTATION

Instead of:

```text
everything inside network trusts everything
```

use:

```text
service-a ─────► service-b
     ALLOW

service-a ──X──► database-admin
     DENY
```

Identity and policy become explicit.

---

### 17. mTLS FAILURE LAB

Break:

```text
wrong CA
wrong hostname
wrong SAN
expired certificate
unknown client certificate
missing client certificate
wrong EKU
wrong private key
wrong certificate/key pair
```

For every failure:

```text
WHO DETECTED IT?
WHEN?
WHAT VALIDATION FAILED?
WHAT SECURITY PROPERTY DID IT PROTECT?
```

---

----------------------------------------------
### DAY 5 HTTP/2, gRPC, PROTOBUF, QUIC AND HTTP/3
----------------------------------------------

### 1. WHY MODERN PROTOCOLS?

HTTP/1.1 works well but has limitations in how many independent requests interact over connections.

Modern systems benefit from:

```text
multiplexing
binary framing
streaming
efficient metadata
strong schemas
```

---

### 2. HTTP/2

HTTP/2 introduces concepts including:

```text
binary framing
streams
multiplexing
header compression
stream-level flow control
connection-level flow control
```

---

### 3. HTTP/2 CONNECTION

Conceptually:

```text
ONE CONNECTION

┌───────────────────────────────┐
│ Stream 1                      │
│ Stream 3                      │
│ Stream 5                      │
│ Stream 7                      │
└───────────────────────────────┘
```

Multiple logical streams share one TCP connection.

---

### 4. HTTP/2 FRAMES

Conceptually:

```text
HEADERS
DATA
DATA
WINDOW_UPDATE
PING
RST_STREAM
GOAWAY
```

Frames are associated with streams or the connection.

---

### 5. MULTIPLEXING

Without multiplexing:

```text
REQUEST A
   │
   ▼
wait

REQUEST B
   │
   ▼
wait

REQUEST C
```

With multiplexing:

```text
connection
    │
    ├── stream A
    ├── stream B
    └── stream C
```

---

### 6. TCP HEAD-OF-LINE EFFECT

Suppose:

```text
HTTP/2 Stream A
HTTP/2 Stream B
HTTP/2 Stream C
```

share one TCP connection.

If a TCP segment carrying bytes is lost:

```text
TCP
 │
 └── missing bytes
       │
       ▼
   retransmission
       │
       ▼
ordered byte stream resumes
```

Because TCP provides one ordered byte stream, all higher-level streams depend on the same transport stream.

This is one motivation for QUIC.

---

### 7. gRPC

RPC:

```text
Remote Procedure Call
```

Conceptually:

```text
LOCAL:

result = get_user(42)

REMOTE REALITY:

local process
     │
     │ request
     ▼
network
     │
     ▼
remote server
     │
     │ response
     ▼
local process
```

---

### 8. gRPC + HTTP/2 + PROTOBUF

```text
gRPC
  │
  ├── RPC semantics
  │
  ├── Protocol Buffers
  │
  └── HTTP/2 transport
```

---

### 9. PROTOBUF

Schema:

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

The schema becomes generated language types.

---

### 10. FIELD NUMBERS

```proto
string name = 2;
```

The `2` is not merely documentation.

It identifies the field in the encoded representation.

Therefore:

```text
field numbers
    =
schema compatibility mechanism
```

Do not casually renumber existing fields in a deployed protocol.

---

### 11. gRPC SERVICE

```proto
service UserService {

    rpc GetUser(
        GetUserRequest
    ) returns (
        UserResponse
    );

}
```

---

### 12. RPC TYPES

```text
UNARY

request
   │
   ▼
response
```

```text
SERVER STREAMING

request
   │
   ▼
response
response
response
response
```

```text
CLIENT STREAMING

request
request
request
request
   │
   ▼
response
```

```text
BIDIRECTIONAL

request  ↔ response
request  ↔ response
request  ↔ response
```

---

### 13. TONIC

Rust implementation:

```text
.proto
  │
  ▼
prost
  │
  ▼
generated Rust types
  │
  ▼
tonic
  │
  ▼
HTTP/2
```

Example:

```toml
[dependencies]
tokio = "..."
tonic = "..."
prost = "..."
tokio-stream = "..."

[build-dependencies]
tonic-build = "..."
```

---

### 14. build.rs

```rust
fn main() {
    tonic_build::compile_protos(
        "proto/service.proto"
    )
    .unwrap();
}
```

---

### 15. BASIC gRPC SERVER

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

---

### 16. QUIC

QUIC runs over UDP:

```text
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

But it is not simply:

```text
UDP with no reliability
```

QUIC implements sophisticated transport behavior.

---

### 17. QUIC PROVIDES

Conceptually:

```text
reliable streams
flow control
congestion control
connection establishment
TLS integration
connection migration
stream independence
```

---

### 18. QUIC STREAMS

A QUIC connection can contain multiple streams.

```text
QUIC CONNECTION

Stream 0
Stream 4
Stream 8
Stream 12
```

Loss affecting one stream does not necessarily create the same cross-stream transport blocking behavior as a single TCP byte stream.

---

### 19. HTTP/3

```text
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

HTTP/2:

```text
HTTP/2
   │
   ▼
TCP
   │
   ▼
IP
```

---

### 20. DAY 5 EXPERIMENT

Compare:

```text
REST / JSON
vs
gRPC / Protobuf
```

Measure:

```text
payload size
serialization cost
latency
streaming behavior
schema enforcement
```

Then conceptually compare:

```text
HTTP/1.1 + TCP
HTTP/2 + TCP
HTTP/3 + QUIC
```

---

----------------------------------------------
### DAY 6 SECURITY ENGINEERING
----------------------------------------------

### 1. SECURITY IS A SYSTEM

Do not think:

```text
TLS = security
```

Real security is layered:

```text
NETWORK
   │
   ▼
TLS
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
INPUT VALIDATION
   │
   ▼
BUSINESS LOGIC
   │
   ▼
DATA SECURITY
   │
   ▼
OBSERVABILITY
```

---

### 2. THREAT MODELING

Before implementing a security control, ask:

```text
What are we protecting?

Who is the attacker?

What can they access?

What can they modify?

What happens if they succeed?

What trust assumptions exist?
```

---

### 3. ATTACK SURFACES

For a service:

```text
HTTP port
gRPC port
TLS endpoint
admin interface
database
filesystem
configuration
logs
metrics
dependencies
```

Each is an attack surface.

---

### 4. AUTHENTICATION METHODS

Examples:

```text
password
API key
session cookie
JWT
OAuth access token
mTLS certificate
SSH public key
```

Each has different:

```text
security properties
lifecycle
revocation model
operational complexity
```

---

### 5. SESSION AUTHENTICATION

Conceptually:

```text
LOGIN
  │
  ▼
server verifies credentials
  │
  ▼
session identifier
  │
  ▼
client stores session
  │
  ▼
future request includes session
```

Security depends on:

```text
random session identifiers
TLS
expiration
revocation
cookie security
CSRF protections where relevant
```

---

### 6. JWT

JWT:

```text
JSON Web Token
```

A signed JWT conceptually contains:

```text
HEADER
PAYLOAD
SIGNATURE
```

Important:

```text
JWT ≠ encryption
```

A normal signed JWT payload is usually readable by anyone who possesses the token.

The signature protects integrity/authenticity of the claims.

---

### 7. JWT CLAIMS

Examples:

```text
iss    issuer
sub    subject
aud    audience
exp    expiration
iat    issued-at
nbf    not-before
jti    token identifier
```

A service should validate claims appropriate to the security design.

---

### 8. OAUTH CONCEPTS

OAuth is fundamentally an authorization framework.

The simplified conceptual model:

```text
RESOURCE OWNER
      │
      ▼
AUTHORIZATION SERVER
      │
      ▼
ACCESS TOKEN
      │
      ▼
CLIENT
      │
      ▼
RESOURCE SERVER
```

OAuth does not inherently mean:

```text
"JWT"
```

JWT may be used as one token format.

---

### 9. API KEYS

An API key is a bearer-style credential in many systems.

```text
Authorization: Bearer ...
```

or a dedicated header.

Treat API keys like secrets.

Properties to consider:

```text
scope
expiration
rotation
revocation
storage
logging
```

---

### 10. RBAC

```text
IDENTITY
   │
   ▼
ROLE
   │
   ▼
PERMISSIONS
```

Example:

```text
service-a
   │
   ▼
reader
   │
   ├── user:read
   └── metrics:read
```

---

### 11. LEAST PRIVILEGE

Give every identity only what it needs.

```text
service-a

ALLOW:
    GET /users

DENY:
    DELETE /users
    POST /admin
```

Least privilege reduces damage if the identity is compromised.

---

### 12. RATE LIMITING

Basic model:

```text
REQUEST
   │
   ▼
RATE LIMITER
   │
   ├── allow
   │
   └── reject
```

Example:

```text
100 requests / minute
```

Possible algorithms:

```text
fixed window
sliding window
token bucket
leaky bucket
```

---

### 13. TOKEN BUCKET

Conceptually:

```text
              TOKENS
                │
                ▼
        ┌────────────────┐
        │   TOKEN BUCKET  │
        └────────────────┘
             │
        request arrives
             │
       ┌─────┴─────┐
       ▼           ▼
    token        no token
    exists       available
       │           │
       ▼           ▼
    ALLOW        REJECT
```

Tokens refill over time.

This allows controlled bursts.

---

### 14. INPUT VALIDATION

Security-sensitive request pipeline:

```text
raw input
   │
   ▼
parse
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

Never rely only on client-side validation.

---

### 15. SECURE ERROR HANDLING

Do not expose internal details unnecessarily.

Bad:

```text
Database error:
postgres password = ...
```

Better:

```json
{
  "error": "internal server error"
}
```

while detailed diagnostics go to controlled logs.

---

### 16. SECRET MANAGEMENT

Secrets include:

```text
private keys
database passwords
API keys
JWT signing keys
CA keys
client credentials
```

Do not hardcode them.

Avoid:

```rust
const PASSWORD: &str = "secret";
```

Prefer controlled secret injection.

---

### 17. LOGGING

Useful fields:

```text
timestamp
request ID
trace ID
service
route
status
latency
authenticated identity
error category
```

Do not log:

```text
passwords
private keys
session secrets
authorization tokens
```

unless there is an exceptional and carefully justified security design.

---

### 18. METRICS

Track:

```text
requests_total
requests_failed
latency
active_connections
tls_failures
auth_failures
authz_denials
rate_limit_rejections
```

This turns the service into an observable system.

---

### 19. DISTRIBUTED TRACING

```text
TRACE
 │
 ├── gateway span
 │
 ├── user-service span
 │
 └── database span
```

A trace ID connects work performed across processes.

---

### 20. DEPENDENCY SECURITY

A Rust service depends on crates.

Security includes:

```text
dependency updates
vulnerability scanning
dependency review
lockfile management
minimal dependencies
```

Do not assume:

```text
"My application code is secure,
therefore my application is secure."
```

---

### 21. SECURE CODING PRINCIPLE

Do not implement cryptographic primitives yourself unless you are explicitly studying cryptography implementation.

Prefer audited libraries.

```text
BAD IDEA

write your own AES
write your own TLS
write your own password KDF
write your own random generator
```

Better:

```text
use established cryptographic libraries
```

The educational goal is to understand the primitives and protocols, not to create a production cipher.

---

----------------------------------------------
### DAY 7 FINAL SECURE DISTRIBUTED RUST SERVICE
----------------------------------------------

### 1. FINAL ARCHITECTURE

```text
                         CLIENT
                           │
                           │
                           ▼
                    ┌─────────────┐
                    │   mTLS/TLS  │
                    └──────┬──────┘
                           │
             ┌─────────────┴─────────────┐
             │                           │
             ▼                           ▼
        REST / Axum                 gRPC / Tonic
             │                           │
             │                           │
             └─────────────┬─────────────┘
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
              │            │            │
              ▼            ▼            ▼
            USERS        METRICS      LOGGING
```

---

### 2. FINAL PROJECT COMPONENTS

Suggested Rust structure:

```text
secure-service/
│
├── Cargo.toml
├── build.rs
│
├── proto/
│   └── user.proto
│
├── certs/
│   ├── ca.crt
│   ├── server.crt
│   ├── server.key
│   ├── client.crt
│   └── client.key
│
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

---

### 3. APPLICATION STATE

```rust
#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<HashMap<String, User>>>,
    metrics: Arc<RwLock<Metrics>>,
}
```

Conceptually:

```text
                 AppState
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
     users       metrics       policy
```

---

### 4. USER MODEL

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

---

### 5. METRICS

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

---

### 6. REST ENDPOINTS

```text
GET  /health
GET  /users
GET  /users/{id}
POST /users
DELETE /users/{id}
GET  /metrics
```

---

### 7. SECURITY PIPELINE

Every protected request:

```text
                    REQUEST
                       │
                       ▼
                  TLS / mTLS
                       │
                       ▼
                 IDENTIFY CLIENT
                       │
                       ▼
                 AUTHENTICATE
                       │
                       ▼
                 AUTHORIZE
                       │
                       ▼
                RATE LIMIT
                       │
                       ▼
                 VALIDATE
                       │
                       ▼
                 BUSINESS LOGIC
                       │
                       ▼
                  RESPONSE
```

---

### 8. gRPC SERVICE

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

---

### 9. HEALTH CHECK

Health endpoints answer:

```text
Is the process alive?
```

Do not confuse this with:

```text
Is every dependency healthy?
```

A useful distinction is:

```text
LIVENESS
    process is running

READINESS
    service is able to serve traffic
```

---

### 10. AUTHORIZATION EXAMPLE

Conceptually:

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

Then:

```text
certificate identity
        │
        ▼
mapped service/user identity
        │
        ▼
role
        │
        ▼
permission
```

---

### 11. RATE LIMITER

A simplified asynchronous limiter:

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

This is educational, not a production distributed rate limiter.

In a multi-instance deployment:

```text
SERVICE A ─┐
SERVICE B ─┼──► shared rate-limit mechanism
SERVICE C ─┘
```

would generally be necessary for globally consistent limits.

---

### 12. AUTHENTICATION FLOW

Example:

```text
CLIENT
  │
  │ mTLS handshake
  ▼
SERVER
  │
  │ verify certificate
  ▼
CLIENT IDENTITY
  │
  ▼
AUTHORIZATION POLICY
  │
  ├──── ALLOW
  │
  └──── DENY
```

---

### 13. OBSERVABILITY FLOW

```text
REQUEST
   │
   ├── TRACE ID
   │
   ├── request start
   │
   ├── authentication result
   │
   ├── authorization result
   │
   ├── handler execution
   │
   ├── latency
   │
   └── response status
```

---

### 14. FAILURE MATRIX

Build a table during the final lab:

```text
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
│ missing resource          │ 404           │
│ malformed JSON             │ 4xx           │
│ rate limit exceeded        │ 429           │
│ internal failure           │ 5xx           │
│ upstream timeout           │ timeout/5xx   │
└────────────────────────────┴───────────────┘
```

The exact status returned should match your API's documented contract.

---

----------------------------------------------
### THE NETWORK PACKET MENTAL MODEL
----------------------------------------------

By the end of the week, understand this:

Suppose a client performs:

```text
GET https://api.example.com/users/42
```

A conceptual chain is:

```text
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

With gRPC:

```text
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

With HTTP/3:

```text
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

With mTLS:

```text
APPLICATION
    │
    ▼
mTLS
    │
    ├── authenticate server
    │
    ├── authenticate client
    │
    ├── establish keys
    │
    └── protect traffic
```

---

### COMPLETE SECURITY MENTAL MODEL

```text
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

Do not collapse these into one concept.

```text
Certificate
    ≠ permission

Authentication
    ≠ authorization

Encryption
    ≠ authentication

Hash
    ≠ encryption

TLS
    ≠ HTTP

mTLS
    ≠ Zero Trust

JWT
    ≠ OAuth

UDP
    ≠ QUIC
```

These distinctions are some of the most important things to retain.

---

### COMPLETE PROTOCOL MAP

```text
                         APPLICATION
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
         HTTP                gRPC                DNS
          │                   │
          │                HTTP/2
          │                   │
          └──────────┬────────┘
                     │
                    TLS
                     │
                  mTLS
                     │
          ┌──────────┴──────────┐
          │                     │
         TCP                   QUIC
          │                     │
          │                     ▼
          │                    UDP
          │                     │
          └──────────┬──────────┘
                     │
                     ▼
                    IP
                     │
                     ▼
             Ethernet / Wi-Fi
                     │
                     ▼
                  PHYSICAL
```

---

### COMPLETE CRYPTOGRAPHY MAP

```text
CRYPTOGRAPHY
     │
     ├── SYMMETRIC
     │      │
     │      ├── AES
     │      └── ChaCha20
     │
     ├── HASHING
     │      │
     │      ├── SHA-256
     │      └── SHA-3
     │
     ├── PASSWORD KDF
     │      │
     │      ├── Argon2
     │      ├── scrypt
     │      └── bcrypt
     │
     ├── ASYMMETRIC
     │      │
     │      ├── RSA
     │      ├── ECC-based systems
     │      └── public/private key pairs
     │
     ├── SIGNATURES
     │      │
     │      ├── ECDSA
     │      ├── Ed25519
     │      └── RSA signatures
     │
     ├── KEY AGREEMENT
     │      │
     │      └── ECDHE
     │
     └── PROTOCOLS
            │
            └── TLS
```

The exact choice of primitive should always be driven by the protocol/library's supported secure constructions rather than by inventing a custom combination.

---

### COMPLETE PKI MAP

```text
                        ROOT CA
                           │
                           │ signs
                           ▼
                  INTERMEDIATE CA
                           │
             ┌─────────────┴─────────────┐
             │                           │
             ▼                           ▼
       SERVER CERT                  CLIENT CERT
             │                           │
        server.key                   client.key
             │                           │
             │                           │
             └────────── mTLS ───────────┘
```

Certificate validation:

```text
certificate
     │
     ▼
signature chain
     │
     ▼
trusted issuer
     │
     ▼
validity
     │
     ▼
key usage
     │
     ▼
extended key usage
     │
     ▼
identity / SAN
     │
     ▼
ACCEPT
```

---

### COMPLETE DISTRIBUTED-SYSTEM MENTAL MODEL

```text
SERVICE A
    │
    │ network call
    ▼
IDENTITY
    │
    ▼
mTLS
    │
    ▼
AUTHENTICATION
    │
    ▼
AUTHORIZATION
    │
    ▼
REQUEST
    │
    ▼
BUSINESS LOGIC
    │
    ▼
DATABASE
    │
    ▼
RESPONSE
```

But every arrow can fail:

```text
DNS failure
TCP timeout
TLS failure
certificate failure
authentication failure
authorization denial
rate limit
validation failure
database failure
response timeout
connection reset
process crash
```

Distributed systems engineering means designing for those failures rather than pretending they do not exist.

---

### FINAL WEEK PROJECT CHECKLIST

```text
NETWORKING

[ ] OSI
[ ] TCP/IP
[ ] Ethernet
[ ] MAC
[ ] ARP
[ ] IP
[ ] IPv4
[ ] IPv6 concepts
[ ] subnetting
[ ] routing
[ ] default gateway
[ ] ICMP
[ ] NAT
[ ] DNS
[ ] ports
[ ] sockets
[ ] TCP
[ ] UDP
[ ] flow control
[ ] congestion control
[ ] retransmission
[ ] framing
```

```text
HTTP

[ ] request
[ ] response
[ ] methods
[ ] headers
[ ] status codes
[ ] cookies
[ ] caching
[ ] ETag
[ ] REST
[ ] idempotency
[ ] proxies
[ ] reverse proxies
[ ] validation
[ ] Axum
```

```text
CRYPTOGRAPHY

[ ] confidentiality
[ ] integrity
[ ] authentication
[ ] symmetric encryption
[ ] AEAD
[ ] hashing
[ ] salts
[ ] password KDFs
[ ] public/private keys
[ ] signatures
[ ] key exchange
[ ] forward secrecy
[ ] random numbers
[ ] nonce concepts
```

```text
TLS / PKI

[ ] TLS
[ ] TLS 1.3
[ ] handshake
[ ] record layer
[ ] certificates
[ ] CA
[ ] CSR
[ ] certificate chain
[ ] trust store
[ ] SAN
[ ] key usage
[ ] EKU
[ ] serverAuth
[ ] clientAuth
[ ] HTTPS
[ ] rustls
```

```text
mTLS / ZERO TRUST

[ ] mutual TLS
[ ] client authentication
[ ] server authentication
[ ] machine identity
[ ] authentication
[ ] authorization
[ ] RBAC
[ ] ABAC
[ ] least privilege
[ ] microsegmentation
[ ] Zero Trust
```

```text
MODERN NETWORKING

[ ] HTTP/2
[ ] binary framing
[ ] streams
[ ] multiplexing
[ ] flow control
[ ] gRPC
[ ] Protobuf
[ ] unary RPC
[ ] server streaming
[ ] client streaming
[ ] bidirectional streaming
[ ] QUIC
[ ] HTTP/3
```

```text
SECURITY ENGINEERING

[ ] threat modeling
[ ] attack surface
[ ] sessions
[ ] API keys
[ ] JWT
[ ] OAuth concepts
[ ] password security
[ ] RBAC
[ ] rate limiting
[ ] token bucket
[ ] input validation
[ ] secure errors
[ ] secrets
[ ] logging
[ ] metrics
[ ] tracing
[ ] dependency security
```

```text
RUST

[ ] Tokio
[ ] async/await
[ ] TcpListener
[ ] TcpStream
[ ] UdpSocket
[ ] Axum
[ ] Serde
[ ] rustls
[ ] tokio-rustls
[ ] rcgen
[ ] Tonic
[ ] Prost
[ ] shared state
[ ] synchronization
[ ] Result / error handling
[ ] graceful shutdown
```

---

### FINAL STANDARD

At the end of the week, do not measure success by:

```text
"I managed to compile the examples."
```

Measure it by whether you can explain this without looking it up:

```text
A Rust client wants to call a secure service.

What happens?

DNS resolves the service name.

The client creates a socket.

TCP establishes a transport connection
(or QUIC establishes a QUIC connection).

TLS negotiates cryptographic parameters.

The server presents its certificate.

The client validates the certificate chain.

In mTLS, the client also presents its certificate.

The server validates the client identity.

Both sides establish protected traffic keys.

HTTP or HTTP/2 carries the application protocol.

REST or gRPC defines application semantics.

Authentication determines identity.

Authorization determines permissions.

The service validates input.

Business logic executes.

Metrics and logs record what happened.

The response travels back through the stack.
```

That is the real objective.

Not:

```text
Tokio
Axum
Rustls
Tonic
```

as isolated libraries.

But:

```text
APPLICATION
     ↓
PROTOCOL
     ↓
SECURITY
     ↓
TRANSPORT
     ↓
NETWORK
     ↓
LINK
     ↓
PHYSICAL
```

with:

```text
IDENTITY
AUTHENTICATION
AUTHORIZATION
OBSERVABILITY
FAILURE HANDLING
```

present across the system.
