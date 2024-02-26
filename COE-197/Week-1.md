# Cloud Computing Technologies

raffy.ramirez@eee.upd.edu.ph

Course Requirements
- Each module has
	- An ending multiple-choice Quiz worth 100 points
	- A graded write-up worth 10 points - together all these write up answers will be equal to a quiz and can replace the lowest multiple choice Quiz
	- Starting from module 2, previous module Quizzes need to be accomplished first (Quizzes cannot be skipped)
	- Hands on (if available) problem sets may also replace one or more Quizzes for multiple modules - TBD

Module 1: Cloud Computing Overview and Introduction
Computer Paradigms
- Model or Paradigms - representation of abstract ideas into easily understandable form to identify, clarify and document complex, detailed concepts or functions for a specific purpose
- Models for using computers for distinct, overall tasks in historical order
	- Batch Computing
		- Batch providing started from Mainframe batch programs
		- Still used today for background processes that handle large IO data sets
	- Interactive Computing
		- Started with Time Sharing for scheduling processes that run on specific time slots
		- Originally developed on latter-day Mainframes, minicomputers and Unix
		- Associated IO devices: terminals, workstations and personal computers
	- Distributed Computing
		- Multiple, cooperating autonomous computers doing common or partitioned tasks over an interconnecting network
		- Client-Server Computing
			- Server - computer serving tasks over a network, not directly used for UI
			- Client - computer requesting tasks over a network, providing UI
		- Distributed Processing - servers interacting with others for distinct tasks
		- Parallel Processing - servers jointly serving the same tasks via common memory on a multiple CPU server or via a coordinated cluster
		- Cluster Computing - servers doing similar tasks with fault tolerance or ideal balancing
		- Grid Computing - combining servers in different geographic locations for common tasks
	- Utility Computing
		- Provision of server resources (CPU, memory, disk) resources on demand in a metered, standard manner similar to an electric utility
		- Originally attempted on OS level (e.g. Multics, pre-cursor of Unix), physical servers
	- Cloud Computing
		- Started as utility computing but now realized using hardware, network and storage virtualization technology and using a standard web service interface
		- 
## Cloud Computing Standard Definition
A definition and reference architecture for Cloud computing published by the US National Institute of Standards (NIST)
- Published in 2011, last updated 2012 but has not been updated since
The NIST reference architecture is for requirements of "what" Cloud Computing Services provide not "how" to provide it

#### NIST Cloud Computing Definition
Keywords
- Shared hardware and software - shared facilities enabled by economies of scale
- Software-defined - later will be seen as enabled by web services and Virtualization
- Metered - everything costs something in the cloud
- Network
- Tenancy
- Browser or API 
Essential Characteristics:
- On demand self service
- Broad network access
- Resource pooling
- Rapid elasticity
- Measured service

#### NIST Cloud Computing Service Models
NIST Cloud Computing Service models: (What service is provided)
- Software as a Service (SaaS)
	- Complete running applications are provided end users
- Platform as a Service (PaaS)
	- Platforms on which developers can create, provision and run custom built applications (possibly interfacing with other applications) are provided
	- Platform is only a building block of applications for end users
- Infrastructure as a Service (IaaS)
	- Infrastructure for computer virtual service, network and storage are provided
	- Infrastructure is only a building block for the platform on which developers can build, provision and run custom built applications
Notes
-Service models can be used in combination and PaaS can be built upon IaaS, SaaS upon PaaS
-Common to all services is an internal cloud network for the cloud account
-Iaas provides the most control over resource but is also more complex to administer, PaaS is next level for control and administration which SaaS is the simplest to use but least flexible

Two new service models
- CaaS - container as a service
	- Virtualization using containers inside an O/S
	- Managed Kubernetes for containers is internally built PaaS
- FaaS - function as a service (also called serverless)
	- Functions triggered by events (also called "serverless)
	- Built upon internally using IaaS and PaaS
Significance of Service Models
- Shows the boundary between Cloud Tenant and Cloud Provider Responsibility
	- Often called "Shared Responsibility" in the Cloud
- All other service models beyond IaaS can be built upon the previous one
	- SaaS can be built upon PaaS, PaaS can be built upon IaaS - this is often done by Cloud Providers to create new services

Cloud Computing Roles Today
- Cloud provider (CSP or Cloud Service Provider) - the entity providing cloud resources and manages the actual physical hardware that ultimately actually runs the cloud services
	- Usually provides (paid) support services for cloud users
	- Provides (paid) certifications 
- Managed Services Provider or MSP for Cloud (Cloud MSP)
- Cloud Security and Compliance Assessor
- Cloud Carrier - ISP or Telecom Provider (Telco)
Shared Responsibility for Security and Management
- CSP is responsible for security the cloud infrastructure itself as a whole
- Cloud user is responsible for
	- Provisioning and managing user-defined cloud resources, including connecting to CSP data center from on-premise data center or end users
	- Security, reliability and performance of custom applications running on cloud user-defined resources
	- Backup and recover of data used by cloud user applications built on cloud resources
Cloud Computing SLA, Billing, and Usage
- SLA (Service Level Agreement), Billing and Usage

NIST Cloud Computing Deployment Models
- Public Cloud
	- Cloud provider is a 3rd party providing physical data centers independent of any its cloud customers. Ex. Amazon Web Services, Oracle Cloud Infrastructure
- Private Cloud
	- Cloud is implemented on-premise either by conventional DIY hardware or by using 3rd party vendor cloud application. Ex. Openstack on x76 servers, Oracle Cloud-at-customer
- Community Cloud
	- Multiple cloud users from separate entities (independent or allied) use a common cloud provider. Ex. US Gov Cloud
- Hybrid Cloud
	- Combination of public cloud with private cloud. Ex. Private cloud is Disaster Recovery site