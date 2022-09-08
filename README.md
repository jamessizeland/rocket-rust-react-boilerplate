# Rust React TypeScript Boilerplate


## Frontend

It is built with [React](https://reactjs.org/) and [TypeScript](https://www.typescriptlang.org/)

It uses [Tailwindcss](https://tailwindcss.com/) for styling, with [DaisyUI](https://daisyui.com/) components and other custom [Elements](./src/components/Elements/)

## Backend

It uses [Rust](https://www.rust-lang.org/) and [Actix](https://actix.rs/)


## Getting Started

[Available Scripts](./docs/scripts.md)

## Project Structure

```project
└── src
    ├── assets
    ├── components
    │   ├── Elements
    │   │   └── index.ts
    │   ├── Layout
    │   │   └── index.ts
    │   └── index.ts
    ├── hooks
    │   └── index.ts
    ├── pages
    │   └── HomePage
    │       ├── components
    │       │   └── HomePageComponent.tsx
    │       └── HomePageContainer.ts
    ├── routes
    │   └── index.ts
    ├── services
    │   └── notifications.ts
    ├── store
    │   ├── reducers
    │   │   ├── userReducer.tsx
    │   │   └── errorReducer.tsx
    │   ├── hooks.ts
    │   └── store.ts
    ├── style
    │   └── global.scss
    ├── test
    │   └── index.ts
    ├── types
    │   └── index.ts
    └── utils
        └── index.ts
```

## Features

**Prettier**: Automatically formats your code. I like to set it up to do this when I stage my files for a commit, so that it's physically impossible for me to commit code that doesn't match my style guide.

**ESLint**: So why would you want a linter too? Because ESLint does more than just style. It picks up when you declare variables you don't use, or reference things that aren't defined, amongst a few other niceties. So while its role diminishes somewhat compared to the days before prettier, it's still useful to have in a project to catch the other errors.

**Typescript**: TypeScript adds additional syntax to JavaScript to support a tighter integration with your editor. Catch errors early in your editor. It understands and compiles to JavaScript, and uses type inference to give you great tooling without additional code.

**Tailwindcss**: A utility-first CSS framework packed with classes like flex, pt-4, text-center and rotate-90 that can be composed to build any design, directly in your markup. Utility classes help you work within the constraints of a system instead of littering your stylesheets with arbitrary values. They make it easy to be consistent with color choices, spacing, typography, shadows, and everything else that makes up a well-engineered design system.
