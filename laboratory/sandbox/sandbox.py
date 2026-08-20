# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

from typing import Any, Dict


class Sandbox:
    def __init__(self, environment: str) -> None:
        self.environment = environment
        self.state: Dict[str, Any] = {}

    def isolate(self) -> None:
        self.state["isolated"] = True
