\copy video_2021 (code, title, description, watch_num, comment_num, mylist_num, category, upload_time, length, file_type, size_high, size_low) FROM './video.csv' HEADER CSV;
\copy tag_2021 (code, tag_name) FROM './tag.csv' HEADER CSV;
