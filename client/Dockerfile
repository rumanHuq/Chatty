FROM node:15.6.0-alpine as base
WORKDIR /client
COPY ./package.json ./package*lock ./
RUN apk add --no-cache --virtual .build-deps make gcc g++ python && npm i
ENV PATH=/client/node_modules/.bin:$PATH
ENV ACLOCAL_PATH=/usr/share/aclocal
ENV LIBRARY_PATH=/lib:/usr/lib

FROM base as dev
WORKDIR /client
COPY . .
COPY --from=base /client .
CMD [ "npm", "run", "dev" ]
EXPOSE 5000

