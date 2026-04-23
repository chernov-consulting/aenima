"""Root invoke Collection. Mirrors relsa/goclaw.io layout.

`uv run inv <namespace>.<task>` resolves via this module because invoke discovers
either a `tasks.py` file or a `tasks/` package at the cwd.
"""

from invoke import Collection

from tasks import daemon, dev, repo

ns = Collection()
ns.add_collection(Collection.from_module(repo), name="repo")
ns.add_collection(Collection.from_module(daemon), name="daemon")
ns.add_collection(Collection.from_module(dev), name="dev")
