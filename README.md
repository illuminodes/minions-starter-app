# Test App

## Main Deps 

- Rust
- Tailwindcss
- Trunk

## Extra deps

```bash
sudo apt-get update && apt-get install -y \
    libssl3 \
    libssl-dev \
    libgcc-s1 \
    libstdc++6 \
    curl \
```

## Cargo Packages

- Yew (WA framework)
- Nostr-Minions (Nostr and PWA utils)
- Gloo (Logging)
- NostrO2 (Nostr Primitives)

```bash
cargo add yew --features csr hydration
cargo add nostr-minions 
cargo add gloo 
cargo add nostro2
```

## TailwindCSS 

Create `tailwind.config.js` file

```bash
tailwindcss init
```

add content sources to `tailwind.config.js`

```js
module.exports = {
  content: [
    'src/*.{html,rs}',
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

build output file 

```bash
mkdir styles
touch styles/input.css
```

```css 
@tailwind base;
@tailwind components;
@tailwind utilities;
```

build css output 

```bash
tailwindcss -i styles/input.css -o styles/output.css
```

## Trunk 

build index file if not present
```bash
touch index.html
```

serve 
```bash
trunk serve
```
