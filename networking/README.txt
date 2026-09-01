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


