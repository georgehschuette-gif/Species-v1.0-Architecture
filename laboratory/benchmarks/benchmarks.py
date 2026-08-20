# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

from typing import List


class Benchmark:
    def __init__(self, name: str, iterations: int = 1000) -> None:
        self.name = name
        self.iterations = iterations
        self.metrics: List[float] = []

    def execute(self) -> float:
        if not self.metrics:
            return 0.0
        return sum(self.metrics) / len(self.metrics)
