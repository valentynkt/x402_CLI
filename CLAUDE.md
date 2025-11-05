# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **Solana x402 AI Hackathon** project workspace (October 28 - November 11, 2025). The project uses the **BMAD Framework v6.0.0-alpha.5** for AI-driven agile development methodology.

**Current State:** Research and framework setup complete. No implementation code yet - ready to start building.

## Hackathon Context

- **Event:** Solana x402 AI Hackathon
- **Deadline:** November 11, 2025 (submissions close)
- **Winners Announced:** November 17, 2025
- **Total Prizes:** $100,000+ across 13 tracks
- **Focus:** AI agents using x402 payment protocol for autonomous blockchain payments

### Key Requirements
- Must integrate x402 protocol (HTTP 402 + stablecoin payments)
- Must be open source
- Must deploy to Solana devnet or mainnet
- Must include 3-minute demo video
- Must have documentation (README, architecture diagrams)

### x402 Protocol Core Concept
HTTP-native crypto payments enabling AI agents to transact autonomously:
1. Client requests resource → Server responds `402 Payment Required`
2. Client constructs USDC payment transaction on Solana
3. Payment verified on-chain via facilitator
4. Resource delivered (<2 seconds total)

## Technology Stack

### Blockchain Layer
- **Primary:** Solana (400ms finality, $0.00025 tx cost)
- **Secondary:** Base (for cross-chain scenarios)
- **Payment Token:** USDC stablecoin
- **Protocol:** x402 (HTTP 402 + on-chain verification)

### Available SDKs & Tools
- **Corbits/Faremeter** - Open-source Solana-first framework (LGPL-3.0)
- **Coinbase CDP SDK** - Official x402 implementation (77-80% market share)
- **PayAI Network** - 7-chain facilitator with fee coverage
- **Switchboard** - Oracle data feeds with x402 integration
- **x402-mcp** - Model Context Protocol integration

### Development Framework
- **BMAD Framework** - AI-agent driven workflow system
  - 19+ specialized agents (PM, Architect, Developer, etc.)
  - 60+ workflows covering complete SDLC
  - Scale-adaptive (Levels 0-4 based on complexity)
  - 3 development tracks: Quick Flow, BMad Method, Enterprise Method

## Project Structure

```
/Hackaton/
├── bmad/                      # BMAD Framework installation
│   ├── core/                  # Core workflows & BMad Master agent
│   ├── bmm/                   # BMad Method Module (project management)
│   ├── bmb/                   # BMad Builder Module (create agents/workflows)
│   ├── cis/                   # Creative Innovation Strategy Module
│   ├── _cfg/manifest.yaml     # Installed modules manifest
│   └── core/config.yaml       # Global configuration
├── hackathon-research/        # Complete hackathon documentation
│   ├── guides/
│   ├── reference/
│   └── tools/
├── docs/                      # Generated documentation output
└── .claude/                   # Claude Code configuration
```

## Key Commands - BMAD Workflows

BMAD uses slash commands to activate specialized AI agent workflows. **Always use fresh chat sessions for workflows** to avoid context hallucinations.

### Project Initialization
- `/bmad:bmm:workflows:workflow-init` - Initialize new project (determines level 0-4)
- `/bmad:bmm:workflows:workflow-status` - Check current status and next steps

### Phase 1: Analysis (Optional)
- `/bmad:core:workflows:brainstorming` - Interactive brainstorming sessions
- `/bmad:cis:workflows:innovation-strategy` - Identify disruption opportunities
- `/bmad:bmm:workflows:brainstorm-project` - Project-specific brainstorming
- `/bmad:bmm:workflows:product-brief` - Create product vision document

### Phase 2: Planning (Required)
- `/bmad:bmm:workflows:prd` - Create Product Requirements Document (Level 2-4)
- `/bmad:bmm:workflows:tech-spec` - Create Technical Specification (Level 0-1)
- `/bmad:bmm:workflows:research` - Conduct market/technical/user research

### Phase 3: Solutioning (Level 2-4 Only)
- `/bmad:bmm:workflows:architecture` - Make architectural decisions
- `/bmad:bmm:workflows:create-epics-and-stories` - Transform PRD into stories
- `/bmad:bmm:workflows:epic-tech-context` - Generate technical specifications
- `/bmad:bmm:workflows:solutioning-gate-check` - Validate before implementation

### Phase 4: Implementation (Required)
- `/bmad:bmm:workflows:sprint-planning` - Generate sprint tracking file
- `/bmad:bmm:workflows:create-story` - Create next user story
- `/bmad:bmm:workflows:story-context` - Assemble dynamic story context
- `/bmad:bmm:workflows:dev-story` - Implement story with tasks/tests
- `/bmad:bmm:workflows:code-review` - Senior developer review
- `/bmad:bmm:workflows:story-ready` - Mark story ready (TODO → IN PROGRESS)
- `/bmad:bmm:workflows:story-done` - Mark story complete (→ DONE)

### Post-Implementation
- `/bmad:bmm:workflows:retrospective` - Review epic completion
- `/bmad:bmm:workflows:correct-course` - Navigate significant changes

### Builder Tools
- `/bmad:bmb:workflows:create-agent` - Build custom BMAD agents
- `/bmad:bmb:workflows:create-workflow` - Build custom workflows
- `/bmad:bmb:workflows:create-module` - Build complete modules

### Collaboration
- `/bmad:core:workflows:party-mode` - Multi-agent group discussions

## Architecture Principles

### BMAD Methodology
1. **Agent-Based Workflow Orchestration** - Specialized AI agents collaborate via defined workflows
2. **Story-Centric Implementation** - Backlog → Drafted → Ready → In Progress → Review → Done
3. **Just-In-Time Context Loading** - Never pre-load context; agents load dynamically when activated
4. **Scale-Adaptive System** - Automatically adjusts complexity based on project level:
   - **Level 0:** Single atomic change (bug fix, add endpoint) - tech-spec only
   - **Level 1:** Small feature (2-5 story points) - tech-spec only
   - **Level 2:** Medium feature (5-13 points) - PRD + architecture
   - **Level 3:** Large feature/module (13-21 points) - Full methodology
   - **Level 4:** System/platform (21+ points) - Enterprise method
5. **Fresh Chat Per Workflow** - Always start new chat to prevent hallucinations

### Hackathon-Specific Principles
- **Autonomous AI Agents** - No human intervention after initial configuration
- **Micropayment Economics** - Optimize for <$0.01 transactions
- **Fast Settlement** - Target <2 second payment confirmation
- **On-Chain Verification** - All payments verified via Solana blockchain
- **Policy-Enforced Safety** - Agent behavior constraints via payment policies

## Critical Configuration Files

### BMAD Configuration
- **Global Config:** `bmad/core/config.yaml`
  ```yaml
  user_name: Valik
  communication_language: English
  document_output_language: English
  output_folder: '{project-root}/docs'
  ```
- **Module Manifest:** `bmad/_cfg/manifest.yaml` (tracks installed modules)
- **Workflow Status:** `docs/bmm-workflow-status.yaml` (created after workflow-init)

### Hackathon Resources (Read-Only Reference)
Located in `hackathon-research/`:
- `x402-protocol-specification.md` - Complete protocol technical specs (495 lines)
- `hackathon-rules-and-tracks.md` - All 13 tracks, requirements, deadlines (689 lines)
- `technical-stack-reference.md` - SDKs, APIs, code examples (1,121 lines)
- `ecosystem-tools-reference.md` - Corbits, PayAI, x402scan, Crossmint guides
- `market-landscape.md` - Market analysis, competitors, opportunities (1,011 lines)
- `sponsor-technologies.md` - Visa TAP, ATXP, Switchboard, CDP, Gradient (1,456 lines)

## Development Workflow

### Recommended Approach for This Hackathon

1. **Brainstorm & Research** (Analysis Phase)
   - Run `/bmad:cis:workflows:innovation-strategy` to identify winning opportunity
   - Review `hackathon-research/hackathon-rules-and-tracks.md` for prize tracks
   - Recommended target: "Best x402 Agent Application" ($10,000 prize)

2. **Initialize Project** (Planning Phase)
   - Run `/bmad:bmm:workflows:workflow-init` to determine project level
   - Expected level: 1-2 (small-to-medium hackathon project)
   - Create tech-spec (Level 0-1) OR product-brief + PRD (Level 2+)

3. **Design Architecture** (Solutioning Phase - if Level 2+)
   - Run `/bmad:bmm:workflows:architecture` for key technical decisions
   - Focus on: x402 integration, Solana deployment, AI agent framework choice
   - Reference `technical-stack-reference.md` for SDK integration patterns

4. **Implement Stories** (Implementation Phase)
   - Run `/bmad:bmm:workflows:sprint-planning` to track progress
   - Iterate: create-story → story-context → dev-story → code-review → story-done
   - Use `story-context` to dynamically load relevant docs before each story

5. **Final Submission**
   - Create 3-minute demo video
   - Write README with architecture diagrams
   - Deploy to Solana devnet/mainnet
   - Submit before November 11, 2025 deadline

### Working with BMAD

**DO:**
- Use fresh chat sessions for each workflow execution
- Run `workflow-status` when unsure of next step
- Let workflows guide you through BMAD methodology
- Reference hackathon-research docs for x402/Solana specifics
- Verify methods/properties exist before using (per global CLAUDE.md)

**DON'T:**
- Skip phases in the BMAD methodology
- Pre-load large amounts of context (use just-in-time loading)
- Reuse chat sessions across multiple workflows
- Make assumptions about project structure - let workflow-init determine it

## Prize Tracks Reference

Quick reference to key prize tracks (see `hackathon-research/hackathon-rules-and-tracks.md` for full details):

| Track | Prize | Focus |
|-------|-------|-------|
| Best x402 Agent Application | $10,000 | Most compelling x402 use case |
| Best Corbits Project | $5,000 | Built with Corbits/Faremeter SDK |
| Best Agent Money Protocol Hack | $5,000 | Innovative payment protocols |
| Visa TAP Integration | $10,000 | Trusted Agent Protocol |
| ATXP Integration | $10,000 | Multi-protocol agent transactions |
| Switchboard Integration | $5,000 | Oracle data feeds with x402 |
| CDP Embedded Wallets | $5,000 | Coinbase wallet infrastructure |
| Gradient Parallax | $5,000 | Distributed AI on Solana |

**Tip:** Projects can win multiple tracks simultaneously. Target 2-3 compatible tracks for maximum prize potential.

## Common Pitfalls

1. **Context Overload** - BMAD agents work best with fresh, focused context. Don't dump entire codebases into a single chat.

2. **Skipping workflow-init** - Always initialize with workflow-init to let BMAD determine the right level and track.

3. **Wrong SDK Choice** - For Solana-first hackathon project, Corbits/Faremeter is recommended over CDP SDK (which favors Base/multi-chain).

4. **Ignoring x402 Requirements** - Every submission MUST integrate x402 protocol. Review `x402-protocol-specification.md` early.

5. **Late Deployment** - Solana devnet/mainnet deployment required. Don't wait until last day to deploy.

## Additional Resources

### BMAD Documentation
Located in `bmad/bmm/docs/`:
- `quickstart.md` - Getting started with BMAD
- `agents-guide.md` - Understanding specialized agents
- `scale-adaptive-system.md` - Project level determination logic
- `troubleshooting.md` - Common issues and solutions

### External Links
- x402 Protocol Docs: https://docs.x402.org
- Corbits Documentation: https://docs.corbits.ai
- Solana Developer Docs: https://docs.solana.com
- BMAD GitHub: (refer to installation method via NPM)

---

**Remember:** This is a hackathon project with a November 11, 2025 deadline. Balance thoroughness with speed. Use BMAD workflows to maintain quality while moving fast.
