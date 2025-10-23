#!/bin/bash

set -e

CURRENT_BRANCH=$(git branch --show-current)

# Update main
git fetch origin
git checkout main
git pull origin main

# Rebase current branch
if [ "$CURRENT_BRANCH" != "main" ]; then
    git checkout "$CURRENT_BRANCH"
    git rebase main
    git push origin "$CURRENT_BRANCH" --force-with-lease
fi

# Rebase all other branches
for branch in $(git branch --format='%(refname:short)' | grep -v '^main$'); do
    if [ "$branch" != "$CURRENT_BRANCH" ]; then
        git checkout "$branch"
        git pull origin "$branch" 2>/dev/null || true
        git rebase main
        git push origin "$branch" --force-with-lease
    fi
done

# Return to original branch
git checkout "$CURRENT_BRANCH"
