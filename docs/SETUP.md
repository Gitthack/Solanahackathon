# Kings Agent Guild - Setup Guide

## Prerequisites

```bash
# 1. Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# 2. Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# 3. Install Node.js dependencies
npm install -g @coral-xyz/anchor-cli

# 4. Verify installations
solana --version
anchor --version
```

## Project Structure

```
contracts/
├── Anchor.toml          # Anchor configuration
├── Cargo.toml           # Rust workspace
├── programs/            # Smart contracts
│   ├── skill_nft/      # Skill NFT program
│   ├── escrow/         # Payment escrow
│   └── reputation/     # Reputation system
├── tests/              # Integration tests
└── migrations/         # Deployment scripts
```

## Quick Start

### 1. Initialize Project
```bash
cd contracts
anchor init kings-agent-guild --force
```

### 2. Configure Solana
```bash
# Set to Devnet
solana config set --url devnet

# Create wallet
solana-keygen new --outfile ~/.config/solana/id.json

# Airdrop SOL for testing
solana airdrop 2
```

### 3. Build & Deploy
```bash
anchor build
anchor deploy --provider.cluster devnet
```

### 4. Run Tests
```bash
anchor test
```

## Bitget Wallet Integration

### Installation
```bash
npm install @bitget-wallet/sdk
```

### Basic Usage
```typescript
import { BitgetWallet } from '@bitget-wallet/sdk';

const wallet = new BitgetWallet({
  network: 'solana-devnet',
});

// Connect
await wallet.connect();

// Sign transaction
const signature = await wallet.signTransaction(transaction);
```

## Environment Variables

Create `.env`:
```
SOLANA_RPC_URL=https://api.devnet.solana.com
ANCHOR_WALLET=~/.config/solana/id.json
BITGET_WALLET_API_KEY=your_api_key
```

## Troubleshooting

### Issue: SSL connection error
**Solution**: Use VPN or try alternative mirror
```bash
# Alternative installation
cargo install solana-cli
```

### Issue: Anchor build fails
**Solution**: Update Rust
```bash
rustup update
```

### Issue: Insufficient funds
**Solution**: Request more airdrop
```bash
solana airdrop 2 --url devnet
```
