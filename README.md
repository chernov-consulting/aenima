# aenima-redirect

Tiny GitHub Pages site whose only purpose is to redirect `https://aenima.io` to `https://aenimalabs.com`.

GitHub Pages only serves content for a domain that's explicitly attached to a Pages repo (via `CNAME` file); to get HTTPS on `aenima.io` and have it redirect, we need a dedicated site. `index.html` does both a `<meta refresh>` and a JavaScript `window.location.replace` so redirection is immediate regardless of client.

The canonical site lives at [chernov-consulting/aenimalabs](https://github.com/chernov-consulting/aenimalabs).
