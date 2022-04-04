FROM postgres:13-bullseye

ENV DB_LANG=ja_JP
RUN localedef -i $DB_LANG \
    -c -f UTF-8 -A /usr/share/locale/locale.alias $DB_LANG.UTF-8
ENV LANG $DB_LANG.utf8
ENV TZ="Asia/Tokyo"
