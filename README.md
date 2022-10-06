# github-api-proxy

Proxy for GitHub's API that exposes convenience APIs for tasks that are either
complicated to achieve manually or are very "hot" (i.e. many requests being
made).

# Endpoints

## `/api/repo/{repo}/latest-tag`

Returns the latest tag (ordered by commit date) of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/latest-tag
{
  "tag": "v3.5.6"
}
```

## `/api/repo/{repo}/latest-release`

Returns the latest release of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/latest-release
{
  "id": 77366513,
  "tag_name": "v3.5.6",
  "draft": false,
  "prerelease": false,
  "assets": [
    {
      "id": 78050441,
      "url": "https://api.github.com/repos/sumneko/vscode-lua/releases/assets/78050441",
      "name": "vscode-lua-v3.5.6-darwin-arm64.vsix",
      "browser_download_url": "https://github.com/sumneko/vscode-lua/releases/download/v3.5.6/vscode-lua-v3.5.6-darwin-arm64.vsix",
      "created_at": "2022-09-16T07:41:36Z",
      "updated_at": "2022-09-16T07:41:37Z",
      "size": 3805557,
      "download_count": 5967
    },
    ...
  ]
}
```

## Dev

```sh
GITHUB_API_KEY=XXX vercel dev
```
