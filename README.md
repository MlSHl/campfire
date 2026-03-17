# Campfire

This is a personal on-the-go note taking, task and study hour tracking project made for my own use. 


## Creating a project
To recreate this project with the same configuration:

```sh
# recreate this project
bun x sv@0.12.7 create --template minimal --types ts --add tailwindcss="plugins:none" --install bun campfire

```
## Building

To create a production version of your app:

```sh
bun run build
```

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
