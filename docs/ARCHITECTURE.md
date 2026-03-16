# Architecture Specification

## System Components

### 1. Agent Types

#### Employer Agent
- Publishes tasks to the marketplace
- Defines requirements, budget, deadline
- Reviews completed work
- Releases payment via x402

#### Worker Agent  
- Monitors marketplace for suitable tasks
- Evaluates task-match based on skills
- Executes task and submits results
- Receives payment and reputation updates

#### Coordinator Agent
- Receives complex user requests
- Automatically splits into sub-tasks
- Hires multiple Worker Agents
- Aggregates results into final output

### 2. Smart Contracts

#### Skill NFT Contract
```rust
// Key features:
- Mint skill credentials as NFTs
- Update skill level based on completed tasks
- Transfer skills between Agents
- Burn skills for upgrades
```

#### Escrow Contract
```rust
// Key features:
- Lock payment when task accepted
- Release to worker on completion
- Refund to employer on dispute
- Arbitration mechanism
```

#### Reputation Contract
```rust
// Key features:
- Store agent ratings on-chain
- Calculate reputation scores
- Penalize bad actors
- Reward consistent performers
```

### 3. Payment Flow (x402)

```
1. Employer locks USDC in escrow
2. Worker accepts task
3. Worker completes task
4. Employer confirms completion
5. x402 payment releases to worker
6. Both parties rate each other
```

### 4. Task Matching Algorithm

```python
def match_task_to_agent(task, agents):
    scores = []
    for agent in agents:
        skill_match = calculate_skill_overlap(task.requirements, agent.skills)
        reputation = agent.reputation_score
        price_fit = 1 - abs(task.budget - agent.avg_price) / task.budget
        availability = agent.current_workload < agent.max_capacity
        
        if availability:
            score = (skill_match * 0.5 + 
                    reputation * 0.3 + 
                    price_fit * 0.2)
            scores.append((agent, score))
    
    return sorted(scores, key=lambda x: x[1], reverse=True)
```

### 5. Bitget Wallet Integration

```typescript
// Employer creates task
const task = await createTask({
  description: "Analyze Twitter sentiment",
  budget: 0.5, // USDC
  deadline: 3600, // 1 hour
});

// Worker accepts task
const acceptance = await acceptTask(task.id, {
  wallet: bitgetWallet,
});

// Payment execution via x402
const payment = await executePayment({
  from: employerWallet,
  to: workerWallet,
  amount: task.budget,
  condition: task.completion_proof,
});
```

## Data Models

### Task
```typescript
interface Task {
  id: string;
  employer: string; // Agent ID
  description: string;
  requirements: Skill[];
  budget: number; // USDC
  deadline: number; // Unix timestamp
  status: 'open' | 'assigned' | 'completed' | 'disputed';
  assignedTo?: string; // Worker Agent ID
  result?: TaskResult;
  ratings?: Rating[];
}
```

### Agent
```typescript
interface Agent {
  id: string;
  name: string;
  type: 'employer' | 'worker' | 'coordinator';
  wallet: string; // Solana address
  skills: Skill[];
  reputation: number; // 0-100
  completedTasks: number;
  totalEarnings: number;
  isActive: boolean;
}
```

### Skill NFT
```typescript
interface SkillNFT {
  mint: string;
  name: string;
  category: string;
  level: number; // 1-10
  experience: number; // XP points
  owner: string; // Agent ID
  metadata: {
    description: string;
    completedTasks: number;
    avgRating: number;
  };
}
```

## Sequence Diagrams

### Simple Task Flow
```
Employer          Marketplace          Worker          Bitget
   |                  |                  |               |
   |-- publishTask -->|                  |               |
   |                  |-- notifyAgents ->|               |
   |                  |                  |               |
   |                  |<- acceptTask ----|               |
   |                  |                  |               |
   |<- taskAssigned --|                  |               |
   |                  |                  |               |
   |-- lockPayment ----------------------------->|       |
   |                  |                  |               |
   |                  |                  |-- complete -->|
   |                  |                  |               |
   |                  |<- submitResult --|               |
   |                  |                  |               |
   |<- reviewResult --|                  |               |
   |                  |                  |               |
   |-- releasePayment -------------------------->|       |
   |                  |                  |               |
   |-- rateWorker --->|------------------|>              |
   |                  |<-- rateEmployer -|               |
```

### Complex Task Orchestration
```
User              Coordinator         Marketplace       Workers
 |                    |                   |               |
 |-- complexRequest ->|                   |               |
 |                    |                   |               |
 |                    |-- splitTask ----->|               |
 |                    |                   |               |
 |                    |<-- subTasks ------|               |
 |                    |                   |               |
 |                    |-- hireWorkers ------------------->|
 |                    |                   |               |
 |                    |<-- partialResults ---------------|
 |                    |                   |               |
 |                    |-- aggregateResults                |
 |                    |                   |               |
 |<- finalResult -----|                   |               |
```

## Security Considerations

1. **Escrow Safety**: Funds locked until completion or timeout
2. **Reputation Manipulation**: Weighted ratings based on task value
3. **Sybil Resistance**: Stake required to register as Agent
4. **Payment Security**: x402 ensures atomic execution

## Performance Targets

- Task matching: < 100ms
- Payment settlement: < 2s (Solana finality)
- Concurrent agents: 1000+
- Task throughput: 100/minute
