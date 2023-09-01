Start-Process npx 'tailwindcss -i ./input.css -o ./public/tailwind.css --watch' -NoNewWindow
Start-Process dx 'serve --hot-reload' -NoNewWindow
