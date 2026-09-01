🚀 One-Day Networking & Security Crash Course with Rust (8 Hours)
------------------------------------------------------------------

┌────────────────────────────────────────────────────────────────┐
│                    ONE-DAY MASTERY PATH                        │
├─────────┬─────────┬─────────┬─────────┬─────────┬──────────────┤
│ Hour 1  │ Hour 2  │ Hour 3  │ Hour 4  │ Hour 5  │  Hours 6-8   │
│ OSI+TCP │ HTTP+   │ TLS+    │ mTLS+   │ gRPC+   │  Final       │
│ +UDP    │ REST    │ Crypto  │ Zero    │ QUIC    │  Project     │
│         │         │         │ Trust   │         │              │
└─────────┴─────────┴─────────┴─────────┴─────────┴──────────────┘


📚 Hour 1: OSI Model, TCP/UDP Fundamentals (60 min)

0.10 OSI Model : 
---------------

┌─────────────────────────────────────────────┐
│ 7. Application: HTTP, DNS, SMTP             │
│ 6. Presentation: TLS/SSL, Encryption        │
│ 5. Session: RPC, NetBIOS                    │
│ 4. Transport: TCP, UDP, QUIC                │
│ 3. Network: IP, ICMP, Routing               │
│ 2. Data Link: Ethernet, MAC, ARP            │
│ 1. Physical: Cables, Radio, Fiber           │
└─────────────────────────────────────────────┘

Data Flow:
Application → TCP Segments → IP Packets → Ethernet Frames → Bits


0.25 TCP Vs UDP 
---------------

┌──────────────────┬──────────────────┐
│ TCP              │ UDP              │
├──────────────────┼──────────────────┤
│ • Connection     │ • Connectionless │
│ • Reliable       │ • Best effort    │
│ • Ordered        │ • Unordered      │
│ • Flow control   │ • No flow control│
│ • 20-60B header  │ • 8B header      │
│ • Slower         │ • Faster         │
└──────────────────┴──────────────────┘

TCP Handshake:
Client → SYN → Server
Client ← SYN-ACK ← Server  
Client → ACK → Server


0.59 Rust Implementation
------------------------

TCP and UDP Server/Client Hello.

> cargo run


📚 Hour 2: HTTP Protocol & REST APIs (60 min)

HTTP Protocol: (15 min)
-------------

┌─────────────────────────────────────────────┐
│ HTTP Request Format:                        │
│ GET /api/users HTTP/1.1                     │
│ Host: example.com                           │
│ Content-Type: application/json              │
│                                             │
│ {"id": 1}                                   │
├─────────────────────────────────────────────┤
│ HTTP Response Format:                       │
│ HTTP/1.1 200 OK                             │
│ Content-Type: application/json              │
│ Content-Length: 27                          │
│                                             │
│ {"name": "John", "age": 30}                 │
└─────────────────────────────────────────────┘

Methods: GET, POST, PUT, DELETE, PATCH
Status: 2xx Success, 3xx Redirect, 4xx Client Error, 5xx Server Error


REST API with Axum (45 min)
--------------------------

i. Home (GET /)

Request:
---------
GET / HTTP/1.1
Host: localhost:3000
Accept: */*

Response:
---------
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 17
REST API Running!


ii. Create User (POST /users)

Request:
---------
POST /users HTTP/1.1
Host: localhost:3000
Content-Type: application/json
{
  "id": 1,
  "name": "Alice",
  "email": "alice@example.com"
}

Response:
---------
HTTP/1.1 201 Created
content-type: application/json
content-length: 53
{"id":1,"name":"Alice","email":"alice@example.com"}


iii. List All Users (GET /users)

Request:
---------
GET /users HTTP/1.1
Host: localhost:3000
Accept: application/json

Response:
---------
HTTP/1.1 200 OK
content-type: application/json
content-length: 55
[{"id":1,"name":"Alice","email":"alice@example.com"}]


iv. Get User (GET /users/{id})

Request:
---------
GET /users/1 HTTP/1.1
Host: localhost:3000
Accept: application/json

Response:
---------
HTTP/1.1 200 OK
content-type: application/json
content-length: 51
{"id":1,"name":"Alice","email":"alice@example.com"}

v. Delete User (DELETE /users/{id})

Request:
---------
DELETE /users/1 HTTP/1.1
Host: localhost:3000

Response:
---------
HTTP/1.1 204 No Content

vi. 404 Not Found

Request:
---------
GET /users/1 HTTP/1.1

Response:
---------
HTTP/1.1 404 Not Found
content-type: application/json
content-length: 31

{"id":0,"name":"","email":""}



📚 Hour 3: TLS & Cryptography (60 min)

TLS Fundamentals: (20 min)
-----------------

┌─────────────────────────────────────────────┐
│ TLS 1.3 Handshake (1-RTT):                  │
│ Client → ClientHello → Server               │
│ Client ← ServerHello ← Server               │
│ Client ← Certificate ← Server               │
│ Client ← Finished ← Server                  │
│ Client → Finished → Server                  │
│ Client ↔ Encrypted Data ↔ Server            │
└─────────────────────────────────────────────┘

Encryption Types:
• Symmetric: AES (same key)
• Asymmetric: RSA, ECDHE (public/private)
• Hashing: SHA-256 (integrity)
• Signatures: ECDSA (authentication)

Cipher Suite: TLS_AES_256_GCM_SHA384

Rust TLS Implementation (40 min)
--------------------------------

> cargo run
: HTTPS / TLS Server listening on https://localhost:8443


📚 Hour 4: mTLS & Zero Trust Security (60 min)

mTLS: Concepts (20 min)
-----------------------

                 CA
            ┌──────────┐
            │  ca.crt  │
            └────┬─────┘
                 │
         signs both identities
            ┌────┴──────┐
            │           │
            ▼           ▼
      ┌─────────┐   ┌─────────┐
      │ SERVER  │   │ CLIENT  │
      │         │   │         │
      │server   │   │client   │
      │.crt     │   │.crt     │
      │server   │   │client   │
      │.key     │   │.key     │
      └────┬────┘   └────┬────┘
           │             │
           │             │
           │   mTLS      │
           │◄───────────►│
           │             │
           │             │
      verify client   verify server
           │             │
           └──────┬──────┘
                  ▼
            ENCRYPTED TLS


Zero Trust Principles:
1. Never trust, always verify
2. Least privilege access
3. Assume breach
4. Micro-segmentation
5. Continuous verification

Certificate Chain:
Root CA → Intermediate CA → Leaf Certificate


mTLS: Implementation (40 min)
-----------------------------

Need to create 3 identities:

CA
├── ca.crt
└── ca.key
Server
├── server.crt
└── server.key
Client
├── client.crt
└── client.key

Generate CA:
> openssl req -x509 -newkey rsa:2048 `
  -keyout ca.key `
  -out ca.crt `
  -days 365 `
  -nodes `
  -subj "/CN=Rustful Test CA" `
  -addext "basicConstraints=critical,CA:TRUE" `
  -addext "keyUsage=critical,keyCertSign,cRLSign"

CA Verification:
> openssl x509 -in ca.crt -text -noout |
    Select-String "CA:TRUE"


----------------------------------------------------

Generate the server private key and CSR:
> openssl req -newkey rsa:2048 `
  -keyout server.key `
  -out server.csr `
  -nodes `
  -subj "/CN=localhost"

The CN=localhost isn't enough by itself for modern TLS hostname validation. Therefore, we need to add the SAN extension to the CSR:

Create a SAN extension for the server:
> @"
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature,keyEncipherment
extendedKeyUsage=serverAuth
subjectAltName=DNS:localhost,IP:127.0.0.1
"@ | Set-Content server.ext

Sign the server CSR with the CA:
> openssl x509 -req `
  -in server.csr `
  -CA ca.crt `
  -CAkey ca.key `
  -CAcreateserial `
  -out server.crt `
  -days 365 `
  -extfile server.ext

Verify the server certificate chain:
> openssl verify -CAfile ca.crt server.crt

----------------------------------------------------

Generate the client private key and CSR:
> openssl req -newkey rsa:2048 `
  -keyout client.key `
  -out client.csr `
  -nodes `
  -subj "/CN=client"

Create a SAN extension for the client:
> @"
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature,keyEncipherment
extendedKeyUsage=clientAuth
"@ | Set-Content client.ext

Sign the client CSR with the CA:
> openssl x509 -req `
  -in client.csr `
  -CA ca.crt `
  -CAkey ca.key `
  -CAcreateserial `
  -out client.crt `
  -days 365 `
  -extfile client.ext


Verify the client certificate chain:
> openssl verify -CAfile ca.crt client.crt

Final files:

networking/
│
├── ca.crt
├── ca.key
│
├── server.crt
├── server.key
├── server.csr
├── server.ext
│
├── client.crt
├── client.key
├── client.csr
└── client.ext


Final Concept:

           Rustful Test CA
                │
      ┌─────────┴─────────┐
      │                   │
      ▼                   ▼
    server.crt           client.crt
    serverAuth            clientAuth
      │                   │
      │ SAN:              │
      │ localhost         │
      │ 127.0.0.1         │
      │                   │
      ▼                   ▼
   server.key          client.key


CLIENT                                      SERVER
  │                                            │
  │──── TCP connection ──────────────────────► │
  │                                            │
  │◄──── server.crt ────────────────────────── │
  │                                            │
  │ verify server.crt                          │
  │   ├─ signed by ca.crt? ✓                   │
  │   ├─ valid dates? ✓                        │
  │   ├─ serverAuth? ✓                         │
  │   └─ SAN localhost? ✓                      │
  │                                            │
  │◄──── "give me your certificate" ────────── │
  │                                            │
  │──── client.crt ─────────────────────────►  │
  │                                            │
  │                              verify client.crt
  │                                ├─ CA? ✓
  │                                ├─ valid? ✓
  │                                └─ clientAuth? ✓
  │                                            │
  │◄══════ encrypted TLS connection ═════════► │
  │                                            │


