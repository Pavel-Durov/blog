FROM node:19-bullseye
WORKDIR /app
COPY index.ts package.json package-lock.json /app/
RUN npm install
EXPOSE 3000
ENTRYPOINT ["/app/node_modules/.bin/ts-node", "index.ts"]
