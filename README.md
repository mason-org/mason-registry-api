[![Tests](https://img.shields.io/badge/CI-Tests-brightgreen?style=flat-square&logo=github)](https://github.com/mason-org/mason-registry-api/actions/workflows/tests.yaml)
[![API tests](https://img.shields.io/badge/CI-API%20tests-brightgreen?style=flat-square&logo=github)](https://github.com/mason-org/mason-registry-api/actions/workflows/schema-tests.yaml)
[![Sponsors](https://img.shields.io/github/sponsors/williamboman?style=flat-square)](https://github.com/sponsors/williamboman)
 
# mason-registry-api

<img src="https://user-images.githubusercontent.com/6705160/230377905-3e194e97-a1fa-47f6-88c7-20f8286e1fce.png" alt="mason-registry-api logo" />

<p align="center">
    Public, edge-cached, API for <a href="https://github.com/williamboman/mason.nvim"><code>mason.nvim</code></a> that exposes convenience APIs
    for the Mason registry.
</p>

# Endpoints

## `/api/github/{repo}/tags/latest`

Returns the latest tag (ordered by commit date) of the provided `{repo}`.

Example:

```
GET /api/github/sumneko/vscode-lua/tags/latest
{
  "tag": "v3.5.6"
}
```

## `/api/github/{repo}/tags/all`

Returns a list of all available tags of the provided `{repo}`.

Example:

```
GET /api/github/sumneko/vscode-lua/tags/all
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

## `/api/github/{repo}/releases/latest`

Returns the latest release of the provided `{repo}`.

Example:

```
GET /api/github/sumneko/vscode-lua/releases/latest
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

## `/api/github/{repo}/releases/all`
Returns a list of all available releases (their tag name) of the provided `{repo}`.

Example:

```
GET /api/github/sumneko/vscode-lua/releases/all
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
