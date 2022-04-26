\COPY video TO './nicocomm/video.csv' DELIMITER ',' CSV HEADER;
\COPY tag TO './nicocomm/tag.csv' DELIMITER ',' CSV HEADER;
\COPY video_tag_relation TO './nicocomm/video_tag_relation.csv' DELIMITER ',' CSV HEADER;
