"""
Sego-Forge — Python reference implementation.
Mirrors the Rust workspace with a minimal Python port for rapid prototyping.
"""
__version__ = "0.1.0"

from .core import WorkflowEngine, Phase, PhaseStatus, Session
from .cli import main as cli_main

__all__ = ["WorkflowEngine", "Phase", "PhaseStatus", "Session", "cli_main", "__version__"]
