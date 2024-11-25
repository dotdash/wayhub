# Wayhub - A GitHub integration for Waybar

Wayhub can be used as a [custom module](https://github.com/Alexays/Waybar/wiki/Module:-Custom) for Waybar.

For now, it can only perform a set of issue/PR counting queries using GraphQL,
so you can get a quick overview for your stuff on GitHub.

![Example of WeyHub in action](example.png)

## Configuration

Running `wayhub` without any configuration file will show an error message that
tells where the configuration file is expected. Usually this will be
`$HOME/.config/wayhub/config.toml`.

The configuration uses [TOML](https://toml.io/en/) and should contain a GitHub
token and some counter definitions.

```toml
github_token = "<your_github_token>"

[[counters]]
label = "<span color=\"green\">Ready</span>"
query = "is:open author:@me review:approved"

[[counters]]
label = "<span color=\"green\">Stale</span>"
query = "is:open is:pr author:@me review:none"
last_updated = "2 weeks ago"

[[counters]]
label = "<span color=\"red\">Changes Requested</span>"
query = "is:open author:@me review:changes_requested"

[[counters]]
label = "<span color=\"lightblue\">Review Requested</span>"
query = "is:pr is:open review-requested:@me"
```

Each counter needs a query using the syntax for [Searching issues and pull
requests](https://docs.github.com/en/search-github/searching-on-github/searching-issues-and-pull-requests)
and a label which can be formatted using [Pango
markup](https://docs.gtk.org/Pango/pango_markup.html).

The optional `last_updated` field accepts a human readable date string, like `2
weeks ago` or `last month` and restricts hits to issues/prs that have have last
updated on of before that date.

Only counters with a number greater than 0 are shown.

## Waybar integration

You can define a custom to update your status module like this:

```json
    "custom/wayhub": {
        "exec": "wayhub",
        "return-type": "json",
        "interval": 60
    }
```
