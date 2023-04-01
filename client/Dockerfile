# For now (and probably ever) we won't care about image sizes.
# Yes. It's 1.14GB. This node_modules is big.
FROM node:19
WORKDIR /app
COPY . .

ARG PUBLIC_BASE_URL
ENV PUBLIC_BASE_URL=$PUBLIC_BASE_URL
ARG PUBLIC_SECRET
ENV PUBLIC_SECRET=$PUBLIC_SECRET

RUN npm install
RUN npm run build

EXPOSE 3000


CMD ["node", "-r", "dotenv/config", "build/index.js"]
