FROM rust:1.62-slim-buster

WORKDIR /rust/src

RUN apt-get update && apt-get install sqlite3 -y

RUN usermod -u 1000 www-data

RUN mkdir -p /var/www/.cache
# RUN usermod -a -G root www-data
RUN chown -R www-data:www-data /rust
RUN chown -R www-data:www-data /rust/src
RUN chgrp -R www-data /rust
RUN chgrp -R www-data /rust/src
RUN chown -R www-data:www-data /var/www/.cache
USER www-data

CMD ["tail", "-f", "/dev/null"]