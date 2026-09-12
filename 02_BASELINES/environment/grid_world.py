# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Shared environment: 2D grid world for baseline policies.

Deterministic transitions. 3 objects, 5 actions (N/S/E/W/wait).
Observation = agent (x,y) + object (x,y) positions as flat float array.
"""
import random
from typing import List, Tuple, Dict, Any
from dataclasses import dataclass, field


@dataclass
class GridWorldConfig:
    width: int = 10
    height: int = 10
    n_objects: int = 3
    n_actions: int = 5  # 0=N, 1=S, 2=E, 3=W, 4=wait
    seed: int = 42
    object_positions: List[Tuple[int, int]] = field(default_factory=list)

    def __post_init__(self):
        if not self.object_positions:
            rng = random.Random(self.seed)
            self.object_positions = [
                (rng.randint(0, self.width - 1), rng.randint(0, self.height - 1))
                for _ in range(self.n_objects)
            ]


class GridWorld:
    """Deterministic 2D grid world with objects."""

    def __init__(self, config: GridWorldConfig | None = None):
        self.cfg = config or GridWorldConfig()
        self.agent_x = self.cfg.width // 2
        self.agent_y = self.cfg.height // 2
        self.objects = list(self.cfg.object_positions)
        self.tick = 0

    def reset(self) -> List[float]:
        """Reset the environment to initial state. Returns observation."""
        self.agent_x = self.cfg.width // 2
        self.agent_y = self.cfg.height // 2
        self.tick = 0
        return self.get_observation()

    def get_observation(self) -> List[float]:
        """Return observation: [ax, ay, ox1, oy1, ox2, oy2, ...]."""
        obs = [float(self.agent_x), float(self.agent_y)]
        for ox, oy in self.objects:
            obs.extend([float(ox), float(oy)])
        return obs

    def step(self, action: int) -> Tuple[List[float], bool]:
        """Execute action. Returns (observation, done).

        Actions: 0=N, 1=S, 2=E, 3=W, 4=wait
        """
        dx, dy = 0, 0
        if action == 0:   dy = -1
        elif action == 1: dy = 1
        elif action == 2: dx = 1
        elif action == 3: dx = -1

        new_x = self.agent_x + dx
        new_y = self.agent_y + dy

        if 0 <= new_x < self.cfg.width and 0 <= new_y < self.cfg.height:
            self.agent_x = new_x
            self.agent_y = new_y

        self.tick += 1
        return self.get_observation(), False

    def nearest_object(self) -> int:
        """Return index of nearest object to agent."""
        best_dist = float('inf')
        best_idx = 0
        for i, (ox, oy) in enumerate(self.objects):
            dist = (ox - self.agent_x) ** 2 + (oy - self.agent_y) ** 2
            if dist < best_dist:
                best_dist = dist
                best_idx = i
        return best_idx

    def action_to_nearest(self) -> int:
        """Compute the action that moves toward the nearest object."""
        idx = self.nearest_object()
        ox, oy = self.objects[idx]

        if ox > self.agent_x:
            return 2  # E
        elif ox < self.agent_x:
            return 3  # W
        elif oy > self.agent_y:
            return 1  # S
        elif oy < self.agent_y:
            return 0  # N
        else:
            return 4  # wait (on object)

    def observation_entropy(self, history: List[List[float]]) -> float:
        """Compute entropy of observation distribution."""
        from collections import Counter
        import math

        symbols = [tuple(round(v, 4) for v in obs) for obs in history]
        counts = Counter(symbols)
        total = sum(counts.values())
        if total == 0:
            return 0.0

        entropy = 0.0
        for count in counts.values():
            p = count / total
            entropy -= p * math.log2(p)
        return entropy


def make_log_entry(tick: int, action: int,
                   prediction: List[float]) -> str:
    """Format a log entry as JSONL."""
    import json
    return json.dumps({
        "timestamp": tick,
        "tick": tick,
        "action": action,
        "prediction": prediction,
        "context": {}
    })


__all__ = [
    "GridWorld",
    "GridWorldConfig",
    "make_log_entry",
]
