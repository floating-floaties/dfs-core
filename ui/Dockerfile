FROM node:16

WORKDIR /app
RUN npm i --location=global expo-cli serve sharp-cli

COPY yarn.lock .
COPY package.json .

RUN yarn install --ignore-enigines

COPY . .

RUN expo build:web
CMD ["serve", "web-build"]

