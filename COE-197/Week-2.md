## Week 2

Physical Infrastructure used by Cloud COmputing
- Network hardware
  - OSI Model, historical networks
  - Ehternet, Layer 2, Switches, Open Network Switches
  - Internet, TCP/IP, Routing, Layer 3 Switches
  - Alternatives - Infiniband and RDMA, PCI-E, CXL
- Server Hardware
  - Unix and Unix Servers
  - IBM perosnal Computer, x86 Processor, x86 Servers
  - Open SOurce and Linux
- Storage Hardware
  - Magnetic Disk, SSD
  - Storage Servers, SAN, NAS, Converged Storage
- Hyperconverged Servers
- Physical Infra/Data Centers   

**OSI Network Mode**

- Open Systems Interconnect (OSI) standard - formed in 1983 by the International Telecommunications Union (ITU)
- OSI 7-layer model of communicaiton protocols
  - A model for communications between sending and receiving computers, with layer 1 starting 

**Internet Protocol**

- IP or Internet Protocol
  - Defines a packet as having a header and data (or payload) part
  - Defines a network node or host (sender or receiver) as having an IP address:
    - IPv4 (version 4) uses a 32-bit address, denoted by 4 octets x.x.x.x
    - IPv6 (version 6) uses a 128-bit address, denoted by 8 hextets x:x:x:x:x:x:x:x
  - The header also has the
    - Sender IP address (for the origin node of packet)
    - Receiver IP address 

**IPv4 Subnet**

- IPv4 subnets (commonly used inside Cloud environment)
  - A subnet is the upper bit part of an IP address, denoting a group of hosts
  - When an IP address has its subnet part removed (i.e. masked out), the remaining bits denote the host part of the IP address
  - Netmask and CIDR (Common Internet Domain Range) notation
    - Netmask notation uses the octets of the upper part of the IP address
    - Ex. 192.168.2.1 with netmask 255.255.255.0 means subnet 192.168.2.0 and host 1
    - Classless Internet Domain Routing (CIDR) uses the count of bits of the netmask
    - Ex. 192.168.2.1/24 means subnet is 24 bits (3 upper octets) 192.168.2.0 and host 1
  - IP addresses that end with host part of all 0 or all 1 bits are unusable (reserved-because its used for broadcast)

**IP Addresses**

- Internet Public IP and Private IPv4 addresses
  - Internet Public Ip addresses are controlled by IANA

**IP or Layer 3 (L3) Network**

- Layer 3 or L3 Network
  - A network where hosts with unique IP addresses communicate via IP packets
  - 