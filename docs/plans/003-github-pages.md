# GitHub Pages Deployment for MkDocs

## Context

OpenSlicky already has a complete MkDocs Material setup (`mkdocs.yml`) and documentation content in `/docs/`. There's no workflow to deploy the docs to GitHub Pages. We need a workflow that builds and deploys the site automatically.

## Changes

### 1. Create `.github/workflows/docs.yml`

**Triggers:**
- Push to `main` (path-filtered to `docs/**` and `mkdocs.yml`)
- Manual dispatch (`workflow_dispatch`)

**Job: `deploy`** on `ubuntu-latest`:
1. Checkout (with `fetch-depth: 0` for git metadata)
2. Set up Python 3.x
3. `pip install mkdocs-material`
4. `mkdocs gh-deploy --force` (builds site + pushes to `gh-pages` branch)

**Permissions:** `contents: write` (needed to push to `gh-pages` branch)

### 2. Save plan to `docs/plans/003-github-pages.md`

Per project convention.

### Files to create

| File | Action |
|------|--------|
| `.github/workflows/docs.yml` | Create |
| `docs/plans/003-github-pages.md` | Create (copy of this plan) |

### Post-deploy note

After the first push, GitHub Pages must be enabled in repo settings:
**Settings → Pages → Source → "Deploy from a branch" → `gh-pages` / `/ (root)`**

### Verification

- Push to `main` with a docs change → workflow triggers
- `gh-pages` branch is created with built HTML site
- Site is accessible at the GitHub Pages URL
