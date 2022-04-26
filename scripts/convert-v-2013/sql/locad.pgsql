\copy video_2021 (code, title, description, watch_num, comment_num, mylist_num, category, upload_time, length, file_type, size_high, size_low) FROM './nicocomm/data.20211222/video.csv' HEADER CSV;
\copy tag_2021 (code, tag_name) FROM './nicocomm/data.20211222/tag.csv' HEADER CSV;

\copy video_2018 (code, title, description, watch_num, comment_num, mylist_num, category, upload_time, length, file_type, size_high, size_low) FROM './nicocomm/data.20181214/video.csv' HEADER CSV;
\copy tag_2018 (code, tag_name) FROM './nicocomm/data.20181214/tag.csv' HEADER CSV;

\copy video_2016 (code, title, description, watch_num, comment_num, mylist_num, category, upload_time, length, file_type, size_high, size_low) FROM './nicocomm/data.20161216/video.csv' HEADER CSV;
\copy tag_2016 (code, tag_name) FROM './nicocomm/data.20161216/tag.csv' HEADER CSV;

\copy video_2013 (code, title, description, watch_num, comment_num, mylist_num, category, upload_time, length, file_type, size_high, size_low) FROM './nicocomm/data.20130427/video.csv' HEADER CSV;
\copy tag_2013 (code, tag_name) FROM './nicocomm/data.20130427/tag.csv' HEADER CSV;
