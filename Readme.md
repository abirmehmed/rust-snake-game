
# 1. Overwrite the old README with a shorter one
cat << 'EOF' > README.md
# Rust Smooth Snake 🐍

A slithering Snake game built in Rust using `macroquad`. 

## Run it
```bash
cargo run
```

## Controls
- **Arrow Keys**: Move
- **Space**: Restart
EOF

# 2. Push it to GitHub
git add README.md
git commit -m "Simplify README"
git push


