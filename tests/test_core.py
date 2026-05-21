"""Tests for forge core workflow engine."""
import pytest

from src.core import WorkflowEngine, PhaseStatus


@pytest.mark.asyncio
async def test_add_and_execute_phase():
    engine = WorkflowEngine("test workflow")
    engine.add_phase("init", "Initialize")
    engine.add_phase("run", "Execute")

    async def _init() -> str:
        return "initialized"

    await engine.execute_phase("init", _init)

    session = engine.finish()
    assert len(session.phases) == 2
    assert session.phases[0].status == PhaseStatus.COMPLETED
    assert session.phases[0].elapsed_ms is not None
    assert session.phases[1].status == PhaseStatus.PENDING


@pytest.mark.asyncio
async def test_phase_not_found():
    engine = WorkflowEngine("test")
    with pytest.raises(KeyError, match="Phase not found"):
        async def _nope() -> str:
            return "nope"
        await engine.execute_phase("nonexistent", _nope)


@pytest.mark.asyncio
async def test_phase_failure():
    engine = WorkflowEngine("test")
    engine.add_phase("fail", "Will fail")

    async def _fail() -> str:
        raise RuntimeError("something broke")

    with pytest.raises(RuntimeError, match="something broke"):
        await engine.execute_phase("fail", _fail)

    # Phase should be marked as failed
    session = engine.finish()
    assert session.phases[0].status == PhaseStatus.FAILED
    assert session.phases[0].message == "something broke"


@pytest.mark.asyncio
async def test_multiple_phases_timing():
    engine = WorkflowEngine("timing test")
    engine.add_phase("a", "Phase A")
    engine.add_phase("b", "Phase B")
    engine.add_phase("c", "Phase C")

    async def _ok() -> str:
        return "done"

    await engine.execute_phase("a", _ok)
    await engine.execute_phase("b", _ok)
    await engine.execute_phase("c", _ok)

    session = engine.finish()
    assert session.completed is True
    assert all(p.status == PhaseStatus.COMPLETED for p in session.phases)
    assert session.duration_ms >= 0


def test_session_defaults():
    engine = WorkflowEngine("default test")
    session = engine.finish()
    assert session.completed is True
    assert session.phases == []
    assert session.duration_ms == 0
