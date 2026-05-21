"""Core workflow engine — Python mirror of forge-core."""
from __future__ import annotations

import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Callable, Awaitable


class PhaseStatus(Enum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    SKIPPED = "skipped"


@dataclass
class Phase:
    key: str
    name: str
    status: PhaseStatus = PhaseStatus.PENDING
    message: str | None = None
    elapsed_ms: int | None = None
    detail: str | None = None


@dataclass
class Session:
    id: str
    title: str
    phases: list[Phase] = field(default_factory=list)
    created_at: str = ""
    duration_ms: int = 0
    completed: bool = False


class WorkflowEngine:
    """Executes a sequence of phases in order, collecting timing and status."""

    def __init__(self, title: str) -> None:
        import uuid
        from datetime import datetime, timezone

        self.session = Session(
            id=str(uuid.uuid4()),
            title=title,
            created_at=datetime.now(timezone.utc).isoformat(),
        )
        self._phase_map: dict[str, Phase] = {}

    def add_phase(self, key: str, name: str) -> None:
        phase = Phase(key=key, name=name)
        self.session.phases.append(phase)
        self._phase_map[key] = phase

    async def execute_phase(
        self, key: str, fn: Callable[[], Awaitable[str]]
    ) -> None:
        phase = self._phase_map.get(key)
        if phase is None:
            raise KeyError(f"Phase not found: {key}")

        phase.status = PhaseStatus.RUNNING
        start = time.monotonic()
        try:
            phase.detail = await fn()
            phase.status = PhaseStatus.COMPLETED
            phase.elapsed_ms = int((time.monotonic() - start) * 1000)
        except Exception as exc:
            phase.status = PhaseStatus.FAILED
            phase.message = str(exc)
            raise

    def finish(self) -> Session:
        self.session.completed = True
        self.session.duration_ms = sum(
            p.elapsed_ms for p in self.session.phases if p.elapsed_ms
        )
        return self.session
