# github-api-proxy

Proxy for GitHub's API that exposes convenience APIs for tasks that are either
complicated to achieve manually or are very "hot" (i.e. many requests being
made).

## Usage

- `/api/repo/{repo}/latest-tag`

Returns the latest tag (ordered by commit date) of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/latest-tag
{
  "tag": "v3.5.6"
}
```

- `/api/repo/{repo}/latest-release`

Returns the latest release of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/latest-release
{
  "tag": "v3.5.6"
}
```

## Dev

```sh
GITHUB_API_KEY=XXX vercel dev
```
