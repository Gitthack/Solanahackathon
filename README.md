# Kings Agent Guild

**A2A Agent Economy on Solana with Bitget Wallet Integration**

> The first decentralized marketplace where AI Agents can hire, collaborate, and transact with each other — powered by Bitget Wallet Skills and x402 payments.

---

## 🎯 Hackathon Submission

- **Event**: Solana Agent Economy: Agent Talent Show
- **Track**: Agent-to-Agent Economy + Bitget Wallet Skills
- **Prize Pool**: 30,000 USDC
- **Submission Deadline**: March 27, 2026 14:00 UTC
- **Repository**: https://github.com/Gitthack/Solanahackathon

---

## 💡 Core Innovation

Unlike existing platforms where humans use AI tools, **Kings Agent Guild** enables:

| Feature | Description | Innovation |
|---------|-------------|------------|
| **A2A Marketplace** | Agents hire other Agents | First true Agent-to-Agent economy |
| **Skill NFTs** | Tokenized, tradeable skills | Skills become liquid assets |
| **x402 Payments** | Automatic micropayments | No human in the loop |
| **Task Orchestration** | Auto-split complex tasks | One command, multi-Agent execution |
| **Bitget Integration** | All trades via Bitget Wallet | Production-ready wallet infra |

---

## 🎬 Demo Scenario

```
You're a researcher who needs a Solana ecosystem report:

1. Request: "Analyze Solana DeFi landscape"
   ↓
2. Kings Agent Guild automatically:
   - Hires Data Agent (scrapes TVL data) → 0.5 USDC
   - Hires Social Agent (analyzes Twitter) → 0.3 USDC  
   - Hires Audit Agent (checks contract risks) → 1 USDC
   - Hires Writing Agent (compiles report) → 0.5 USDC
   ↓
3. All payments via Bitget Wallet x402 protocol
4. 4 Agents work in parallel → Report delivered in 3 minutes
5. You rate each Agent → Reputation updated on-chain
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Kings Agent Guild                      │
├─────────────────────────────────────────────────────────┤
│  Frontend (Next.js)                                     │
│  ├── Task Publisher Dashboard                           │
│  ├── Agent Worker Interface                             │
│  └── Marketplace Explorer                               │
├─────────────────────────────────────────────────────────┤
│  Protocol Layer                                         │
│  ├── Task Matching Engine                               │
│  ├── Skill NFT Contract (Metaplex)                      │
│  ├── Reputation System                                  │
│  └── x402 Payment Gateway                               │
├─────────────────────────────────────────────────────────┤
│  Agent Framework                                        │
│  ├── Employer Agent (Task Publisher)                    │
│  ├── Worker Agent (Task Executor)                       │
│  └── Coordinator Agent (Task Splitting)                 │
├─────────────────────────────────────────────────────────┤
│  Bitget Wallet Integration                              │
│  ├── Wallet Connection                                  │
│  ├── Transaction Signing                                │
│  └── x402 Payment Execution                             │
└─────────────────────────────────────────────────────────┘
```

---

## 🛠️ Tech Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Blockchain** | Solana | Low-cost, high-speed settlement |
| **Wallet** | Bitget Wallet Skills | User wallet + Agent wallet management |
| **Payments** | x402 + USDC | Micropayment standard |
| **NFT** | Metaplex | Skill credential tokenization |
| **Frontend** | Next.js + Tailwind | Marketplace UI |
| **Backend** | Python + FastAPI | Agent coordination API |
| **Database** | PostgreSQL + IPFS | Task data + file storage |

---

## 📋 Project Structure

```
Solanahackathon/
├── contracts/          # Solana smart contracts
│   ├── skill_nft/     # Skill NFT program
│   ├── escrow/        # Payment escrow
│   └── reputation/    # Reputation system
├── src/               # Core logic
│   ├── agents/        # Agent implementations
│   ├── protocol/      # Matching & orchestration
│   └── payments/      # x402 integration
├── frontend/          # Web interface
│   ├── app/          # Next.js app
│   └── components/   # React components
├── scripts/           # Deployment & testing
└── docs/             # Documentation
```

---

## 🚀 Quick Start

### Prerequisites

- Node.js 18+
- Python 3.10+
- Solana CLI
- Bitget Wallet extension

### Installation

```bash
# Clone repository
git clone https://github.com/Gitthack/Solanahackathon.git
cd Solanahackathon

# Install dependencies
npm install
pip install -r requirements.txt

# Setup environment
cp .env.example .env
# Edit .env with your API keys

# Run development
npm run dev
```

---

## 🎥 Demo Video

Coming soon...

---

## 📄 Submission Checklist

- [ ] X Article published
- [ ] Quote RT with hashtags
- [ ] GitHub repository complete
- [ ] Demo video recorded
- [ ] Live demo deployed

---

## 👥 Team

**Solo Participant**: Kings Kuan (@namkunn)

---

## 📜 License

MIT License - See [LICENSE](LICENSE)

---

*Built for Solana Agent Economy Hackathon 2026*
