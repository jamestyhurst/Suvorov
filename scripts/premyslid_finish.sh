#!/usr/bin/env bash
# Finish a Premyslid content session: validate, commit, push, open a pull request.
# The PowerShell script of the same name is the reference version. Keep the two in step.
set -euo pipefail

model=""
open_pull_request=1
while [ $# -gt 0 ]; do
  case "$1" in
    --model) model="$2"; shift 2 ;;
    --no-pull-request) open_pull_request=0; shift ;;
    *) echo "Unknown argument: $1" >&2; exit 1 ;;
  esac
done

if [ -z "$model" ]; then
  echo "Pass --model <your model name>. The trailer it writes is the only record of which model wrote this content." >&2
  exit 1
fi

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repository_root"

task_id="$( [ -f .premyslid-session ] && tr -d '\r\n' < .premyslid-session || echo unknown )"

if ! (cd python && python -m suvorov.tools.validate); then
  echo "The content does not validate. Fix the problems above, then run this script again." >&2
  exit 1
fi

changed="$(git status --porcelain | cut -c4- | tr -d '"' | grep -v '^\.premyslid-session$' || true)"
if [ -z "$changed" ]; then
  echo "Nothing changed. If the task could not be done, write games/premyslid/research/blocked-$task_id.md and run this again." >&2
  exit 1
fi

forbidden="$(printf '%s\n' "$changed" | grep -v '^games/premyslid/content/' | grep -v '^games/premyslid/research/' || true)"
if [ -n "$forbidden" ]; then
  echo "These files are outside the writable folders:" >&2
  printf '  %s\n' $forbidden >&2
  echo "Revert them with 'git restore <file>'. Only content/ and research/ may be changed by a content session." >&2
  exit 1
fi

git add games/premyslid/content games/premyslid/research
git commit -m "content(premyslid): $task_id" -m "Written by an automated content session. The task text came from
python -m suvorov.tools.next and the content validates against
games/premyslid/schema." -m "Task: $task_id
Model: $model"

branch="$(git rev-parse --abbrev-ref HEAD)"
git push -u origin "$branch"

if [ "$open_pull_request" -eq 1 ]; then
  if command -v gh >/dev/null 2>&1; then
    gh pr create --fill --base main --head "$branch"
  else
    echo "The gh CLI is not installed, so no pull request was opened. The branch is pushed."
  fi
fi

rm -f .premyslid-session
echo
echo "Session finished: $task_id by $model."
echo "Stop here. Do not start another task in this session."
