# AGENTS.md

이 파일은 Codex가 이 프로젝트에서 작업할 때 따라야 할 규칙입니다.

## Project Scope

- 이 저장소 안의 파일만 수정합니다.
- 다른 프로젝트나 부모 폴더의 파일은 사용자가 명시적으로 요청하지 않으면 수정하지 않습니다.
- 작업 시작 전 `pwd`, `git rev-parse --show-toplevel`, `git status --short --branch`를 확인합니다.

## Commands

```bash
# install

# test

# typecheck

# dev server
```

## Git Safety

- `git add .`를 사용하지 않습니다.
- 필요한 경로만 `git add -- <path>`로 staging합니다.
- 삭제 전에는 `git clean -nd -- <path>`로 dry-run을 먼저 확인합니다.
- `.env`, DB, 캐시, 빌드 결과, 대용량 바이너리를 커밋하지 않습니다.

