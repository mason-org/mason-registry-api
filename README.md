# mason-registry-api

Public edge-cached API for
[mason.nvim](https://github.com/williamboman/mason.nvim) that exposes
convenience APIs for tasks that are either complicated to achieve on the
client-side or are very "hot" (i.e. requests being made frequently).

# Endpoints

## `/api/repo/{repo}/tags/latest`

Returns the latest tag (ordered by commit date) of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/tags/latest
{
  "tag": "v3.5.6"
}
```

## `/api/repo/{repo}/tags/all`

Returns a list of all available tags of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/tags/all
[
  "v3.5.6",
  "v3.5.5",
  "v3.5.4",
  "v3.5.3",
  "v3.5.2",
  "v3.5.1",
  ...
]
```

## `/api/repo/{repo}/releases/latest`

Returns the latest release of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/releases/latest
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

## `/api/repo/{repo}/releases/all`
Returns a list of all available releases (their tag name) of the provided `{repo}`.

Example:

```
GET /api/repo/sumneko/vscode-lua/releases/all
[
  "v3.5.6",
  "v3.5.5",
  "v3.5.4",
  "v3.5.3",
  "v3.5.2",
  "v3.5.1",
  ...
]
```

## `/api/npm/{package}/versions/latest`

Returns the latest version (`$.["dist-tags"]["latest"]`) of the provided `{package}`.

Example:

```
GET /api/npm/@elm-tooling/elm-language-server/versions/latest
{
  "name": "@elm-tooling/elm-language-server",
  "version": "2.5.2"
}
```

## `/api/npm/{package}/versions/all`

Returns a list of all available version IDs of the provided `{package}`.

Example:

```
GET /api/npm/@elm-tooling/elm-language-server/versions/all
[
  "2.5.2",
  "2.5.0",
  "2.5.0-rc.2",
  "2.5.0-rc.1",
  "2.5.0-alpha.1",
  "2.4.7",
  ...
]
```

## Dev

```sh
GITHUB_API_KEY=XXX vercel dev
```
