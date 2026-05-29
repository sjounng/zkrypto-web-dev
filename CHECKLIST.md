# Git Safety Checklist

## Start Of Work

```bash
pwd
git rev-parse --show-toplevel
git status --short --branch
```

## Before Commit

```bash
git status --short
git diff --stat
git add -- <specific-paths>
git status --short
```

## Before Delete

```bash
git clean -nd -- <path>
```

## Weekly Check

```bash
git status --short --branch
git clean -nd
du -sh .git
git count-objects -vH
```

