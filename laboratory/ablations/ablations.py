# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

from typing import Any, Dict


class Ablation:
    def __init__(self, component: str, baseline: Dict[str, Any]) -> None:
        self.component = component
        self.baseline = baseline
        self.variants: Dict[str, Any] = {}

    def register_variant(self, name: str, config: Dict[str, Any]) -> None:
        self.variants[name] = config
