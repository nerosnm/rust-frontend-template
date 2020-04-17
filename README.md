# Rust Frontend Template

This is a template for a frontend web application written in Rust, using [Yew](https://yew.rs) and 
bundled with [webpack](https://webpack.js.org).

## Dependencies

- [`rustup`](https://rustup.rs/)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- [`node`](https://nodejs.org/)
- [`yarn`](https://yarnpkg.com/getting-started/install)

## Build

First, run `yarn install`, then:

### Development

To build the app and run it using a development server, run:

```bash
$ yarn start
```

### Production

To build the app in production mode, run:

```bash
$ yarn build
```

The app can then be started in production mode with:

```bash
$ node dist/server.js
```

