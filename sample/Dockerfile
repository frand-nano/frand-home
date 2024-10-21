# syntax=docker/dockerfile:1
FROM frand-home-bin:latest

WORKDIR /usr/local/bin

COPY ./.env ./.env
COPY ./config ./config
COPY ./res ./res

CMD ["./frand-home-actix"]