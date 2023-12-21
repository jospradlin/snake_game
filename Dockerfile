# stage 1
FROM node:20.10-alpine3.19 AS base

WORKDIR /base
COPY package.json ./
COPY pkg ./pkg
COPY www ./www
RUN npm run build
RUN npm install

# stage 2

FROM gcr.io/distroless/nodejs20-debian12

WORKDIR /app

COPY server ./server

COPY --from=base /base/pkg ./pkg
COPY --from=base /base/www ./www
COPY --from=base /base/node_modules ./node_modules

EXPOSE 8000

LABEL org.opencontainers.image.source https://github.com/jospradlin/snake_game
CMD ["server/index.js"]