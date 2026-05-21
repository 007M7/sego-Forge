"""CLI entry point — Python mirror of forge-cli."""
import argparse
import asyncio

from .core import WorkflowEngine


async def _run(config: str) -> None:
    engine = WorkflowEngine(f"Workflow from {config}")
    engine.add_phase("context", "Build execution context")
    engine.add_phase("exec", "Execute tasks")
    engine.add_phase("persist", "Save results")

    async def _context() -> str:
        return "Context built (mock)"

    async def _exec() -> str:
        return "Tasks executed (mock)"

    async def _persist() -> str:
        return "Results persisted (mock)"

    await engine.execute_phase("context", _context)
    await engine.execute_phase("exec", _exec)
    await engine.execute_phase("persist", _persist)

    session = engine.finish()
    completed = sum(1 for p in session.phases if p.status.value == "completed")
    print(f"✅ Workflow complete: {completed}/{len(session.phases)} phases")


def main() -> None:
    parser = argparse.ArgumentParser(prog="forge", description="Sego-Forge CLI")
    sub = parser.add_subparsers(dest="command")

    sub.add_parser("init", help="Initialize a forge project")
    run_p = sub.add_parser("run", help="Run the workflow")
    run_p.add_argument("-c", "--config", default="forge.toml")
    sub.add_parser("status", help="Show status")

    args = parser.parse_args()
    if args.command == "run":
        asyncio.run(_run(args.config))
    elif args.command == "init":
        print("✅ forge init (Python reference — use Rust CLI for full features)")
    elif args.command == "status":
        print("📋 forge status (Python reference — use Rust CLI for full features)")
    else:
        parser.print_help()
