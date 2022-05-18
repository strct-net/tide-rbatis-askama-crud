# tide-rbatis-askama-crud

More information at [template.strct.net](https://template.strct.net)

## Setup

- Copy and modify the .env.example file to .env
- run `npx tailwindcss-cli build -c ./templates/tailwind.config.js -o ./templates/style.css`
- A docker-compose.yml file for a quick PostgreSQL database is provided in the root directory.
- "cargo run/build"

## Building for production

- Modify the `./templates/tailwind.config.js` file to enable purge
- run `npx tailwindcss-cli build -c ./templates/tailwind.config.js -o ./templates/style.css`
- `cargo run build --release`


## Troubleshooting
| Message | Solution |
|---------|----------|
|`couldn't read src\../templates/style.css: The system cannot find the file specified. (os error 2)`|You have to build the style.css file first. Check step 2 of the setup.|

## Need help?

[Join the strct.net Discord server](https://discord.strct.net/) and you might get suspiciously fast replies
