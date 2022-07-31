FROM golang:1.16

ENV PATH="/go/bin:${PATH}"

WORKDIR /go/src

RUN apt-get update && apt-get install sqlite3 -y

RUN go get -u github.com/spf13/cobra@latest && \
    go install github.com/golang/mock/mockgen@v1.5.0 && \
    go install github.com/spf13/cobra-cli@latest

RUN usermod -u 1000 www-data

RUN mkdir -p /var/www/.cache
RUN chown www-data:www-data /go
RUN chown www-data:www-data /go/src
RUN chgrp -R www-data /go
RUN chgrp -R www-data /go/src
RUN chown -R www-data:www-data /var/www/.cache
USER www-data

CMD ["tail", "-f", "/dev/null"]