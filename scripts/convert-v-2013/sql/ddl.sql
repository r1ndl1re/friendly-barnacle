CREATE TABLE IF NOT EXISTS video_2021 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(4000),
    watch_num INTEGER,
    comment_num INTEGER,
    mylist_num INTEGER,
    category VARCHAR(255),
    length INTEGER,
    file_type VARCHAR(3),
    upload_time TIMESTAMP WITH TIME ZONE,
    size_high INTEGER,
    size_low INTEGER,
    CONSTRAINT pk_video_2021 PRIMARY KEY (id),
    CONSTRAINT un1_video_2021 UNIQUE (code)
);

CREATE TABLE IF NOT EXISTS tag_2021 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL,
    CONSTRAINT pk_tag_2021 PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS video_2018 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(4000),
    watch_num INTEGER,
    comment_num INTEGER,
    mylist_num INTEGER,
    category VARCHAR(255),
    length INTEGER,
    file_type VARCHAR(3),
    upload_time TIMESTAMP WITH TIME ZONE,
    size_high INTEGER,
    size_low INTEGER,
    CONSTRAINT pk_video_2018 PRIMARY KEY (id),
    CONSTRAINT un1_video_2018 UNIQUE (code)
);

CREATE TABLE IF NOT EXISTS tag_2018 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL,
    CONSTRAINT pk_tag_2018 PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS video_2016 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(4000),
    watch_num INTEGER,
    comment_num INTEGER,
    mylist_num INTEGER,
    category VARCHAR(255),
    length INTEGER,
    file_type VARCHAR(3),
    upload_time TIMESTAMP WITH TIME ZONE,
    size_high INTEGER,
    size_low INTEGER,
    CONSTRAINT pk_video_2016 PRIMARY KEY (id),
    CONSTRAINT un1_video_2016 UNIQUE (code)
);

CREATE TABLE IF NOT EXISTS tag_2016 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL,
    CONSTRAINT pk_tag_2016 PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS video_2013 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(4000),
    watch_num INTEGER,
    comment_num INTEGER,
    mylist_num INTEGER,
    category VARCHAR(255),
    length INTEGER,
    file_type VARCHAR(3),
    upload_time TIMESTAMP WITH TIME ZONE,
    size_high INTEGER,
    size_low INTEGER,
    CONSTRAINT pk_video_2013 PRIMARY KEY (id),
    CONSTRAINT un1_video_2013 UNIQUE (code)
);

CREATE TABLE IF NOT EXISTS tag_2013 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL,
    CONSTRAINT pk_tag_2013 PRIMARY KEY (id)
);

-- CREATE INDEX i_tag_2021_code ON tag_2021 (code);
