# syntax=docker/dockerfile:1
FROM frand-home-bin:latest

WORKDIR /usr/local/bin

COPY ./config ./config
COPY ./res ./res

VOLUME /usr/local/bin/data

EXPOSE 4430

CMD ["./frand-home-actix"]