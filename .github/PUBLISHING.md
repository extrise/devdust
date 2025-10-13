# Publishing to crates.io

This guide explains how to publish `devdust` to crates.io so users can install it with `cargo install devdust`.

## Prerequisites

1. **crates.io Account**: Create an account at https://crates.io/
2. **API Token**: Generate an API token from https://crates.io/me
3. **GitHub Secret**: Add your crates.io token as a GitHub secret named `CRATES_IO_TOKEN`

## Setup GitHub Secret

1. Go to your GitHub repository
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Name: `CRATES_IO_TOKEN`
5. Value: Your crates.io API token
6. Click **Add secret**

## Publishing Process

### Automatic Publishing (Recommended)

The project uses GitHub Actions to automatically publish to crates.io when you create a version tag:

1. **Update version** in `Cargo.toml` (workspace.package.version)
2. **Commit changes**:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 1.0.0"
   ```
3. **Create and push tag**:
   ```bash
   git tag v1.0.0
   git push origin main
   git push origin v1.0.0
   ```

The GitHub Action will:
- Run all tests
- Build the release binary
- Publish `devdust-core` to crates.io
- Wait for crates.io to update
- Publish `devdust` CLI to crates.io
- Create a GitHub release

### Manual Publishing

If you prefer to publish manually:

1. **Login to crates.io**:
   ```bash
   cargo login
   ```

2. **Publish core library first**:
   ```bash
   cargo publish -p devdust-core
   ```

3. **Wait a moment for crates.io to update** (30 seconds)

4. **Publish CLI**:
   ```bash
   cargo publish -p devdust
   ```

## Installation by Users

Once published, anyone can install devdust with:

```bash
cargo install devdust
```

## Version Management

- Version is managed in the workspace `Cargo.toml`
- Both `devdust-core` and `devdust` CLI share the same version
- Always update the version before creating a new tag
- Follow semantic versioning (MAJOR.MINOR.PATCH)
