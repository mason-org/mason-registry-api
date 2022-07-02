# vercel-github-api-latest-tag-proxy

Proxy for GitHub's API that returns the latest tag (by commit date) of a given repository.
This is information that is only queryable via GitHub's GraphQL API, for which you need to be authenticated to use.
This proxy API allows for looking up the latest tag, with no authentication required.

## Usage

This project exposes a single endpoint `/api/latest-tag` which require a `?repo=` query parameter. If the provided repository doesn't have any tags, or if the proxied request fail for any other reason, a 5xx response will currently be produced.

Example:

```sh
$ curl -s <url>/api/latest-tag?repo=williamboman/vercel-github-api-latest-tag-proxy
{
  "tag": "v1.0.0"
}
```

## Dev

```sh
$ GITHUB_API_KEY=XXX vercel dev
```
