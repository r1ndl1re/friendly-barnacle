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
    size_low INTEGER
);

CREATE TABLE IF NOT EXISTS tag_2018 (
    id SERIAL NOT NULL,
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL,
    CONSTRAINT pk_tag_2018 PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS video_2016 (
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
    size_low INTEGER
);

CREATE TABLE IF NOT EXISTS tag_2016 (
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL
);


CREATE TABLE IF NOT EXISTS video_2013 (
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
    size_low INTEGER
);

CREATE TABLE IF NOT EXISTS tag_2013 (
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS tag_intedgrated (
    code VARCHAR(255) NOT NULL,
    tag_name VARCHAR(255) NOT NULL,
    CONSTRAINT un1_tag_inte UNIQUE (code, tag_name)
);

CREATE INDEX i1_tag_2021 ON tag_2021 (code);
CREATE INDEX i2_tag_2021 ON tag_2021 (tag_name);
CREATE INDEX i1_tag_2018 ON tag_2018 (code);
CREATE INDEX i2_tag_2018 ON tag_2018 (tag_name);
CREATE INDEX i1_tag_2016 ON tag_2016 (code);
CREATE INDEX i2_tag_2016 ON tag_2016 (tag_name);
CREATE INDEX i1_tag_2013 ON tag_2013 (code);
CREATE INDEX i2_tag_2013 ON tag_2013 (tag_name);

CREATE INDEX I1_tag_intedgrated ON tag_intedgrated (code);
CREATE INDEX I2_tag_intedgrated ON tag_intedgrated (tag_name);
