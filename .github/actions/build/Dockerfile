FROM denoland/deno:latest

WORKDIR /app

COPY deno*.json ./

RUN deno cache --unstable --reload

CMD ["deno", "task", "build"]
