# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.
"""Cryptographic hash chain for append-only log verification.

Every log entry is hashed with the previous entry's hash,
creating an immutable chain. Any modification to past entries
is detectable.
"""
import hashlib
import json
from typing import List, Dict, Any, Optional


class HashChain:
    """Append-only hash chain for log integrity."""

    def __init__(self, genesis: str = "Ω_RESEARCH_PROGRAM"):
        self.head: str = hashlib.sha256(genesis.encode()).hexdigest()
        self.length: int = 0
        self._entries: List[Dict[str, Any]] = []

    def append(self, data: Dict[str, Any]) -> str:
        """Append data to the chain and return the new head hash.

        Args:
            data: Dictionary to append.

        Returns:
            New head hash after appending.
        """
        entry = {
            "index": self.length,
            "timestamp": self.length,  # placeholder, use real timestamp
            "data_hash": hashlib.sha256(
                json.dumps(data, sort_keys=True).encode()
            ).hexdigest(),
            "prev_hash": self.head,
        }
        entry["entry_hash"] = hashlib.sha256(
            json.dumps(entry, sort_keys=True).encode()
        ).hexdigest()

        self.head = entry["entry_hash"]
        self.length += 1
        self._entries.append(entry)
        return self.head

    def verify(self) -> bool:
        """Verify the integrity of the hash chain.

        Returns:
            True if all hashes match, False otherwise.
        """
        current = self.head
        for i in range(self.length - 1, -1, -1):
            entry = self._entries[i]
            if entry["entry_hash"] != current:
                return False
            if i > 0:
                prev_entry = self._entries[i - 1]
                if entry["prev_hash"] != prev_entry["entry_hash"]:
                    return False
            else:
                genesis_hash = hashlib.sha256("Ω_RESEARCH_PROGRAM".encode()).hexdigest()
                if entry["prev_hash"] != genesis_hash:
                    return False
            current = entry["prev_hash"]
        return True

    def export(self) -> str:
        """Export the full chain as JSON."""
        return json.dumps({
            "genesis": "Ω_RESEARCH_PROGRAM",
            "length": self.length,
            "head": self.head,
            "entries": self._entries,
        }, indent=2)

    @classmethod
    def from_export(cls, json_str: str) -> "HashChain":
        """Reconstruct a hashchain from exported JSON."""
        data = json.loads(json_str)
        chain = cls()
        chain.head = data["head"]
        chain.length = data["length"]
        chain._entries = data["entries"]
        return chain


def hash_file(path: str) -> str:
    """Compute SHA-256 hash of a file."""
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


__all__ = ["HashChain", "hash_file"]
