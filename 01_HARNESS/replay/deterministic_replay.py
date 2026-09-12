# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Deterministic replay system for reproducible metric computation.

Ensures that all metrics can be reproduced exactly from raw logs.
Each replay session produces a hash that must match the original.
"""
import hashlib
import json
import os
from typing import List, Dict, Any, Optional
from datetime import datetime


class ReplaySession:
    """Manages a deterministic replay session with hash chaining."""

    def __init__(self, seed_hash: str = "genesis"):
        self.chain: List[str] = [seed_hash]
        self.events: List[Dict[str, Any]] = []

    def replay_log(self, log_path: str) -> List[Dict[str, Any]]:
        """Read and replay a JSONL log file deterministically.

        Args:
            log_path: Path to JSONL log file.

        Returns:
            List of parsed log entries.
        """
        entries: List[Dict[str, Any]] = []
        with open(log_path, "r") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                entry = json.loads(line)
                entries.append(entry)
                self._hash_event(entry)

        return entries

    def _hash_event(self, event: Dict[str, Any]) -> str:
        """Hash an event and append to the chain."""
        event_str = json.dumps(event, sort_keys=True)
        h = hashlib.sha256(
            (self.chain[-1] + event_str).encode()
        ).hexdigest()
        self.chain.append(h)
        self.events.append(event)
        return h

    def verify_chain(self) -> bool:
        """Verify the integrity of the hash chain."""
        for i in range(1, len(self.chain)):
            expected = self.chain[i]
            event = self.events[i - 1]
            event_str = json.dumps(event, sort_keys=True)
            computed = hashlib.sha256(
                (self.chain[i - 1] + event_str).encode()
            ).hexdigest()
            if computed != expected:
                return False
        return True

    def final_hash(self) -> str:
        """Return the final hash of the chain."""
        return self.chain[-1]

    def to_json(self) -> str:
        """Export the replay session as JSON."""
        return json.dumps({
            "chain": self.chain,
            "events": self.events,
            "final_hash": self.final_hash(),
        }, indent=2)


def compute_log_hash(log_path: str) -> str:
    """Compute a deterministic hash of a log file's content."""
    h = hashlib.sha256()
    with open(log_path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


__all__ = ["ReplaySession", "compute_log_hash"]
