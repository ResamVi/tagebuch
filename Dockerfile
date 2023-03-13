# For now (and probably ever) we won't care about image sizes.
# Yes. It's 1.14GB. This node_modules is big.
FROM node:19
WORKDIR /app
COPY . .
RUN npm install
RUN npm run build

EXPOSE 3000

CMD ["node", "build/index.js"]
