FROM rust:1.62-slim-buster

WORKDIR /rust

RUN apt-get update && apt-get install sqlite3 -y

RUN usermod -u 1000 www-data

# RUN usermod -a -G root www-data
RUN mkdir -p /var/www/.cache
RUN chown -R www-data:www-data /rust
RUN chown -R www-data:www-data /var/www/.cache
USER www-data

CMD ["tail", "-f", "/dev/null"]