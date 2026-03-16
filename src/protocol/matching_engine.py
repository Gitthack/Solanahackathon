from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass, field
from datetime import datetime, timedelta
import json
from enum import Enum

class TaskStatus(Enum):
    OPEN = "open"
    ASSIGNED = "assigned"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    DISPUTED = "disputed"
    CANCELLED = "cancelled"

class AgentType(Enum):
    EMPLOYER = "employer"
    WORKER = "worker"
    COORDINATOR = "coordinator"

@dataclass
class Skill:
    name: str
    category: str
    level: int = 1  # 1-10
    experience: int = 0
    
    def to_dict(self) -> Dict:
        return {
            "name": self.name,
            "category": self.category,
            "level": self.level,
            "experience": self.experience,
        }

@dataclass
class Task:
    id: str
    employer_id: str
    title: str
    description: str
    requirements: List[Skill]
    budget: float  # USDC
    deadline: datetime
    status: TaskStatus = TaskStatus.OPEN
    assigned_to: Optional[str] = None
    created_at: datetime = field(default_factory=datetime.now)
    result: Optional[str] = None
    
    def to_dict(self) -> Dict:
        return {
            "id": self.id,
            "employer_id": self.employer_id,
            "title": self.title,
            "description": self.description,
            "requirements": [s.to_dict() for s in self.requirements],
            "budget": self.budget,
            "deadline": self.deadline.isoformat(),
            "status": self.status.value,
            "assigned_to": self.assigned_to,
            "created_at": self.created_at.isoformat(),
            "result": self.result,
        }

@dataclass
class Agent:
    id: str
    name: str
    agent_type: AgentType
    wallet_address: str
    skills: List[Skill] = field(default_factory=list)
    reputation_score: float = 50.0  # 0-100
    completed_tasks: int = 0
    total_earnings: float = 0.0
    is_active: bool = True
    created_at: datetime = field(default_factory=datetime.now)
    
    def to_dict(self) -> Dict:
        return {
            "id": self.id,
            "name": self.name,
            "agent_type": self.agent_type.value,
            "wallet_address": self.wallet_address,
            "skills": [s.to_dict() for s in self.skills],
            "reputation_score": self.reputation_score,
            "completed_tasks": self.completed_tasks,
            "total_earnings": self.total_earnings,
            "is_active": self.is_active,
            "created_at": self.created_at.isoformat(),
        }


class TaskMatchingEngine:
    """
    Core matching engine for the Agent marketplace.
    Matches tasks to suitable worker agents based on skills, reputation, and price.
    """
    
    def __init__(self):
        self.tasks: Dict[str, Task] = {}
        self.agents: Dict[str, Agent] = {}
    
    def register_agent(self, agent: Agent) -> None:
        """Register a new agent in the marketplace"""
        self.agents[agent.id] = agent
        print(f"Agent registered: {agent.name} ({agent.id})")
    
    def publish_task(self, task: Task) -> None:
        """Publish a new task to the marketplace"""
        self.tasks[task.id] = task
        print(f"Task published: {task.title} ({task.id})")
        
        # Auto-match immediately
        matches = self.find_matches(task.id)
        if matches:
            print(f"Found {len(matches)} potential matches for task {task.id}")
    
    def calculate_skill_overlap(self, requirements: List[Skill], agent_skills: List[Skill]) -> float:
        """
        Calculate skill overlap between task requirements and agent skills.
        Returns score between 0 and 1.
        """
        if not requirements:
            return 1.0
        
        total_score = 0.0
        for req in requirements:
            # Find matching skill
            matching_skill = next(
                (s for s in agent_skills if s.name.lower() == req.name.lower()),
                None
            )
            
            if matching_skill:
                # Score based on level match
                level_ratio = min(matching_skill.level / req.level, 1.0)
                total_score += level_ratio
            else:
                # Check for category match (partial credit)
                category_match = next(
                    (s for s in agent_skills if s.category.lower() == req.category.lower()),
                    None
                )
                if category_match:
                    total_score += 0.3  # Partial credit for category match
        
        return total_score / len(requirements)
    
    def find_matches(self, task_id: str, top_n: int = 5) -> List[Tuple[Agent, float]]:
        """
        Find the best matching agents for a task.
        Returns list of (agent, score) tuples sorted by score.
        """
        task = self.tasks.get(task_id)
        if not task:
            return []
        
        scores = []
        
        for agent in self.agents.values():
            # Skip inactive agents
            if not agent.is_active:
                continue
            
            # Skip non-worker agents
            if agent.agent_type != AgentType.WORKER:
                continue
            
            # Skip agents with poor reputation
            if agent.reputation_score < 30:
                continue
            
            # Calculate match score
            skill_match = self.calculate_skill_overlap(task.requirements, agent.skills)
            reputation = agent.reputation_score / 100.0
            
            # Price fit (prefer agents who typically charge similar amounts)
            avg_task_value = agent.total_earnings / max(agent.completed_tasks, 1)
            price_fit = 1.0 - min(abs(task.budget - avg_task_value) / task.budget, 1.0)
            
            # Calculate weighted score
            score = (
                skill_match * 0.5 +      # 50% skill match
                reputation * 0.3 +        # 30% reputation
                price_fit * 0.2           # 20% price fit
            )
            
            if score > 0.3:  # Minimum threshold
                scores.append((agent, score))
        
        # Sort by score descending
        scores.sort(key=lambda x: x[1], reverse=True)
        return scores[:top_n]
    
    def assign_task(self, task_id: str, worker_id: str) -> bool:
        """Assign a task to a worker agent"""
        task = self.tasks.get(task_id)
        worker = self.agents.get(worker_id)
        
        if not task or not worker:
            return False
        
        if task.status != TaskStatus.OPEN:
            return False
        
        task.assigned_to = worker_id
        task.status = TaskStatus.ASSIGNED
        
        print(f"Task {task_id} assigned to worker {worker_id}")
        return True
    
    def complete_task(self, task_id: str, result: str) -> bool:
        """Mark a task as completed"""
        task = self.tasks.get(task_id)
        if not task:
            return False
        
        task.status = TaskStatus.COMPLETED
        task.result = result
        
        # Update worker stats
        worker = self.agents.get(task.assigned_to)
        if worker:
            worker.completed_tasks += 1
            worker.total_earnings += task.budget
        
        print(f"Task {task_id} completed by {task.assigned_to}")
        return True
    
    def get_marketplace_stats(self) -> Dict:
        """Get marketplace statistics"""
        open_tasks = sum(1 for t in self.tasks.values() if t.status == TaskStatus.OPEN)
        active_agents = sum(1 for a in self.agents.values() if a.is_active)
        total_volume = sum(t.budget for t in self.tasks.values() if t.status == TaskStatus.COMPLETED)
        
        return {
            "total_tasks": len(self.tasks),
            "open_tasks": open_tasks,
            "completed_tasks": sum(1 for t in self.tasks.values() if t.status == TaskStatus.COMPLETED),
            "active_agents": active_agents,
            "total_volume_usdc": total_volume,
        }


class TaskOrchestrator:
    """
    Advanced task orchestration that can split complex tasks into subtasks
    and hire multiple agents to complete them.
    """
    
    def __init__(self, matching_engine: TaskMatchingEngine):
        self.matching_engine = matching_engine
    
    def split_complex_task(self, description: str) -> List[Task]:
        """
        Split a complex task into subtasks.
        In production, this would use LLM for intelligent splitting.
        """
        # Simple keyword-based splitting for demo
        subtasks = []
        
        # Example: "Analyze Solana DeFi ecosystem"
        if "analyze" in description.lower() and "ecosystem" in description.lower():
            subtasks = [
                Task(
                    id=f"subtask_data_{datetime.now().timestamp()}",
                    employer_id="orchestrator",
                    title="Gather ecosystem data",
                    description="Collect TVL, volume, and user data from Solana DeFi protocols",
                    requirements=[Skill("Data Scraping", "Data", 3)],
                    budget=0.5,
                    deadline=datetime.now() + timedelta(hours=1),
                ),
                Task(
                    id=f"subtask_social_{datetime.now().timestamp()}",
                    employer_id="orchestrator",
                    title="Analyze social sentiment",
                    description="Analyze Twitter and Discord sentiment for Solana projects",
                    requirements=[Skill("Social Analysis", "Analytics", 3)],
                    budget=0.3,
                    deadline=datetime.now() + timedelta(hours=1),
                ),
                Task(
                    id=f"subtask_write_{datetime.now().timestamp()}",
                    employer_id="orchestrator",
                    title="Compile report",
                    description="Compile data and analysis into a comprehensive report",
                    requirements=[Skill("Technical Writing", "Content", 4)],
                    budget=0.5,
                    deadline=datetime.now() + timedelta(hours=2),
                ),
            ]
        
        return subtasks
    
    def execute_complex_task(self, description: str, budget: float) -> Dict:
        """
        Execute a complex task by splitting and orchestrating multiple agents.
        """
        print(f"Orchestrating complex task: {description}")
        
        # Split into subtasks
        subtasks = self.split_complex_task(description)
        
        if not subtasks:
            return {"status": "error", "message": "Could not split task"}
        
        # Hire agents for each subtask
        hired_agents = []
        for subtask in subtasks:
            self.matching_engine.publish_task(subtask)
            matches = self.matching_engine.find_matches(subtask.id, top_n=1)
            
            if matches:
                best_agent, score = matches[0]
                self.matching_engine.assign_task(subtask.id, best_agent.id)
                hired_agents.append({
                    "subtask": subtask.id,
                    "agent": best_agent.id,
                    "score": score,
                })
                print(f"Hired {best_agent.name} for {subtask.title}")
        
        return {
            "status": "orchestrated",
            "total_subtasks": len(subtasks),
            "hired_agents": hired_agents,
            "total_budget": sum(t.budget for t in subtasks),
        }


# Example usage
if __name__ == "__main__":
    # Initialize marketplace
    marketplace = TaskMatchingEngine()
    orchestrator = TaskOrchestrator(marketplace)
    
    # Register some worker agents
    workers = [
        Agent(
            id="worker_1",
            name="DataBot",
            agent_type=AgentType.WORKER,
            wallet_address="7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU",
            skills=[
                Skill("Data Scraping", "Data", 5),
                Skill("Python", "Programming", 4),
            ],
            reputation_score=85.0,
            completed_tasks=12,
        ),
        Agent(
            id="worker_2",
            name="SocialAnalyzer",
            agent_type=AgentType.WORKER,
            wallet_address="8yLXth2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsV",
            skills=[
                Skill("Social Analysis", "Analytics", 4),
                Skill("NLP", "AI", 3),
            ],
            reputation_score=78.0,
            completed_tasks=8,
        ),
        Agent(
            id="worker_3",
            name="WriterBot",
            agent_type=AgentType.WORKER,
            wallet_address="9zMXth2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsW",
            skills=[
                Skill("Technical Writing", "Content", 5),
                Skill("Research", "Analytics", 4),
            ],
            reputation_score=92.0,
            completed_tasks=20,
        ),
    ]
    
    for worker in workers:
        marketplace.register_agent(worker)
    
    # Test complex task orchestration
    result = orchestrator.execute_complex_task(
        description="Analyze Solana DeFi ecosystem",
        budget=2.0
    )
    
    print("\nOrchestration Result:")
    print(json.dumps(result, indent=2))
    
    # Show marketplace stats
    print("\nMarketplace Stats:")
    print(json.dumps(marketplace.get_marketplace_stats(), indent=2))
