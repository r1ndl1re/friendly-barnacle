INSERT INTO video (
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
)
SELECT
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
FROM
    video_2021
ON CONFLICT DO NOTHING;
COMMIT;

INSERT INTO video (
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
)
SELECT
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
FROM
    video_2018
ON CONFLICT DO NOTHING;

INSERT INTO video (
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
)
SELECT
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
FROM
    video_2016
ON CONFLICT DO NOTHING;

INSERT INTO video (
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
)
SELECT
    code
,   title
,   description
,   watch_num
,   comment_num
,   mylist_num
,   category
,   length
,   file_type
,   upload_time
,   size_high
,   size_low
FROM
    video_2013
ON CONFLICT DO NOTHING;

INSERT INTO tag (
    name
)
SELECT
    tag_name
FROM
    tag_2021
ON CONFLICT DO NOTHING;

INSERT INTO tag (
    name
)
SELECT
    tag_name
FROM
    tag_2018
ON CONFLICT DO NOTHING;

INSERT INTO tag (
    name
)
SELECT
    tag_name
FROM
    tag_2016
ON CONFLICT DO NOTHING;

INSERT INTO tag (
    name
)
SELECT
    tag_name
FROM
    tag_2013
ON CONFLICT DO NOTHING;


INSERT INTO tag_intedgrated (code, tag_name) SELECT code, tag_name FROM tag_2021 ON CONFLICT DO NOTHING;
INSERT INTO tag_intedgrated (code, tag_name) SELECT code, tag_name FROM tag_2018 ON CONFLICT DO NOTHING;
INSERT INTO tag_intedgrated (code, tag_name) SELECT code, tag_name FROM tag_2016 ON CONFLICT DO NOTHING;
INSERT INTO tag_intedgrated (code, tag_name) SELECT code, tag_name FROM tag_2013 ON CONFLICT DO NOTHING;

INSERT INTO video_tag_relation (
    video_id
,   tag_id
)
SELECT
    video.id
,   tag.id
FROM
    tag_intedgrated
INNER JOIN video ON video.code = tag_intedgrated.code
INNER JOIN tag ON tag.name = tag_intedgrated.tag_name;
