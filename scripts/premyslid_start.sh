#!/usr/bin/env bash
# Start a Premyslid content session: make a branch and print one task.
# The PowerShell script of the same name is the reference version; this one exists because
# some agents run under bash. Keep the two in step.
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

if [ ! -f games/premyslid/plan/regions.json ]; then
  echo "Run this from the Suvorov repository: games/premyslid was not found." >&2
  exit 1
fi

current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [ "$current_branch" != "main" ]; then
  echo "Switching from '$current_branch' to main."
  git switch main
fi
git pull --ff-only

# One call only: the task is chosen at random, so asking twice would ask for two tasks.
payload="$(cd python && python -m suvorov.tools.next --json)"

task_id="$(printf '%s' "$payload" | python -c 'import json,sys; task=json.load(sys.stdin)["task"]; print(task["task_id"] if task else "")')"
if [ -z "$task_id" ]; then
  echo "No tasks outstanding. Tell James: the queue is empty, which should not happen."
  exit 0
fi

branch="content/$task_id"
if git branch --list "$branch" | grep -q .; then
  branch="$branch-$(git rev-parse --short HEAD)"
fi
git switch -c "$branch"
printf '%s\n' "$task_id" > .premyslid-session

echo
echo "Branch: $branch"
echo
printf '%s' "$payload" | python -c '
import json
import sys

payload = json.load(sys.stdin)
task = payload["task"]
if payload.get("milestone"):
    print(payload["milestone"], end="\n\n")
print(task["what"], end="\n\n")
print("File:   " + task["file"])
if task.get("schema"):
    print("Schema: " + task["schema"])
if task.get("rules"):
    print("\nRules:")
    for rule in task["rules"]:
        print("  - " + rule)
'
echo
echo 'When the files are written, run:  bash scripts/premyslid_finish.sh --model <your model name>'
