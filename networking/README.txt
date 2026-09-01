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
