FROM node:16

WORKDIR /app
RUN npm install --location=global expo-cli http-server sharp-cli

COPY yarn.lock .
COPY package.json .

RUN yarn install --ignore-enigines

COPY . .

RUN expo build:web

CMD ["http-server", "web-build", "-a", "0.0.0.0", "-p", "80"]
