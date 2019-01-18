# Architecture

There are two primary front-end pieces:

1. The main marketing site and home pages on https://xyz.com
2. The JavaScript SPA chat application nested under https://xyz.com/app<sup>1</sup>

The nginx server on the Droplet will serve up the SPA `index.html` file for
any requests that match `/app/*` path, whereas any other location (including `/`)
will be matched up with static files that comprised the marketing site.

> <sup>1</sup> The primary reason that the application is served on a nested path
> rather than a subdomain (eg. https://app.xyz.com) is that this frees us up to
> offer paid, private servers on subdomains (eg. https://some-company.xyz.com)

The application itself will make calls to https://xyz.com/api, which nginx will
proxy directly to the underlying back-end.

## Front-End

### Home

The main home page is a series of static files served directly by nginx.

**Build**

Page templates and stylesheets are built with **middleman**

**Deploy**

Upon building, the static pages are copied directly to the Droplet,
whereas the other assets are deployed to Spaces.

### App

The application uses TypeScript with React and is built by webpack.

**Deploy**

The resultant `index.html` file is deployed to the Droplet to be served
by nginx, whereas the JS bundle is delivered to Spaces.


## Back-End

### API

The API uses Rust and is compiled into a single binary, which can be
deployed directly to the Droplet.

### Database

All data is stored in a Postgres instance on a Droplet.

### Migrations

Database changes affect more than just the API, so they belong to more
than the API (hence the top-level directory).

TODO: Ruby? Rust? What involves least amount of tool/dependency management?
Deploying a server as a binary eliminates the need for Docker, so using that
to manage Ruby installations seems... excessive?
