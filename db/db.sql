create table course
(
id serial primary key,
teacher_id INT not null,
name  varchar(140) not null,
time TIMESTAMP default now()
);


insert into course(id,teacher_id,name,time)VALUES(1,1,'First course','2022-09-07 15:49:31');
insert into course(id,teacher_id,name,time)VALUES(2,1,'First course','2022-09-07 15:50:31');