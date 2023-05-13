FROM node:18-bullseye-slim as builder
RUN npm install -g pnpm
ARG VITE_WEBSOCKET_PORT
ENV VITE_WEBSOCKET_PORT $VITE_WEBSOCKET_PORT
ARG VITE_SECURE
ENV VITE_SECURE $VITE_SECURE
WORKDIR /builder
COPY package.json pnpm-lock.yaml ./
RUN pnpm i
COPY . .
RUN pnpm run build 

FROM nginx:mainline-alpine
COPY  --from=builder /builder/dist/ /usr/share/nginx/html/