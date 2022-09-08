

DROP TABLE IF EXISTS "public"."course";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS course_id_seq;

-- Table Definition
CREATE TABLE "public"."course" (
    "id" int4 NOT NULL DEFAULT nextval('course_id_seq'::regclass),
    "teacher_id" int4 NOT NULL,
    "name" varchar(140) NOT NULL,
    "time" timestamp DEFAULT now(),
    "description" varchar(2000),
    "format" varchar(30),
    "structure" varchar(200),
    "duration" varchar(30),
    "price" int4,
    "language" varchar(30),
    "level" varchar(30),
    PRIMARY KEY ("id")
);

DROP TABLE IF EXISTS "public"."teacher";
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS teacher_id_seq;

-- Table Definition
CREATE TABLE "public"."teacher" (
    "id" int4 NOT NULL DEFAULT nextval('teacher_id_seq'::regclass),
    "name" varchar(30),
    "picture_url" varchar(30),
    "profile" varchar(30),
    PRIMARY KEY ("id")
);

