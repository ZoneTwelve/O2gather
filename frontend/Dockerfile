FROM node:18.15.0 as builder
WORKDIR /usr/src/app
COPY package.json ./
COPY package-lock.json ./
RUN npm install
COPY . ./
RUN npm run build

FROM nginx:stable-alpine3.17-slim
COPY --from=builder /usr/src/app/public /usr/share/nginx/html