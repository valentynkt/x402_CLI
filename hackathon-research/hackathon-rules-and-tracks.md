# Hackathon Rules & Tracks

## Official Information

**Event:** Solana x402 AI Hackathon
**Website:** https://solana.com/x402/hackathon
**Start Date:** October 28, 2025
**Submission Deadline:** November 11, 2025
**Winners Announced:** November 17, 2025

---

## Submission Requirements

All participants must adhere to these mandatory requirements:

### 1. Open Source
**Requirement:** All code must be open sourced
**Details:**
- Repository must be public
- Clear licensing (MIT, Apache 2.0, GPL, etc.)
- Complete source code available
- No proprietary or closed components

### 2. x402 Integration
**Requirement:** Projects must integrate x402 protocol or related agent infrastructure with Solana
**Details:**
- Must use x402 payment protocol
- OR use related agent infrastructure (ATXP, Visa TAP, etc.)
- Integration with Solana blockchain required
- Can support multiple chains, but Solana must be included

### 3. Deployment
**Requirement:** All programs must be deployed to Solana devnet or mainnet
**Details:**
- Smart contracts must be deployed (if applicable)
- Applications must be accessible
- Devnet acceptable for testing
- Mainnet preferred for production-ready projects

### 4. Demo Video
**Requirement:** Submit a demo video (maximum 3 minutes)
**Details:**
- Maximum length: 3 minutes
- Must showcase your project in action
- Clear demonstration of features
- Explain problem solved and solution approach

### 5. Documentation
**Requirement:** Include documentation on how to run and use your project
**Details:**
- README.md with setup instructions
- Installation steps
- Usage guide
- API documentation (if applicable)
- Architecture overview

---

## Prize Structure

### Total Prize Pool: $50,000+

**Core Tracks:** 5 × $10,000 = $50,000
**Sponsor Bounties:** 8 × $5,000-$10,000 = $50,000+

**Grand Total:** $100,000+ across 13 tracks

---

## Core Tracks (5 Tracks × $10,000)

### Track 1: Best Trustless Agent

**Prize:** $10,000
**Focus:** Autonomous agents with identity, reputation, and validation systems

**Requirements:**
- Build agents that can operate independently
- Implement identity verification systems
- Create reputation tracking mechanisms
- Develop validation frameworks for autonomous operations
- Address "Know Your Agent" (KYA) challenges

**Key Concepts:**
- Agent authentication
- Digital identity for agents
- Multi-agent trust frameworks
- Security and fraud detection
- Compromised agent credential protection

**Evaluation Criteria:**
- Identity system robustness
- Reputation mechanism design
- Autonomous operation capabilities
- Security implementation
- Scalability of trust framework

---

### Track 2: Best x402 API Integration

**Prize:** $10,000
**Focus:** Agent-to-agent payments and micropayments with x402

**Requirements:**
- Implement x402 payment flows
- Enable autonomous API payments
- Create agent-to-agent transaction systems
- Demonstrate micropayment economics

**Technical Stack:**
- x402 protocol implementation (Corbits, PayAI, or CDP)
- Solana SPL tokens (USDC)
- HTTP 402 status handling
- Payment verification systems

**Use Cases:**
- AI agents paying for data access
- Autonomous API consumption
- Machine-to-machine payments
- Per-request billing systems

**Evaluation Criteria:**
- x402 implementation quality
- Payment flow reliability
- Integration ease
- Novel use case demonstration
- Performance and latency

---

### Track 3: Best MCP Server

**Prize:** $10,000
**Focus:** Model Context Protocol servers connecting AI agents to payments

**What is MCP:**
Model Context Protocol (MCP) is an open standard introduced by Anthropic in November 2024 to standardize how AI systems integrate with external tools, systems, and data sources.

**Requirements:**
- Build MCP servers with payment integration
- Enable AI agents to access paid tools
- Implement x402 payment flows in MCP context
- Create monetizable AI services

**Technical Implementation:**
- Use x402-mcp wrapper package
- Define paidTools with pricing
- Integrate USDC payments on Base or Solana
- Support autonomous tool discovery

**Available Payment MCP Servers (Reference):**
- Bitnovo Pay (cryptocurrency payments)
- Alby Bitcoin Payments (Lightning Network)
- AlipayPlus integration
- Antom Checkout Payment

**Resources:**
- MCP documentation: Anthropic's official docs
- x402-mcp: Light wrapper for paid tools
- Vercel's x402-mcp implementation

**Evaluation Criteria:**
- MCP standard compliance
- Payment integration quality
- Tool usefulness and practicality
- Documentation clarity
- Ease of deployment

---

### Track 4: Best x402 Dev Tool

**Prize:** $10,000
**Focus:** SDKs, libraries, frameworks, or infrastructure to accelerate x402 development on Solana

**Requirements:**
- Create developer tools for x402
- Build SDKs or libraries
- Develop infrastructure components
- Simplify x402 integration

**Existing Tools (Build Upon or Complement):**
- Corbits SDK (Solana-first)
- Coinbase CDP SDK
- PayAI facilitator
- MCPay.tech

**Potential Project Types:**
- Testing frameworks for x402
- Payment verification libraries
- Developer dashboards
- Integration templates
- Monitoring and analytics tools
- Code generators
- Debugging utilities
- Performance profiling tools

**Evaluation Criteria:**
- Developer experience improvement
- Integration simplification
- Documentation quality
- Code quality and maintainability
- Community value
- Innovation level

---

### Track 5: Best x402 Agent Application

**Prize:** $10,000
**Focus:** Practical AI agent applications that use x402 for autonomous payments

**Requirements:**
- Build functional AI agent application
- Demonstrate real-world use case
- Integrate x402 payments seamlessly
- Show autonomous operation

**Application Categories:**

**Trading & Finance:**
- Autonomous trading agents
- DeFi yield optimizers
- Arbitrage bots with paid data feeds
- Portfolio management agents

**Content & Media:**
- AI-powered news aggregators
- Content recommendation engines
- Media licensing platforms
- Autonomous content creation

**Data & Analytics:**
- Real-time data analysis agents
- Market intelligence platforms
- Research assistants with paid sources
- Autonomous reporting systems

**Infrastructure:**
- RPC service resellers
- Compute resource brokers
- Storage marketplaces
- API gateway services

**Evaluation Criteria:**
- Real-world applicability
- User experience
- x402 integration quality
- Agent autonomy level
- Business model viability
- Innovation and creativity

---

## Sponsor Bounties (8 Tracks)

### Bounty 1: Best use of Visa TAP

**Prize:** $10,000
**Sponsor:** Visa
**Focus:** Create an agent that interacts with Visa's Trusted Agent Protocol

**Visa TAP Overview:**
- Launched October 14, 2025
- Developed with Cloudflare
- Built on HTTP Message Signature standard
- Aligned with Web Both Auth
- Interoperable with x402

**Technical Capabilities:**
- Agent intent verification
- Consumer recognition data (token IDs, loyalty accounts)
- Device identifiers for purchase history
- Three payment methods: hashed credentials, API tokens, IOUs

**Requirements:**
- Integrate Visa TAP with Solana application
- Demonstrate secure agent commerce
- Show interoperability with x402
- Enable trusted AI agent transactions

**Resources:**
- Visa Developer Center: https://developer.visa.com/capabilities/trusted-agent-protocol/overview
- GitHub repository with sample code
- Technical documentation

**Evaluation Criteria:**
- TAP integration quality
- Security implementation
- x402 interoperability
- Commercial applicability
- Innovation in agent trust

---

### Bounty 2: Best use of CASH

**Prize:** $10,000 in $CASH
**Sponsor:** Phantom
**Focus:** Create x402 agent that uses Phantom CASH for payments

**Note:** Specific technical details about "$CASH" token not fully documented in research. Likely Phantom-sponsored token or stablecoin variant.

**Likely Requirements:**
- Integration with Phantom wallet
- Use of CASH token for payments
- Mobile-first payment experiences
- Consumer-focused applications

**Resources:**
- Phantom wallet documentation
- CASH token specifications (check Phantom announcements)

---

### Bounty 3: Best Multi-Protocol Agent

**Prize:** $10,000 in ATXP credits
**Sponsor:** ATXP (Circuit & Chisel)
**Focus:** Create agent that combines multiple payment protocols

**ATXP Overview:**
- Agentic Transaction Protocol
- $19.2M seed funding (September 2025)
- Co-led by Primary Venture Partners and ParaFi Capital
- Backed by Stripe, Coinbase Ventures, Solana Ventures, Samsung Next, Polygon Labs

**Key Features:**
- Real-time decision-making
- Nested transactions
- Autonomous tool discovery
- Instant, delegated, low-cost micropayments
- Multi-protocol compatibility
- Compatible with x402

**Requirements:**
- Build agents operating across multiple protocols
- Demonstrate cross-chain capabilities
- Show autonomous tool discovery
- Enable nested transaction flows

**Evaluation Criteria:**
- Multi-protocol integration depth
- Cross-chain functionality
- Agent autonomy
- Transaction complexity handling
- Innovation in protocol composition

---

### Bounty 4: Best Usage of CDP Embedded Wallets

**Prize:** $5,000
**Sponsor:** Coinbase Developer Platform
**Focus:** Create autonomous agent with CDP Embedded Wallets

**CDP Embedded Wallets Overview:**
- Wallet integration without key management
- Human-readable verification
- Policy-enforced safety
- x402 integration ready

**Integration with x402:**
- Embedded Wallets + x402 = automatic monetization
- Enables agents to transact without managing keys
- Policy-enforced transactions
- User-friendly onboarding

**Real-World Example:**
Questflow integration has processed 130,000+ autonomous microtransactions using CDP Wallets v2 and x402.

**Requirements:**
- Integrate CDP Embedded Wallets
- Enable autonomous agent payments
- Demonstrate user-friendly onboarding
- Show policy enforcement

**Resources:**
- CDP documentation: https://docs.cdp.coinbase.com/
- Wallet API reference
- Integration guides

**Evaluation Criteria:**
- Wallet integration quality
- User experience
- Security implementation
- Agent autonomy
- Policy enforcement effectiveness

---

### Bounty 5: Parallax Eco Track

**Prize:** $5,000
**Sponsor:** Gradient Network
**Focus:** Best agent created on Gradient Parallax

**Gradient Network Overview:**
- $10M raised from Pantera, Multicoin, HSG
- Building on Solana blockchain
- Two core protocols: Lattica and Parallax

**Parallax Protocol:**
- Partitions AI models into smaller segments
- Processes concurrently across multiple devices
- Distributed computation model
- Keeps user data local (enhanced privacy)
- Enables AI inference across smartphones, laptops, computers

**Lattica Protocol:**
- Universal peer-to-peer data communication
- Fast and resilient data flow
- Supports AI inference to multi-agent collaboration
- Permissionless network
- Real-time coordination between models, agents, endpoints

**Requirements:**
- Build on Gradient infrastructure
- Use Parallax for distributed AI inference
- Demonstrate multi-agent collaboration
- Show decentralized AI deployment

**Evaluation Criteria:**
- Parallax protocol usage
- Distributed computation efficiency
- Privacy preservation
- Multi-agent coordination
- Innovation in decentralized AI

---

### Bounty 6: Best use of Switchboard

**Prize:** $5,000
**Sponsor:** Switchboard
**Focus:** Use Switchboard oracle protocol with x402

**Switchboard Overview:**
- Permission-less, customizable, multi-chain oracle
- General-purpose data feeds & verifiable randomness
- Built on Solana
- Protects $5B+ in assets for 50+ protocols

**Key Products:**
- **Surge:** Sub-100ms latency real-time price feeds (300x faster, 100x cheaper)
- **Oracle Quotes:** Lock-free on-chain integration
- **VRF:** Verifiable random number services
- **Attestation Program (V3):** TEE-based verifiable off-chain compute

**x402 Integration:**
- Achieved x402 compatibility via Corbits SDK (October 23, 2025)
- Only x402-compatible oracle at present
- AI agents can pay for data queries on-demand via SPL tokens
- Each query costs less than $0.001

**Requirements:**
- Integrate Switchboard oracle data
- Implement x402 payment for queries
- Build agent applications using real-time data
- Demonstrate on-demand data access

**Resources:**
- Switchboard documentation: https://docs.switchboard.xyz/
- x402 integration guide
- API reference

**Evaluation Criteria:**
- Oracle integration quality
- x402 payment implementation
- Data usage creativity
- Real-world applicability
- Performance optimization

---

### Bounty 7: Machine Economy Prize

**Prize:** $5,000 + platform credits
**Sponsor:** (TBD - specific platform not confirmed in research)
**Focus:** Create autonomous machine-to-machine payment systems

**Machine Economy Overview:**
- Autonomous transaction economy projected to reach $30 trillion by 2030 (Gartner)
- Machine-to-machine (M2M) transactions
- Devices exchanging services, data, energy, or value
- Autonomous identities for machines
- Independent payment capabilities

**Use Cases:**
- IoT device payments
- Vehicle-to-vehicle transactions
- Energy trading between devices
- Compute resource sharing
- Storage marketplaces
- Sensor data exchanges

**Requirements:**
- Build machine-to-machine payment systems
- Enable autonomous device transactions
- Demonstrate value exchange between machines
- Show scalable M2M economy model

**Note:** Specific technical requirements and platform details not fully documented. Check official hackathon Discord or announcements for "OM1 protocol" or machine economy sponsor details.

**Evaluation Criteria:**
- M2M transaction implementation
- Device autonomy
- Economic model viability
- Scalability
- Innovation in machine economy

---

### Bounty 8: Dark Open Source Repository Prize

**Prize:** $10,000
**Sponsor:** Dark Research (likely)
**Focus:** Best agent using Dark open source repositories

**Note:** Specific details about "Dark open source repositories" not conclusively documented in research.

**Potential Interpretation:**
- Dark Research collaboration (mentioned with Corbits building Mallory)
- Open-source AI tools and frameworks
- Privacy-focused development tools
- Decentralized code repositories

**Known Example:**
Dark Research + Corbits building **Mallory**: an open-source AI chat powered by x402 on Solana

**Likely Requirements:**
- Significant open-source contributions
- Well-documented code
- Community-focused development
- Reusable frameworks or libraries
- Transparent development process

**Recommendation:** Check official hackathon Discord/Telegram for clarification on Dark-specific requirements and available repositories.

---

## Organizing Partners

### Primary Organizers
- **Solana Foundation** - Blockchain infrastructure
- **Coinbase Developer Platform** - x402 protocol creator
- **Phantom** - Solana wallet provider

### Technology Partners
- **Merit Systems** - x402scan ecosystem explorer
- **Corbits** - x402 SDK provider
- **Crossmint** - Agent payments platform
- **PayAI Network** - x402 facilitator
- **ATXP** - Multi-protocol agent infrastructure
- **Gradient** - Distributed AI infrastructure
- **Coral Protocol** - Solana developer tools
- **Trends.fun** - (role not specified in research)

---

## Judging Criteria

While specific judging criteria not officially published, based on submission requirements and track descriptions, evaluation likely focuses on:

### Technical Excellence (30%)
- Code quality and architecture
- Security implementation
- Performance and optimization
- Bug-free execution
- Proper error handling

### Innovation (25%)
- Novel approach to problems
- Creative use of technologies
- Unique value proposition
- Differentiation from existing solutions

### x402 Integration (20%)
- Proper protocol implementation
- Payment flow reliability
- Seamless integration
- Effective use of ecosystem tools

### Real-World Applicability (15%)
- Solves actual problems
- Clear target users
- Viable business model
- Market demand validation

### Documentation & Presentation (10%)
- Clear documentation
- Compelling demo video
- Setup ease
- User experience

---

## Timeline

| Date | Event |
|------|-------|
| October 28, 2025 | Hackathon begins, registration opens |
| October 28 - November 11 | Building period (14 days) |
| November 11, 2025 23:59 UTC | Submission deadline |
| November 12-16, 2025 | Judging period |
| November 17, 2025 | Winners announced |

---

## Registration & Submission

**Registration:**
- Register at: https://solana.com/x402/hackathon
- After registration, participants receive Telegram chat invitation
- Telegram chat contains additional information and support

**Submission Process:**
(Specific submission platform not documented - likely Devpost or custom portal)

**Required Deliverables:**
1. Public GitHub repository with source code
2. Demo video (max 3 minutes) - uploaded to YouTube/Vimeo
3. Deployed application (Solana devnet/mainnet)
4. Documentation (README with setup/usage)
5. Project description
6. Track selection (primary track + any applicable bounties)

---

## Support & Resources

### Community Support
- **Telegram Chat** - Primary communication channel (join after registration)
- **Solana Discord** - Technical support for Solana development
- **x402 Foundation Discord** - Protocol-specific questions
- **Individual Partner Discords** - Tool-specific support

### Technical Resources
- Solana documentation: https://docs.solana.com/
- x402 docs: https://docs.cdp.coinbase.com/x402/welcome
- Ecosystem tools: Corbits, PayAI, Crossmint documentation
- GitHub repositories: Example projects and templates

---

## Important Notes

### Multi-Track Submission
Projects can potentially qualify for multiple tracks:
- 1 core track (Best Trustless Agent, Best x402 API Integration, etc.)
- Multiple applicable sponsor bounties

**Example:** An x402 agent using Switchboard oracles could compete in:
- Core track: Best x402 Agent Application
- Bounty: Best use of Switchboard
- Bounty: Best Usage of CDP Embedded Wallets (if used)

### Open Source Requirement
All code must be publicly accessible with appropriate licensing. Proprietary or closed-source components disqualify submissions.

### Deployment Requirement
Non-functional or non-deployed projects are ineligible. Devnet deployment minimum, mainnet preferred.

### Video Requirement
Projects without demo videos are incomplete submissions. Keep under 3 minutes.

---

## Hackathon Statistics (Reference from 2024)

**Solana AI Hackathon 2024:**
- 400+ projects submitted
- $200M+ combined market cap of winning tokens
- Strong ecosystem growth
- 300% increase in Solana AI projects post-hackathon

**Expected 2025 x402 Hackathon:**
- Similar or larger participation
- Focus on payment infrastructure
- Production-ready applications
- Commercial viability emphasis

---

**Official Hackathon Page:** https://solana.com/x402/hackathon
**Last Updated:** November 4, 2025
